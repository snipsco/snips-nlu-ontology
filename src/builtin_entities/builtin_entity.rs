use std::ops::Range;

use serde::Deserialize;
use serde_json;

use errors::*;
use language::Language;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BuiltinEntity {
    pub value: String,
    pub range: Range<usize>,
    pub entity: ::SlotValue,
    #[serde(serialize_with = "serialize_builtin_entity_kind", deserialize_with = "deserialize_builtin_entity_kind")]
    pub entity_kind: BuiltinEntityKind,
}

fn serialize_builtin_entity_kind<S>(value: &BuiltinEntityKind, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where S: ::serde::Serializer {
    serializer.serialize_str(value.identifier())
}

fn deserialize_builtin_entity_kind<'de, D>(deserializer: D) -> ::std::result::Result<BuiltinEntityKind, D::Error>
    where D: ::serde::Deserializer<'de> {
    String::deserialize(deserializer)
        .and_then(|s| BuiltinEntityKind::from_identifier(&s).map_err(::serde::de::Error::custom))
}

enum_kind!(
    BuiltinEntityKind,
    [
        AmountOfMoney,
        Duration,
        Number,
        Ordinal,
        Temperature,
        Time,
        Percentage
    ]
);

impl BuiltinEntityKind {
    pub fn identifier(&self) -> &str {
        match *self {
            BuiltinEntityKind::AmountOfMoney => "snips/amountOfMoney",
            BuiltinEntityKind::Duration => "snips/duration",
            BuiltinEntityKind::Number => "snips/number",
            BuiltinEntityKind::Ordinal => "snips/ordinal",
            BuiltinEntityKind::Temperature => "snips/temperature",
            BuiltinEntityKind::Time => "snips/datetime",
            BuiltinEntityKind::Percentage => "snips/percentage",
        }
    }

    pub fn from_identifier(identifier: &str) -> Result<Self> {
        BuiltinEntityKind::all()
            .iter()
            .find(|kind| kind.identifier() == identifier)
            .map(|k| k.clone())
            .ok_or(format!("Unknown EntityKind identifier: {}", identifier).into())
    }
}

impl BuiltinEntityKind {
    pub fn description(&self) -> &str {
        match *self {
            BuiltinEntityKind::AmountOfMoney => "Matches amount of money",
            BuiltinEntityKind::Duration => "Matches time duration",
            BuiltinEntityKind::Number => "Matches a cardinal numbers",
            BuiltinEntityKind::Ordinal => "Matches a ordinal numbers",
            BuiltinEntityKind::Temperature => "Matches a temperature",
            BuiltinEntityKind::Time => "Matches date, time, intervals or date and time together",
            BuiltinEntityKind::Percentage => "Matches a percentage",
        }
    }
}

impl BuiltinEntityKind {
    pub fn examples(&self) -> &[&str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &["ten dollars and five cents", "around 5€"],
            BuiltinEntityKind::Duration => &["3 month", "4 seconds", "8 years"],
            BuiltinEntityKind::Number => &["twenty-two", "1.2"],
            BuiltinEntityKind::Ordinal => &["the second"],
            BuiltinEntityKind::Temperature => &["Twenty three degrees celsius", "3°C"],
            BuiltinEntityKind::Time => &[
                "Today",
                "4:30 pm",
                "next monday at 8p.m.",
                "yesterday morning",
                "3rd tuesday of June",
                "June 2nd at 9 pm",
            ],
            BuiltinEntityKind::Percentage => &["twenty percent", "25%"],
        }
    }
}

