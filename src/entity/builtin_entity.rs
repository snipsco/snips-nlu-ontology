use crate::enum_kind;
use crate::errors::*;
use crate::ontology::*;
use failure::format_err;
use serde::Deserialize;
use serde_json;
use std::ops::Range;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BuiltinEntity {
    pub value: String,
    pub range: Range<usize>,
    pub entity: SlotValue,
    #[serde(
        serialize_with = "serialize_builtin_entity_kind",
        deserialize_with = "deserialize_builtin_entity_kind"
    )]
    pub entity_kind: BuiltinEntityKind,
}

fn serialize_builtin_entity_kind<S>(
    value: &BuiltinEntityKind,
    serializer: S,
) -> ::std::result::Result<S::Ok, S::Error>
where
    S: ::serde::Serializer,
{
    serializer.serialize_str(value.identifier())
}

fn deserialize_builtin_entity_kind<'de, D>(
    deserializer: D,
) -> ::std::result::Result<BuiltinEntityKind, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
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
        Datetime,
        Date,
        Time,
        DatePeriod,
        TimePeriod,
        Percentage,
        MusicAlbum,
        MusicArtist,
        MusicTrack,
        City,
        Country,
        Region
    ]
);

pub trait IntoBuiltinEntityKind: Copy {
    fn into_builtin_kind(self) -> BuiltinEntityKind;

    fn identifier(&self) -> &'static str {
        self.into_builtin_kind().identifier()
    }

    fn description(&self) -> &'static str {
        self.into_builtin_kind().description()
    }

    fn result_description(&self) -> String {
        self.into_builtin_kind().result_description()
    }
}

impl BuiltinEntityKind {
    pub fn identifier(&self) -> &'static str {
        match *self {
            BuiltinEntityKind::AmountOfMoney => "snips/amountOfMoney",
            BuiltinEntityKind::Duration => "snips/duration",
            BuiltinEntityKind::Number => "snips/number",
            BuiltinEntityKind::Ordinal => "snips/ordinal",
            BuiltinEntityKind::Temperature => "snips/temperature",
            BuiltinEntityKind::Datetime => "snips/datetime",
            BuiltinEntityKind::Date => "snips/date",
            BuiltinEntityKind::Time => "snips/time",
            BuiltinEntityKind::DatePeriod => "snips/datePeriod",
            BuiltinEntityKind::TimePeriod => "snips/timePeriod",
            BuiltinEntityKind::Percentage => "snips/percentage",
            BuiltinEntityKind::MusicAlbum => "snips/musicAlbum",
            BuiltinEntityKind::MusicArtist => "snips/musicArtist",
            BuiltinEntityKind::MusicTrack => "snips/musicTrack",
            BuiltinEntityKind::City => "snips/city",
            BuiltinEntityKind::Country => "snips/country",
            BuiltinEntityKind::Region => "snips/region",
        }
    }

    pub fn from_identifier(identifier: &str) -> Result<Self> {
        BuiltinEntityKind::all()
            .iter()
            .find(|kind| kind.identifier() == identifier)
            .map(|k| k.clone())
            .ok_or(format_err!("Unknown EntityKind identifier: {}", identifier))
    }
}

impl BuiltinEntityKind {
    pub fn description(&self) -> &'static str {
        match *self {
            BuiltinEntityKind::AmountOfMoney => "Matches an amount of money",
            BuiltinEntityKind::Duration => "Matches a time duration",
            BuiltinEntityKind::Number => "Matches a cardinal number",
            BuiltinEntityKind::Ordinal => "Matches an ordinal number",
            BuiltinEntityKind::Temperature => "Matches a temperature",
            BuiltinEntityKind::Datetime => {
                "Matches a date, time, interval or a date and time together"
            }
            BuiltinEntityKind::Date => "Matches a date",
            BuiltinEntityKind::Time => "Matches a time of day",
            BuiltinEntityKind::DatePeriod => {
                "Matches a period of time spanning over days or larger grains"
            }
            BuiltinEntityKind::TimePeriod => {
                "Matches a period of time spanning over hours or smaller grains"
            }
            BuiltinEntityKind::Percentage => "Matches a percentage",
            BuiltinEntityKind::MusicAlbum => "Matches a music album",
            BuiltinEntityKind::MusicArtist => "Matches a music artist",
            BuiltinEntityKind::MusicTrack => "Matches a music track",
            BuiltinEntityKind::City => "Matches main local and world cities",
            BuiltinEntityKind::Country => "Matches countries around the world",
            BuiltinEntityKind::Region => "Matches local administrative regions",
        }
    }
}

