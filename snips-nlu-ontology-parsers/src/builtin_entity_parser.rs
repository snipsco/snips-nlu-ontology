use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};

use conversion::*;
use errors::*;
use failure::ResultExt;
use gazetteer_parser::GazetteerParser;
use itertools::Itertools;
use nlu_ontology::*;
use nlu_utils::string::{convert_to_byte_range, convert_to_char_index};
use rustling_ontology::{build_parser, OutputKind, Parser as RustlingParser, ResolverContext};
use serde_json;
use utils::{get_ranges_mapping, NON_SPACE_REGEX, NON_SPACE_SEPARATED_LANGUAGES};

pub struct BuiltinEntityParser {
    gazetteer_parser: Option<GazetteerParser<BuiltinGazetteerEntityKind>>,
    rustling_parser: RustlingParser,
    language: Language,
    rustling_entity_kinds: Vec<BuiltinEntityKind>,
}

#[derive(Serialize, Deserialize)]
pub struct BuiltinEntityParserLoader {
    language: Language,
    gazetteer_parser_path: Option<PathBuf>,
}

impl BuiltinEntityParserLoader {
    pub fn new(language: Language) -> Self {
        BuiltinEntityParserLoader { language, gazetteer_parser_path: None }
    }

    pub fn use_gazetter_parser<P: AsRef<Path>>(&mut self, parser_path: P) -> &mut Self {
        self.gazetteer_parser_path = Some(parser_path.as_ref().to_path_buf());
        self
    }

    pub fn load(&self) -> Result<BuiltinEntityParser> {
        let supported_entity_kinds = BuiltinEntityKind::supported_entity_kinds(self.language);
        let ordered_entity_kinds = OutputKind::all()
            .iter()
            .map(|output_kind| output_kind.ontology_into())
            .filter(|builtin_entity_kind| supported_entity_kinds.contains(&builtin_entity_kind))
            .collect();
        let rustling_parser = build_parser(self.language.ontology_into())
            .map_err(|_| format_err!("Cannot create Rustling Parser for language {:?}", self.language))?;
        let gazetteer_parser = match &self.gazetteer_parser_path {
            Some(parser_path) => Some(GazetteerParser::from_path(parser_path)?),
            None => None
        };

        Ok(BuiltinEntityParser {
            gazetteer_parser,
            rustling_parser,
            language: self.language,
            rustling_entity_kinds: ordered_entity_kinds,
        })
    }
}

impl BuiltinEntityParser {
    pub fn extract_entities(
        &self,
        sentence: &str,
        filter_entity_kinds: Option<&[BuiltinEntityKind]>,
    ) -> Vec<BuiltinEntity> {
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
    ) -> Vec<BuiltinEntity> {
        let context = ResolverContext::default();
        let rustling_output_kinds = self.rustling_entity_kinds
            .iter()
            .filter(|entity_kind|
                filter_entity_kinds
                    .map(|kinds| kinds.contains(&entity_kind))
                    .unwrap_or(true))
            .flat_map(|kind| kind.try_ontology_into().ok())
            .collect::<Vec<OutputKind>>();

        let rustling_entities = if rustling_output_kinds.is_empty() {
            vec![]
        } else {
            self.rustling_parser
                .parse_with_kind_order(&sentence.to_lowercase(), &context, &rustling_output_kinds)
                .unwrap_or_else(|_| vec![])
                .into_iter()
                .map(|parser_match| rustling::convert_to_builtin(sentence, parser_match))
                .sorted_by(|a, b| Ord::cmp(&a.range.start, &b.range.start))
        };

        let mut gazetteer_entities = match &self.gazetteer_parser {
            Some(gazetteer_parser) => {
                let gazetteer_entity_kinds = filter_entity_kinds
                    .map(|kinds|
                        kinds.into_iter()
                            .flat_map(|kind| kind.try_into_gazetteer_kind().ok())
                            .collect());
                gazetteer_parser
                    .extract_builtin_entities(sentence, gazetteer_entity_kinds.as_ref())
                    .unwrap_or_else(|_| vec![])
            }
            None => vec![]
        };

        let mut entities = rustling_entities;
        entities.append(&mut gazetteer_entities);
        entities
    }

