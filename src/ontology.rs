use std::ops::Range;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IntentParserResult {
    pub input: String,
    pub intent: IntentClassifierResult,
    pub slots: Vec<Slot>,
    #[serde(default)]
    pub alternatives: Vec<IntentParserAlternative>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IntentParserAlternative {
    pub intent: IntentClassifierResult,
    pub slots: Vec<Slot>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntentClassifierResult {
    pub intent_name: Option<String>,
    pub confidence_score: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    pub raw_value: String,
    pub value: SlotValue,
    #[serde(default)]
    pub alternatives: Vec<SlotValue>,
    pub range: Range<usize>,
    pub entity: String,
    pub slot_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<f32>,
}

impl Slot {
    pub fn new_custom(
        value: String,
        range: Range<usize>,
        entity: String,
        slot_name: String,
        confidence_score: Option<f32>,
        alternatives: Vec<SlotValue>,
    ) -> Slot {
        Slot {
            raw_value: value.clone(),
            value: SlotValue::Custom(value.into()),
            alternatives,
            range,
            entity,
            slot_name,
            confidence_score,
        }
    }
}

impl Slot {
    pub fn with_slot_value(self, slot_value: SlotValue) -> Slot {
        Slot {
            raw_value: self.raw_value,
            value: slot_value,
            alternatives: self.alternatives,
            range: self.range,
            entity: self.entity,
            slot_name: self.slot_name,
            confidence_score: self.confidence_score,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SlotValue {
    Custom(StringValue),
    Number(NumberValue),
    Ordinal(OrdinalValue),
    Percentage(PercentageValue),
    InstantTime(InstantTimeValue),
    TimeInterval(TimeIntervalValue),
    AmountOfMoney(AmountOfMoneyValue),
    Temperature(TemperatureValue),
    Duration(DurationValue),
    MusicAlbum(StringValue),
    MusicArtist(StringValue),
    MusicTrack(StringValue),
    City(StringValue),
    Country(StringValue),
    Region(StringValue),
}

/// This struct is required in order to use serde Internally tagged enum representation
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct StringValue {
    pub value: String,
}

impl From<String> for StringValue {
    fn from(string: String) -> Self {
        StringValue { value: string }
    }
}

impl From<&'static str> for StringValue {
    fn from(str: &str) -> Self {
        StringValue {
            value: str.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct NumberValue {
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct OrdinalValue {
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct PercentageValue {
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InstantTimeValue {
    pub value: String,
    pub grain: Grain,
    pub precision: Precision,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TimeIntervalValue {
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AmountOfMoneyValue {
    pub value: f32,
    pub precision: Precision,
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TemperatureValue {
    pub value: f32,
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct DurationValue {
    pub years: i64,
    pub quarters: i64,
    pub months: i64,
    pub weeks: i64,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub precision: Precision,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum Grain {
    Year = 0,
    Quarter = 1,
    Month = 2,
    Week = 3,
    Day = 4,
    Hour = 5,
    Minute = 6,
    Second = 7,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Precision {
    Approximate,
    Exact,
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_deserializing_with_default_alternatives() {
        // Given
        let intent_parser_result_json = r#"
            {
                "input": "foo bar baz",
                "intent": {
                    "intentName": "FooBar",
                    "confidenceScore": 0.42
                },
                "slots": [
                    {
                        "rawValue": "baz",
                        "value": {
                            "kind": "Custom",
                            "value": "baz"
                        },
                        "range": {
                            "start": 8,
                            "end": 11
                        },
                        "entity": "foo",
                        "slotName": "foo"
                    }
                ]
            }
        "#;

        // When
        let deserialized = serde_json::from_str(intent_parser_result_json).unwrap();

        // Then
        let expected_result = IntentParserResult {
            input: "foo bar baz".to_string(),
            intent: IntentClassifierResult {
                intent_name: Some("FooBar".to_string()),
                confidence_score: 0.42,
            },
            slots: vec![Slot {
                value: SlotValue::Custom("baz".into()),
                raw_value: "baz".to_string(),
                range: 8..11,
                entity: "foo".to_string(),
                slot_name: "foo".to_string(),
                alternatives: vec![],
                confidence_score: None,
            }],
            alternatives: vec![],
        };
        assert_eq!(expected_result, deserialized);
    }
}
