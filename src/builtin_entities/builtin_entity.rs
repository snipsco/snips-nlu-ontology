use std::ops::Range;

use errors::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltinEntity {
    pub value: String,
    pub range: Range<usize>,
    pub entity: ::SlotValue,
    pub entity_kind: ::BuiltinEntityKind,
}

#[derive(Copy, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinEntityKind {
    AmountOfMoney,
    Duration,
    Number,
    Ordinal,
    Temperature,
    Time,
    Percentage,
}

impl BuiltinEntityKind {
    pub fn all() -> Vec<Self> {
        vec![
            BuiltinEntityKind::AmountOfMoney,
            BuiltinEntityKind::Duration,
            BuiltinEntityKind::Number,
            BuiltinEntityKind::Ordinal,
            BuiltinEntityKind::Temperature,
            BuiltinEntityKind::Time,
            BuiltinEntityKind::Percentage,
        ]
    }
}

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
            .into_iter()
            .find(|kind| kind.identifier() == identifier)
            .ok_or(
                format!("Unknown EntityKind identifier: {}", identifier).into(),
            )
    }
}
