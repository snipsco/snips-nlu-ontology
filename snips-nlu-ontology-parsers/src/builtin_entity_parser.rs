use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::Range;

use itertools::Itertools;
use regex::Regex;

use errors::*;
use gazetteer_entity_parser::{Gazetteer, Parser as _GazetteerParser};
use conversion::*;
use nlu_ontology::*;
use nlu_utils::string::{convert_to_byte_range, convert_to_char_index};
use rustling_ontology::{build_parser, OutputKind, Parser as RustlingParser, ResolverContext};

lazy_static! {
    static ref NON_SPACE_REGEX: Regex = Regex::new(r"[^\s]+").unwrap();
}

lazy_static! {
    static ref NON_SPACE_SEPARATED_LANGUAGES: HashSet<Language> = hashset!(Language::JA);
}

pub struct BuiltinEntityParser {
    gazetteer_parsers: Vec<GazetteerParser>,
    rustling_parser: RustlingParser,
    language: Language,
    rustling_entity_kinds: Vec<BuiltinEntityKind>,
}

#[derive(Serialize, Deserialize)]
pub struct BuiltinEntityParserConfiguration {
    pub language: Language,
    pub builtin_entities_resources: Vec<BuiltinEntityResource>,
}

#[derive(Serialize, Deserialize)]
pub struct BuiltinEntityResource {
    pub builtin_entity_name: String,
    pub resource_path: String,
}

struct GazetteerParser {
    parser: _GazetteerParser,
    entity_kind: GazetteerEntityKind,
}