    pub fn _extract_entities_for_non_space_separated(
        &self,
        sentence: &str,
        filter_entity_kinds: Option<&[BuiltinEntityKind]>,
    ) -> Vec<BuiltinEntity> {
        let original_tokens_bytes_ranges: Vec<Range<usize>> = NON_SPACE_REGEX
            .find_iter(sentence)
            .map(|m| m.start()..m.end())
            .collect();

        let joined_sentence = original_tokens_bytes_ranges
            .iter()
            .map(|r| &sentence[r.clone()])
            .join("");

        if original_tokens_bytes_ranges.is_empty() {
            return vec![];
        }

        let ranges_mapping = get_ranges_mapping(&original_tokens_bytes_ranges);

        self._extract_entities(&*joined_sentence, filter_entity_kinds)
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
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BuiltinParserMetadata {
    language: Language,
    gazetteer_parser: Option<String>,
}

impl BuiltinEntityParser {
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::create_dir(path.as_ref())
            .with_context(|_| format!("Cannot create builtin entity parser directory at path: {:?}",
                                      path.as_ref()))?;
        let gazetteer_parser_directory = if let Some(ref gazetteer_parser) = self.gazetteer_parser {
            let gazetteer_parser_path = path.as_ref().join("gazetteer_entity_parser");
            gazetteer_parser.persist(gazetteer_parser_path)?;
            Some("gazetteer_entity_parser".to_string())
        } else {
            None
        };
        let gazetteer_parser_metadata = BuiltinParserMetadata {
            language: self.language,
            gazetteer_parser: gazetteer_parser_directory,
        };
        let metadata_path = path.as_ref().join("metadata.json");
        let metadata_file = fs::File::create(&metadata_path)
            .with_context(|_| format!("Cannot create metadata file at path: {:?}", metadata_path))?;
        serde_json::to_writer_pretty(metadata_file, &gazetteer_parser_metadata)
            .with_context(|_| "Cannot serialize builtin parser metadata")?;
        Ok(())
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let metadata_path = path.as_ref().join("metadata.json");
        let metadata_file = fs::File::open(&metadata_path)
            .with_context(|_| format!("Cannot open builtin parser metadata file at path: {:?}",
                                      metadata_path))?;
        let metadata: BuiltinParserMetadata = serde_json::from_reader(metadata_file)
            .with_context(|_| "Cannot deserialize builtin parser metadata")?;
        let mut parser_loader = BuiltinEntityParserLoader::new(metadata.language);
        if let Some(gazetteer_parser_dir) = metadata.gazetteer_parser {
            let gazetteer_parser_path = path.as_ref().join(&gazetteer_parser_dir);
            parser_loader.use_gazetter_parser(gazetteer_parser_path);
        }
        parser_loader.load()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use nlu_ontology::SlotValue::InstantTime;
    use nlu_ontology::IntoBuiltinEntityKind;
    use nlu_ontology::language::Language;
    use test_utils::test_path;
    use tempfile::tempdir;

    #[test]
    fn test_entities_extraction() {
        let parser = BuiltinEntityParserLoader::new(Language::EN).load().unwrap();
        assert_eq!(
            vec![BuiltinEntityKind::Number, BuiltinEntityKind::Time],
            parser
                .extract_entities("Book me restaurant for two people tomorrow", None)
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            vec![BuiltinEntityKind::Duration],
            parser
                .extract_entities("The weather during two weeks", None)
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            vec![BuiltinEntityKind::Percentage],
            parser
                .extract_entities("Set light to ten percents", None)
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
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );
    }

    #[test]
    fn test_entities_extraction_with_empty_scope() {
        let parser = BuiltinEntityParserLoader::new(Language::EN).load().unwrap();
        let entities = parser.extract_entities("tomorrow morning", Some(&[]));
        assert_eq!(Vec::<BuiltinEntity>::new(), entities);
    }

    #[test]
    fn test_entities_extraction_with_gazetteer_entities() {
        // Given
        let language = Language::FR;
        let parser = BuiltinEntityParserLoader::new(language)
            .use_gazetter_parser(test_path().join("builtin_gazetteer_parser"))
            .load()
            .unwrap();

        // When
        let above_threshold_entity = parser
            .extract_entities("Je voudrais écouter the stones s'il vous plaît", None);
        let below_threshold_entity = parser
            .extract_entities("Je voudrais écouter les stones", None);

        // Then
        let expected_entity = BuiltinEntity {
            value: "the stones".to_string(),
            range: 20..30,
            entity: SlotValue::MusicArtist(StringValue { value: "The Rolling Stones".to_string() }),
            entity_kind: BuiltinEntityKind::MusicArtist,
        };
        assert_eq!(
            vec![expected_entity],
            above_threshold_entity
        );
        assert_eq!(
            Vec::<BuiltinEntity>::new(),
            below_threshold_entity
        );
    }

    #[test]
    fn test_entities_extraction_for_non_space_separated_languages() {
        let parser = BuiltinEntityParserLoader::new(Language::JA).load().unwrap();
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
        );
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
            )
        );
    }

    #[test]
    fn test_entity_examples_should_be_parsed() {
        for language in Language::all() {
            let parser = BuiltinEntityParserLoader::new(*language).load().unwrap();
            for entity_kind in GrammarEntityKind::all() {
                for example in (*entity_kind).examples(*language) {
                    let results = parser.extract_entities(example, Some(&[entity_kind.into_builtin_kind()]));
                    assert_eq!(
                        1, results.len(),
                        "Expected 1 result for entity kind '{:?}' in language '{:?}' for example \
                        {:?}, but found: {:?}", entity_kind, language, example, results);
                    assert_eq!(example.to_string(), results[0].value);
                }
            }
        }
    }

    #[test]
    fn test_should_persist_parser() {
        // Given
        let language = Language::FR;
        let parser = BuiltinEntityParserLoader::new(language)
            .load()
            .unwrap();

        let temp_dir = tempdir().unwrap();
        let parser_dir = temp_dir.path().join("builtin_entity_parser");

        // When
        parser.persist(&parser_dir).unwrap();
        let loaded_parser = BuiltinEntityParser::from_path(&parser_dir).unwrap();

        // Then
        assert_eq!(parser.language, loaded_parser.language);
        assert_eq!(None, loaded_parser.gazetteer_parser);
        assert_eq!(parser.rustling_entity_kinds, loaded_parser.rustling_entity_kinds);
    }

    #[test]
    fn test_should_load_parser_from_path() {
        // Given
        let parser_path = test_path().join("builtin_entity_parser_no_gazetteer");

        // When
        let parser = BuiltinEntityParser::from_path(parser_path).unwrap();

        // Then
        let expected_parser = BuiltinEntityParserLoader::new(Language::EN)
            .load()
            .unwrap();
        assert_eq!(expected_parser.language, parser.language);
        assert_eq!(expected_parser.gazetteer_parser, parser.gazetteer_parser);
        assert_eq!(expected_parser.rustling_entity_kinds, parser.rustling_entity_kinds);
    }

    #[test]
    fn test_should_persist_parser_with_gazetteer_entities() {
        // Given
        let language = Language::FR;
        let parser = BuiltinEntityParserLoader::new(language)
            .use_gazetter_parser(test_path().join("builtin_gazetteer_parser"))
            .load()
            .unwrap();

        let temp_dir = tempdir().unwrap();
        let parser_dir = temp_dir.path().join("builtin_entity_parser");

        // When
        parser.persist(&parser_dir).unwrap();
        let loaded_parser = BuiltinEntityParser::from_path(&parser_dir).unwrap();

        // Then
        assert_eq!(parser.language, loaded_parser.language);
        assert_eq!(parser.gazetteer_parser, loaded_parser.gazetteer_parser);
        assert_eq!(parser.rustling_entity_kinds, loaded_parser.rustling_entity_kinds);
    }

    #[test]
    fn test_should_load_parser_with_gazetteer_entities_from_path() {
        // Given
        let parser_path = test_path().join("builtin_entity_parser");

        // When
        let parser = BuiltinEntityParser::from_path(parser_path).unwrap();

        // Then
        let expected_parser = BuiltinEntityParserLoader::new(Language::FR)
            .use_gazetter_parser(test_path().join("builtin_gazetteer_parser"))
            .load()
            .unwrap();
        assert_eq!(expected_parser.language, parser.language);
        assert_eq!(expected_parser.gazetteer_parser, parser.gazetteer_parser);
        assert_eq!(expected_parser.rustling_entity_kinds, parser.rustling_entity_kinds);
    }
}
