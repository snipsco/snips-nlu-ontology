use std::ops::Range;

use errors::*;
use language::Language;

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltinEntity {
    pub value: String,
    pub range: Range<usize>,
    pub entity: ::SlotValue,
    pub entity_kind: ::BuiltinEntityKind,
}

enum_kind!(BuiltinEntityKind,
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
            .ok_or(
                format!("Unknown EntityKind identifier: {}", identifier).into(),
            )
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
            BuiltinEntityKind::AmountOfMoney => &[
                "ten dollars and five cents",
                "around 5€"
            ],
            BuiltinEntityKind::Duration => &[
                "3 month",
                "4 seconds",
                "8 years"
            ],
            BuiltinEntityKind::Number => &[
                "twenty-two",
                "1.2"
            ],
            BuiltinEntityKind::Ordinal => &[
                "the second"
            ],
            BuiltinEntityKind::Temperature => &[
                "Twenty three degrees celsius",
                "3°C"
            ],
            BuiltinEntityKind::Time => &[
                "Today",
                "4:30 pm",
                "next monday at 8p.m.",
                "yesterday morning",
                "3rd tuesday of June",
                "June 2nd at 9 pm"
            ],
            BuiltinEntityKind::Percentage => &[
                "twenty percent",
                "25%"
            ],
        }
    }
}

impl BuiltinEntityKind {
    pub fn result_description(&self) -> &str {
        match *self {
            BuiltinEntityKind::AmountOfMoney => "[{\"kind\":\"Builtin\",\"value\":{\"kind\":\"AmountOfMoney\",\"value\":{\"value\":10.05,\"precision\":\"Approximate\",\"unit\":\"€\"}}}]",
            BuiltinEntityKind::Duration => "[{\"kind\":\"Builtin\",\"value\":{\"kind\":\"Duration\",\"value\":{\"years\":0,\"quarters\":0,\"months\":3,\"weeks\":0,\"days\":0,\"hours\":0,\"minutes\":0,\"seconds\":0,\"precision\":\"Exact\"}}}]",
            BuiltinEntityKind::Number => "[{\"kind\":\"Builtin\",\"value\":{\"kind\":\"Number\",\"value\":22}}, {\"kind\":\"Builtin\",\"value\":{\"kind\":\"Number\",\"value\":2.5}}]",
            BuiltinEntityKind::Ordinal => "[{\"kind\":\"Builtin\",\"value\":{\"kind\":\"Ordinal\",\"value\":2}}]",
            BuiltinEntityKind::Temperature => "[{\"kind\":\"Builtin\",\"value\":{\"kind\":\"Temperature\",\"value\":{\"value\":23.0,\"unit\":\"celsius\"}}},{\"kind\":\"Builtin\",\"value\":{\"kind\":\"Temperature\",\"value\":{\"value\":60.0,\"unit\":\"fahrenheit\"}}}]",
            BuiltinEntityKind::Time => "[{\"kind\":\"Builtin\",\"value\":{\"kind\":\"Time\",\"value\":{\"kind\":\"InstantTime\",\"value\":{\"value\":\"2017-06-13 18:00:00 +02:00\",\"grain\":\"Hour\",\"precision\":\"Exact\"}}}},{\"kind\":\"Builtin\",\"value\":{\"kind\":\"Time\",\"value\":{\"kind\":\"TimeInterval\",\"value\":{\"from\":\"2017-06-07 18:00:00 +02:00\",\"to\":\"2017-06-08 00:00:00 +02:00\"}}}}]",
            BuiltinEntityKind::Percentage => "[{\"kind\":\"Builtin\",\"value\":{\"kind\":\"Percentage\",\"value\":20}}, {\"kind\":\"Builtin\",\"value\":{\"kind\":\"Percentage\",\"value\": 25}}]",
        }
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
                Language::KO
            ],
            BuiltinEntityKind::Duration => &[
                Language::EN,
                Language::ES,
                Language::FR,
                Language::KO,
                Language::DE
            ],
            BuiltinEntityKind::Number => &[
                Language::EN,
                Language::ES,
                Language::FR,
                Language::KO,
                Language::DE
            ],
            BuiltinEntityKind::Ordinal => &[
                Language::EN,
                Language::ES,
                Language::FR,
                Language::KO,
                Language::DE
            ],
            BuiltinEntityKind::Temperature => &[
                Language::EN,
                Language::ES,
                Language::FR,
                Language::KO,
                Language::DE
            ],
            BuiltinEntityKind::Time => &[
                Language::EN,
                Language::ES,
                Language::FR,
                Language::KO,
                Language::DE
            ],
            BuiltinEntityKind::Percentage => &[
                Language::EN,
                Language::ES,
                Language::FR,
                Language::DE
            ],
        }
    }
}
