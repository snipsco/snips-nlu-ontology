use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::Range;
use std::time::Instant;
use std::sync::{Arc, Mutex};

use itertools::Itertools;
use regex::Regex;

use rustling_converters::{FromRustling, IntoBuiltin};
use nlu_ontology::*;
use rustling_ontology::{build_parser, OutputKind, Parser, ResolverContext};

pub struct BuiltinEntityParser {
    parser: Parser,
    lang: Language,

}

lazy_static! {
    static ref NON_SPACE_REGEX: Regex = Regex::new(r"[^\s]+").unwrap();
}

lazy_static! {
    static ref NON_SPACE_SEPARATED_LANGUAGES: HashSet<Language> = hashset!(Language::JA);
}

impl BuiltinEntityParser {
    pub fn get(lang: Language) -> Arc<BuiltinEntityParser> {
        lazy_static! {
            static ref CACHED_PARSERS: Mutex<HashMap<String, Arc<BuiltinEntityParser>>> =
                Mutex::new(HashMap::new());
        }

        CACHED_PARSERS
            .lock()
            .unwrap()
            .entry(lang.to_string())
            .or_insert_with(|| {
                Arc::new(BuiltinEntityParser {
                    parser: build_parser(lang.into_builtin()).unwrap(),
                    lang,
                })
            })
            .clone()
    }