impl BuiltinEntityKind {
    pub fn result_description(&self) -> String {
        match *self {
            BuiltinEntityKind::AmountOfMoney => {
                serde_json::to_string_pretty(&vec![SlotValue::AmountOfMoney(AmountOfMoneyValue {
                    value: 10.05,
                    precision: Precision::Approximate,
                    unit: Some("€".to_string()),
                })])
            }
            BuiltinEntityKind::Duration => {
                serde_json::to_string_pretty(&vec![SlotValue::Duration(DurationValue {
                    years: 0,
                    quarters: 0,
                    months: 3,
                    weeks: 0,
                    days: 0,
                    hours: 0,
                    minutes: 0,
                    seconds: 0,
                    precision: Precision::Exact,
                })])
            }
            BuiltinEntityKind::Number => {
                serde_json::to_string_pretty(&vec![SlotValue::Number(NumberValue { value: 42. })])
            }
            BuiltinEntityKind::Ordinal => {
                serde_json::to_string_pretty(&vec![SlotValue::Ordinal(OrdinalValue { value: 2 })])
            }
            BuiltinEntityKind::Temperature => serde_json::to_string_pretty(&vec![
                SlotValue::Temperature(TemperatureValue {
                    value: 23.0,
                    unit: Some("celsius".to_string()),
                }),
                SlotValue::Temperature(TemperatureValue {
                    value: 60.0,
                    unit: Some("fahrenheit".to_string()),
                }),
            ]),
            BuiltinEntityKind::Datetime => serde_json::to_string_pretty(&vec![
                SlotValue::InstantTime(InstantTimeValue {
                    value: "2017-06-13 18:00:00 +02:00".to_string(),
                    grain: Grain::Hour,
                    precision: Precision::Exact,
                }),
                SlotValue::TimeInterval(TimeIntervalValue {
                    from: Some("2017-06-07 18:00:00 +02:00".to_string()),
                    to: Some("2017-06-08 00:00:00 +02:00".to_string()),
                }),
            ]),
            BuiltinEntityKind::Date => {
                serde_json::to_string_pretty(&vec![SlotValue::InstantTime(InstantTimeValue {
                    value: "2017-06-13 00:00:00 +02:00".to_string(),
                    grain: Grain::Day,
                    precision: Precision::Exact,
                })])
            }
            BuiltinEntityKind::Time => {
                serde_json::to_string_pretty(&vec![SlotValue::InstantTime(InstantTimeValue {
                    value: "2017-06-13 18:00:00 +02:00".to_string(),
                    grain: Grain::Hour,
                    precision: Precision::Exact,
                })])
            }
            BuiltinEntityKind::DatePeriod => {
                serde_json::to_string_pretty(&vec![SlotValue::TimeInterval(TimeIntervalValue {
                    from: Some("2017-06-07 00:00:00 +02:00".to_string()),
                    to: Some("2017-06-09 00:00:00 +02:00".to_string()),
                })])
            }
            BuiltinEntityKind::TimePeriod => {
                serde_json::to_string_pretty(&vec![SlotValue::TimeInterval(TimeIntervalValue {
                    from: Some("2017-06-07 18:00:00 +02:00".to_string()),
                    to: Some("2017-06-07 20:00:00 +02:00".to_string()),
                })])
            }
            BuiltinEntityKind::Percentage => {
                serde_json::to_string_pretty(&vec![SlotValue::Percentage(PercentageValue {
                    value: 20.,
                })])
            }
            BuiltinEntityKind::MusicAlbum => {
                serde_json::to_string_pretty(&vec![SlotValue::MusicAlbum(StringValue {
                    value: "Discovery".to_string(),
                })])
            }
            BuiltinEntityKind::MusicArtist => {
                serde_json::to_string_pretty(&vec![SlotValue::MusicArtist(StringValue {
                    value: "Daft Punk".to_string(),
                })])
            }
            BuiltinEntityKind::MusicTrack => {
                serde_json::to_string_pretty(&vec![SlotValue::MusicTrack(StringValue {
                    value: "Harder Better Faster Stronger".to_string(),
                })])
            }
            BuiltinEntityKind::City => {
                serde_json::to_string_pretty(&vec![SlotValue::City(StringValue {
                    value: "Paris".to_string(),
                })])
            }
            BuiltinEntityKind::Country => {
                serde_json::to_string_pretty(&vec![SlotValue::Country(StringValue {
                    value: "France".to_string(),
                })])
            }
            BuiltinEntityKind::Region => {
                serde_json::to_string_pretty(&vec![SlotValue::Region(StringValue {
                    value: "California".to_string(),
                })])
            }
        }
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_result_descriptions() {
        // Given
        let description = BuiltinEntityKind::Percentage.result_description();

        // When/Then
        let expected_description =
            "[\n  {\n    \"kind\": \"Percentage\",\n    \"value\": 20.0\n  }\n]";
        assert_eq!(expected_description, description);
    }

    #[test]
    fn test_builtin_entity_ser_de() {
        let entity = BuiltinEntity {
            value: "hello".to_string(),
            range: 12..42,
            entity: SlotValue::InstantTime(InstantTimeValue {
                value: "some_value".into(),
                grain: Grain::Year,
                precision: Precision::Exact,
            }),
            entity_kind: BuiltinEntityKind::Datetime,
        };

        assert_tokens(
            &entity,
            &[
                Token::Struct {
                    name: "BuiltinEntity",
                    len: 4,
                },
                Token::Str("value"),
                Token::Str("hello"),
                Token::Str("range"),
                Token::Struct {
                    name: "Range",
                    len: 2,
                },
                Token::Str("start"),
                Token::U64(12),
                Token::Str("end"),
                Token::U64(42),
                Token::StructEnd,
                Token::Str("entity"),
                Token::Struct {
                    name: "InstantTimeValue",
                    len: 4,
                },
                Token::Str("kind"),
                Token::Str("InstantTime"),
                Token::Str("value"),
                Token::String("some_value"),
                Token::Str("grain"),
                Token::UnitVariant {
                    name: "Grain",
                    variant: "Year",
                },
                Token::Str("precision"),
                Token::UnitVariant {
                    name: "Precision",
                    variant: "Exact",
                },
                Token::StructEnd,
                Token::Str("entity_kind"),
                Token::Str("snips/datetime"),
                Token::StructEnd,
            ],
        );
    }
}