impl BuiltinEntityKind {
    pub fn result_description(&self) -> Result<String> {
        Ok(match *self {
            BuiltinEntityKind::AmountOfMoney => serde_json::to_string(&vec![
                ::SlotValue::AmountOfMoney(::AmountOfMoneyValue {
                    value: 10.05,
                    precision: ::Precision::Approximate,
                    unit: Some("€".to_string()),
                })
            ]),
            BuiltinEntityKind::Duration => serde_json::to_string(&vec![
                ::SlotValue::Duration(::DurationValue {
                    years: 0,
                    quarters: 0,
                    months: 3,
                    weeks: 0,
                    days: 0,
                    hours: 0,
                    minutes: 0,
                    seconds: 0,
                    precision: ::Precision::Exact,
                }),
            ]),
            BuiltinEntityKind::Number => serde_json::to_string(&vec![
                ::SlotValue::Number(::NumberValue { value: 42. })
            ]),
            BuiltinEntityKind::Ordinal => serde_json::to_string(&vec![::SlotValue::Ordinal(::OrdinalValue { value: 2 })]),
            BuiltinEntityKind::Temperature => serde_json::to_string(&vec![
                ::SlotValue::Temperature(::TemperatureValue {
                    value: 23.0,
                    unit: Some("celsius".to_string()),
                }),
                ::SlotValue::Temperature(::TemperatureValue {
                    value: 60.0,
                    unit: Some("fahrenheit".to_string()),
                }),
            ]),
            BuiltinEntityKind::Time => serde_json::to_string(&vec![
                ::SlotValue::InstantTime(::InstantTimeValue {
                    value: "2017-06-13 18:00:00 +02:00".to_string(),
                    grain: ::Grain::Hour,
                    precision: ::Precision::Exact,
                }),
                ::SlotValue::TimeInterval(::TimeIntervalValue {
                    from: Some("2017-06-07 18:00:00 +02:00".to_string()),
                    to: Some("2017-06-08 00:00:00 +02:00".to_string()),
                }),
            ]),
            BuiltinEntityKind::Percentage => serde_json::to_string(&vec![
                ::SlotValue::Percentage(::PercentageValue { value: 20. }),
            ]),
        }?)
    }
}

impl BuiltinEntityKind {
    pub fn supported_languages(&self) -> &[Language] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[
                Language::EN,
                Language::FR,
                Language::DE,
                Language::ES,
                Language::KO,
            ],
            BuiltinEntityKind::Duration => &[
                Language::EN,
                Language::ES,
                Language::FR,
                Language::KO,
                Language::DE,
            ],
            BuiltinEntityKind::Number => &[
                Language::EN,
                Language::ES,
                Language::FR,
                Language::KO,
                Language::DE,
            ],
            BuiltinEntityKind::Ordinal => &[
                Language::EN,
                Language::ES,
                Language::FR,
                Language::KO,
                Language::DE,
            ],
            BuiltinEntityKind::Temperature => &[
                Language::EN,
                Language::ES,
                Language::FR,
                Language::KO,
                Language::DE,
            ],
            BuiltinEntityKind::Time => &[
                Language::EN,
                Language::ES,
                Language::FR,
                Language::KO,
                Language::DE,
            ],
            BuiltinEntityKind::Percentage => {
                &[Language::EN, Language::ES, Language::FR, Language::DE]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{Token, assert_tokens};

    #[test]
    fn test_result_descriptions() {
        // Given
        let description = BuiltinEntityKind::Temperature.result_description().unwrap();

        // When/Then
        let expected_description = "[{\"kind\":\"Temperature\",\"value\":23.0,\"unit\":\"celsius\"},{\"kind\":\"Temperature\",\"value\":60.0,\"unit\":\"fahrenheit\"}]";
        assert_eq!(expected_description, description);
    }

    #[test]
    fn test_builtin_entity_ser_de() {
        let entity = BuiltinEntity {
            value: "hello".to_string(),
            range: 12..42,
            entity: ::SlotValue::InstantTime(::InstantTimeValue {
                value: "some_value".into(),
                grain: ::Grain::Year,
                precision: ::Precision::Exact,
            }),
            entity_kind: BuiltinEntityKind::Time,
        };

        assert_tokens(&entity, &[
            Token::Struct { name: "BuiltinEntity", len: 4 },
            Token::Str("value"),
            Token::Str("hello"),
            Token::Str("range"),
            Token::Struct { name: "Range", len: 2 },
            Token::Str("start"),
            Token::U64(12),
            Token::Str("end"),
            Token::U64(42),
            Token::StructEnd,
            Token::Str("entity"),
            Token::Struct { name: "InstantTimeValue", len: 4 },
            Token::Str("kind"),
            Token::Str("InstantTime"),
            Token::Str("value"),
            Token::String("some_value"),
            Token::Str("grain"),
            Token::UnitVariant { name: "Grain", variant: "Year" },
            Token::Str("precision"),
            Token::UnitVariant { name: "Precision", variant: "Exact" },
            Token::StructEnd,
            Token::Str("entity_kind"),
            Token::Str("snips/datetime"),
            Token::StructEnd,
        ]);
    }
}