    pub fn extract_entities(
        &self,
        sentence: &str,
        filter_entity_kinds: Option<&[BuiltinEntityKind]>,
    ) -> Vec<BuiltinEntity> {
        if NON_SPACE_SEPARATED_LANGUAGES.contains(&self.lang) {
            let original_tokens_bytes_ranges: Vec<Range<usize>> = NON_SPACE_REGEX
                .find_iter(sentence)
                .map(|m| Range { start: m.start(), end: m.end() })
                .collect();

            let joined_sentence = original_tokens_bytes_ranges
                .iter()
                .map(|r| &sentence[r.start..r.end])
                .join("");

            if original_tokens_bytes_ranges.len() == 0 {
                return vec![];
            }

            let joined_sentence_match_end_byte_index_to_token_index = HashMap::<usize, usize>::from_iter(
                original_tokens_bytes_ranges
                    .iter()
                    .enumerate()
                    .fold(vec![], |mut acc: Vec<(usize, usize)>, (i, ref r)| {
                        let previous_end = if i == 0 {
                            0 as usize
                        } else {
                            acc[acc.len() - 1].0
                        };
                        acc.push((previous_end + r.end - r.start, i));
                        acc
                    })
            );

            let entities = self._extract_entities(&*joined_sentence, filter_entity_kinds);
            entities
                .into_iter()
                .filter_map(|ent| {
                    let byte_range = convert_to_byte_range(&*joined_sentence, &ent.range);
                    let start = byte_range.start;
                    let end = byte_range.end;
                    // Check if match range correspond to original tokens otherwise skip the entity
                    if (start == 0 as usize || joined_sentence_match_end_byte_index_to_token_index.contains_key(&start))
                        && (joined_sentence_match_end_byte_index_to_token_index.contains_key(&end)) {
                        let start_token_index = if start == 0 as usize {
                            0 as usize
                        } else {
                            joined_sentence_match_end_byte_index_to_token_index[&start]
                        };
                        let end_token_index = joined_sentence_match_end_byte_index_to_token_index[&end];

                        let original_start = original_tokens_bytes_ranges[start_token_index].start;
                        let original_end = original_tokens_bytes_ranges[end_token_index].end;
                        let value = sentence[original_start..original_end].to_string();

                        let original_ent = BuiltinEntity {
                            value,
                            range: Range { start: original_start, end: original_end },
                            entity: ent.entity,
                            entity_kind: ent.entity_kind,
                        };
                        Some(original_ent)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            self._extract_entities(sentence, filter_entity_kinds)
        }
    }

    fn _extract_entities(&self,
                         sentence: &str,
                         filter_entity_kinds: Option<&[BuiltinEntityKind]>,
    ) -> Vec<BuiltinEntity> {
        lazy_static! {
            static ref CACHED_ENTITY: Mutex<EntityCache> = Mutex::new(EntityCache::new(60));
        }
        let key = CacheKey {
            lang: self.lang.to_string(),
            input: sentence.into(),
            kinds: filter_entity_kinds.map(|kinds| kinds.to_vec()),
        };
        CACHED_ENTITY
            .lock()
            .unwrap()
            .cache(&key, |key| {
                let context = ResolverContext::default();
                if let Some(kinds) = key.kinds.as_ref() {
                    let kind_order = kinds
                        .iter()
                        .map(|kind| kind.into_builtin())
                        .collect::<Vec<OutputKind>>();
                    self.parser
                        .parse_with_kind_order(&sentence.to_lowercase(), &context, &kind_order)
                        .unwrap_or(Vec::new())
                        .iter()
                        .filter_map(|m| {
                            let entity_kind = BuiltinEntityKind::from_rustling(&m.value);
                            kinds.iter().find(|kind| **kind == entity_kind).map(|kind| {
                                BuiltinEntity {
                                    value: sentence[m.byte_range.0..m.byte_range.1].into(),
                                    range: m.char_range.0..m.char_range.1,
                                    entity: m.value.clone().into_builtin(),
                                    entity_kind: kind.clone(),
                                }
                            })
                        })
                        .sorted_by(|a, b| Ord::cmp(&a.range.start, &b.range.start))
                } else {
                    self.parser
                        .parse(&sentence.to_lowercase(), &context)
                        .unwrap_or(Vec::new())
                        .iter()
                        .map(|entity| BuiltinEntity {
                            value: sentence[entity.byte_range.0..entity.byte_range.1].into(),
                            range: entity.char_range.0..entity.char_range.1,
                            entity: entity.value.clone().into_builtin(),
                            entity_kind: BuiltinEntityKind::from_rustling(&entity.value),
                        })
                        .sorted_by(|a, b| Ord::cmp(&a.range.start, &b.range.start))
                }
            })
            .entities
    }
}

struct EntityCache {
    container: HashMap<CacheKey, CacheValue>,
    valid_duration_sec: u64,
}

impl EntityCache {
    fn new(valid_duration_sec: u64) -> EntityCache {
        EntityCache {
            container: HashMap::new(),
            valid_duration_sec,
        }
    }

    fn cache<F: Fn(&CacheKey) -> Vec<BuiltinEntity>>(
        &mut self,
        key: &CacheKey,
        producer: F,
    ) -> CacheValue {
        let cached_value = self.container.get(key).map(|a| a.clone());
        if let Some(value) = cached_value {
            if value.is_valid(self.valid_duration_sec) {
                return value;
            }
        }
        let value = CacheValue::new(producer(key));
        self.container.insert(key.clone(), value.clone());
        value
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    lang: String,
    input: String,
    kinds: Option<Vec<BuiltinEntityKind>>,
}

#[derive(Debug, Clone)]
struct CacheValue {
    entities: Vec<BuiltinEntity>,
    instant: Instant,
}

impl CacheValue {
    fn new(entities: Vec<BuiltinEntity>) -> CacheValue {
        CacheValue {
            entities,
            instant: Instant::now(),
        }
    }

    fn is_valid(&self, valid_duration_sec: u64) -> bool {
        self.instant.elapsed().as_secs() < valid_duration_sec
    }
}


fn convert_to_byte_range(string: &str, range: &Range<usize>) -> Range<usize> {
    Range {
        start: convert_to_byte_index(string, range.start),
        end: convert_to_byte_index(string, range.end),
    }
}

fn convert_to_byte_index(string: &str, char_index: usize) -> usize {
    let mut result = 0;
    for (current_char_index, char) in string.chars().enumerate() {
        if current_char_index == char_index {
            return result;
        }
        result += char.len_utf8()
    }
    result
}


#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_entities_extraction() {
        let parser = BuiltinEntityParser::get(Language::EN);
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
    fn test_entities_extraction_for_non_space_separated_languages() {
        let parser = BuiltinEntityParser::get(Language::JA);
        assert_eq!(
            vec![BuiltinEntityKind::Time],
            parser
                .extract_entities(" 明 日 の カリフォルニア州の天気予報は？", None)
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );

        assert_eq!(
            Vec::<BuiltinEntityKind>::new(),
            parser
                .extract_entities(" 明 日の カリフォルニア州の天気予報は？", None)
                .iter()
                .map(|e| e.entity_kind)
                .collect_vec()
        );
    }

    #[test]
    fn test_entity_cache() {
        fn parse(_: &CacheKey) -> Vec<BuiltinEntity> {
            vec![
                BuiltinEntity {
                    value: "two".into(),
                    range: 23..26,
                    entity_kind: BuiltinEntityKind::Number,
                    entity: SlotValue::Number(NumberValue { value: 2.0 }),
                },
                BuiltinEntity {
                    value: "4.5".into(),
                    range: 34..42,
                    entity_kind: BuiltinEntityKind::Number,
                    entity: SlotValue::Number(NumberValue { value: 4.5 }),
                },
            ]
        }

        let key = CacheKey {
            lang: "en".into(),
            input: "test".into(),
            kinds: None,
        };

        let mut cache = EntityCache::new(10); // caching for 10s
        assert_eq!(
            cache.cache(&key, parse).instant,
            cache.cache(&key, parse).instant
        );

        let mut cache = EntityCache::new(0); // no caching
        assert_ne!(
            cache.cache(&key, parse).instant,
            cache.cache(&key, parse).instant
        );
    }
}