impl BuiltinEntityParser {
    pub fn new(config: BuiltinEntityParserConfiguration) -> Result<Self> {
        let supported_entity_kinds = BuiltinEntityKind::supported_entity_kinds(config.language);
        let ordered_entity_kinds = OutputKind::all()
            .iter()
            .map(|output_kind| output_kind.ontology_into())
            .filter(|builtin_entity_kind| supported_entity_kinds.contains(&builtin_entity_kind))
            .collect();
        let rustling_parser = build_parser(config.language.ontology_into())
            .map_err(|_| format_err!("Cannot create Rustling Parser for language {:?}", config.language))?;
        let gazetteer_parsers = config.builtin_entities_resources
            .iter()
            .map(|resources| {
                let entity_kind = GazetteerEntityKind::from_identifier(&resources.builtin_entity_name)?;
                if !supported_entity_kinds.contains(&(entity_kind.into())) {
                    return Err(
                        format_err!("Gazetteer entity kind {:?} is not yet supported in language {:?}",
                                    entity_kind, config.language));
                }
                let gazetteer = Gazetteer::from_json(&resources.resource_path, None)?;
                let parser = _GazetteerParser::from_gazetteer(&gazetteer, 1.0)?;
                Ok(GazetteerParser { parser, entity_kind })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(BuiltinEntityParser {
            gazetteer_parsers,
            rustling_parser,
            language: config.language,
            rustling_entity_kinds: ordered_entity_kinds,
        })
    }

    pub fn from_language(language: Language) -> Result<Self> {
        Self::new(BuiltinEntityParserConfiguration {
            language,
            builtin_entities_resources: vec![],
        })
    }

    pub fn extract_entities(
        &self,
        sentence: &str,
        filter_entity_kinds: Option<&[BuiltinEntityKind]>,
    ) -> Result<Vec<BuiltinEntity>> {
        if NON_SPACE_SEPARATED_LANGUAGES.contains(&self.language) {
            self._extract_entities_for_non_space_separated(sentence, filter_entity_kinds)
        } else {
            self._extract_entities(sentence, filter_entity_kinds)
        }
    }

    fn _extract_entities(
        &self,
        sentence: &str,
        filter_entity_kinds: Option<&[BuiltinEntityKind]>,
    ) -> Result<Vec<BuiltinEntity>> {
        let context = ResolverContext::default();
        let rustling_output_kinds = self.rustling_entity_kinds
            .iter()
            .filter(|entity_kind|
                filter_entity_kinds
                    .map(|kinds| kinds.contains(&entity_kind))
                    .unwrap_or(true))
            .flat_map(|kind| kind.try_ontology_into().ok())
            .collect::<Vec<OutputKind>>();

        let rustling_entities = self.rustling_parser
            .parse_with_kind_order(&sentence.to_lowercase(), &context, &rustling_output_kinds)
            .map_err(|_| format_err!("Error when parsing sentence {} with Rustling", sentence))?
            .into_iter()
            .map(|parser_match| rustling::convert_to_builtin(sentence, parser_match))
            .sorted_by(|a, b| Ord::cmp(&a.range.start, &b.range.start));

        let mut gazetteer_entities: Vec<BuiltinEntity> = self.gazetteer_parsers.iter()
            .filter(|parser|
                filter_entity_kinds
                    .map(|kinds| kinds.contains(&parser.entity_kind.into()))
                    .unwrap_or(true))
            .map(|parser| {
                Ok(parser.parser
                    .run(&sentence.to_lowercase())?
                    .into_iter()
                    .map(|parsed_value|
                        gazetteer_entities::convert_to_builtin(parsed_value, parser.entity_kind))
                    .collect())
            })
            .collect::<Result<Vec<Vec<BuiltinEntity>>>>()?
            .into_iter()
            .flat_map(|entities| entities)
            .collect();

        let mut entities = rustling_entities;
        entities.append(&mut gazetteer_entities);
        Ok(entities)
    }

    pub fn _extract_entities_for_non_space_separated(
        &self,
        sentence: &str,
        filter_entity_kinds: Option<&[BuiltinEntityKind]>,
    ) -> Result<Vec<BuiltinEntity>> {
        let original_tokens_bytes_ranges: Vec<Range<usize>> = NON_SPACE_REGEX
            .find_iter(sentence)
            .map(|m| m.start()..m.end())
            .collect();

        let joined_sentence = original_tokens_bytes_ranges
            .iter()
            .map(|r| &sentence[r.clone()])
            .join("");

        if original_tokens_bytes_ranges.is_empty() {
            return Ok(vec![]);
        }

        let ranges_mapping = get_ranges_mapping(&original_tokens_bytes_ranges);

        Ok(self._extract_entities(&*joined_sentence, filter_entity_kinds)?
            .into_iter()
            .filter_map(|ent| {
                let byte_range = convert_to_byte_range(&*joined_sentence, &ent.range);
                let start = byte_range.start;
                let end = byte_range.end;
                // Check if match range correspond to original tokens otherwise skip the entity
                if (start == 0 as usize || ranges_mapping.contains_key(&start))
                    && (ranges_mapping.contains_key(&end))
                    {
                        let start_token_index = if start == 0 as usize {
                            0 as usize
                        } else {
                            ranges_mapping[&start] + 1
                        };
                        let end_token_index = ranges_mapping[&end];

                        let original_start = original_tokens_bytes_ranges[start_token_index].start;
                        let original_end = original_tokens_bytes_ranges[end_token_index].end;
                        let value = sentence[original_start..original_end].to_string();

                        let original_ent = BuiltinEntity {
                            value,
                            range: convert_to_char_index(&sentence, original_start)
                                ..convert_to_char_index(&sentence, original_end),
                            entity: ent.entity,
                            entity_kind: ent.entity_kind,
                        };
                        Some(original_ent)
                    } else {
                    None
                }
            })
            .collect())
    }
}

fn get_ranges_mapping(tokens_ranges: &Vec<Range<usize>>) -> HashMap<usize, usize> {
    /* Given tokens ranges returns a mapping of byte index to a token index
    The byte indexes corresponds to indexes of the end of tokens in string given by joining all
    the tokens. The tokens index gives the index of the tokens preceding the byte index.

    For instance, if range_mapping[65] -> 5, this means that the token of index 6 starts at the
    65th byte in the joined string
    */
    let ranges_mapping =
        HashMap::<usize, usize>::from_iter(tokens_ranges.iter().enumerate().fold(
            vec![],
            |mut acc: Vec<(usize, usize)>, (token_index, ref original_range)| {
                let previous_end = if token_index == 0 {
                    0 as usize
                } else {
                    acc[acc.len() - 1].0
                };
                acc.push((
                    previous_end + original_range.end - original_range.start,
                    token_index,
                ));
                acc
            },
        ));
    ranges_mapping
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    use nlu_ontology::SlotValue::InstantTime;
    use nlu_ontology::language::Language;

    #[test]
    fn test_entities_extraction() {
        let parser = BuiltinEntityParser::from_language(Language::EN).unwrap();
        assert_eq!(
            vec![BuiltinEntityKind::Number, BuiltinEntityKind::Time],
            parser
                .extract_entities("Book me restaurant for two people tomorrow", None)
                .unwrap()
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            vec![BuiltinEntityKind::Duration],
            parser
                .extract_entities("The weather during two weeks", None)
                .unwrap()
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            vec![BuiltinEntityKind::Percentage],
            parser
                .extract_entities("Set light to ten percents", None)
                .unwrap()
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            vec![BuiltinEntityKind::AmountOfMoney],
            parser
                .extract_entities(
                    "I would like to do a bank transfer of ten euros for my friends",
                    None,
                )
                .unwrap()
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );
    }

    #[test]
    fn test_entities_extraction_with_empty_scope() {
        let parser = BuiltinEntityParser::from_language(Language::EN).unwrap();
        let entities = parser.extract_entities("tomorrow morning", Some(&[])).unwrap();
        assert_eq!(Vec::<BuiltinEntity>::new(), entities);
    }

    #[test]
    fn test_entities_extraction_for_non_space_separated_languages() {
        let parser = BuiltinEntityParser::from_language(Language::JA).unwrap();
        let expected_time_value = InstantTimeValue {
            value: "2013-02-10 00:00:00 +01:00".to_string(),
            grain: Grain::Day,
            precision: Precision::Exact,
        };

        let expected_entity = BuiltinEntity {
            value: "二 千 十三 年二 月十 日".to_string(),
            range: 10..24,
            entity_kind: BuiltinEntityKind::Time,
            entity: InstantTime(expected_time_value.clone()),
        };

        let parsed_entities = parser.extract_entities(
            " の カリフォル  二 千 十三 年二 月十 日  ニア州の天気予報は？",
            None,
        ).unwrap();
        assert_eq!(1, parsed_entities.len());
        let parsed_entity = &parsed_entities[0];
        assert_eq!(expected_entity.value, parsed_entity.value);
        assert_eq!(expected_entity.range, parsed_entity.range);
        assert_eq!(expected_entity.entity_kind, parsed_entity.entity_kind);

        if let SlotValue::InstantTime(ref parsed_time) = parsed_entity.entity {
            assert_eq!(expected_time_value.grain, parsed_time.grain);
            assert_eq!(expected_time_value.precision, parsed_time.precision);
        } else {
            panic!("")
        }

        assert_eq!(
            Vec::<BuiltinEntity>::new(),
            parser.extract_entities(
                "二 千 十三 年二 月十 日の カリフォルニア州の天気予報は？",
                None,
            ).unwrap()
        );
    }

    #[test]
    fn test_entity_examples_should_be_parsed() {
        for language in Language::all() {
            let parser = BuiltinEntityParser::from_language(*language).unwrap();
            for entity_kind in BuiltinEntityKind::all() {
                for example in entity_kind.examples(*language) {
                    let results = parser.extract_entities(example, Some(&[*entity_kind])).unwrap();
                    assert_eq!(
                        1, results.len(),
                        "Expected 1 result for entity kind '{:?}' in language '{:?}' for example \
                        {:?}, but found: {:?}", entity_kind, language, example, results);
                    assert_eq!(example.to_string(), results[0].value);
                }
            }
        }
    }
}
