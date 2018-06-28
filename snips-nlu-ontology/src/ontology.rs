use std::ops::Range;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct IntentParserResult {
    pub input: String,
    pub intent: Option<IntentClassifierResult>,
    pub slots: Option<Vec<Slot>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntentClassifierResult {
    pub intent_name: String,
    pub probability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    pub raw_value: String,
    pub value: SlotValue,
    pub range: Option<Range<usize>>,
    pub entity: String,
    pub slot_name: String,
}

impl Slot {
    pub fn new_custom(
        value: String,
        range: Range<usize>,
        entity: String,
        slot_name: String,
    ) -> Slot {
        Slot {
            raw_value: value.clone(),
            value: SlotValue::Custom(value.into()),
            range: Some(range),
            entity,
            slot_name,
        }
    }
}

impl Slot {
    pub fn with_slot_value(self, slot_value: SlotValue) -> Slot {
        Slot {
            raw_value: self.raw_value,
            value: slot_value,
            range: self.range,
            entity: self.entity,
            slot_name: self.slot_name,
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
    MusicArtist(StringValue)
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
    fn test_custom_slot() {
        let slot = Slot {
            raw_value: "value".into(),
            value: SlotValue::Custom("value".into()),
            range: None,
            entity: "toto".into(),
            slot_name: "toto".into(),
        };
        assert!(serde_json::to_string(&slot).is_ok());
        assert!(serde_json::from_str::<Slot>(&serde_json::to_string(&slot).unwrap()).is_ok());
    }

    #[test]
    fn test_builtin_slot_1() {
        let slot = Slot {
            raw_value: "fifth".into(),
            value: SlotValue::Ordinal(OrdinalValue { value: 5 }),
            range: None,
            entity: "toto".into(),
            slot_name: "toto".into(),
        };
        assert!(serde_json::to_string(&slot).is_ok());
        assert!(serde_json::from_str::<Slot>(&serde_json::to_string(&slot).unwrap()).is_ok());
    }

    #[test]
    fn test_builtin_slot_2() {
        let slot = Slot {
            raw_value: "some_value".into(),
            value: SlotValue::InstantTime(InstantTimeValue {
                value: "some_value".into(),
                grain: Grain::Year,
                precision: Precision::Exact,
            }),
            range: None,
            entity: "toto".into(),
            slot_name: "toto".into(),
        };
        assert!(serde_json::to_string(&slot).is_ok());
        assert!(serde_json::from_str::<Slot>(&serde_json::to_string(&slot).unwrap()).is_ok());
    }
}
