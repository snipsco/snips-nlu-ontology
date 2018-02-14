use errors::*;
use rustling_ontology::Grain as RustlingGrain;
use rustling_ontology::dimension::Precision as RustlingPrecision;
use rustling_ontology::output::{AmountOfMoneyOutput, DurationOutput, FloatOutput, IntegerOutput,
                                OrdinalOutput, Output, OutputKind, TemperatureOutput, TimeIntervalOutput,
                                TimeOutput, PercentageOutput};

impl From<IntegerOutput> for ::NumberValue {
    fn from(rustling_output: IntegerOutput) -> Self {
        Self {
            value: rustling_output.0 as f64,
        }
    }
}

impl From<FloatOutput> for ::NumberValue {
    fn from(rustling_output: FloatOutput) -> Self {
        Self {
            value: rustling_output.0 as f64,
        }
    }
}

impl From<OrdinalOutput> for ::OrdinalValue {
    fn from(rustling_output: OrdinalOutput) -> Self {
        Self {
            value: rustling_output.0,
        }
    }
}

impl From<PercentageOutput> for ::PercentageValue {
    fn from(rustling_output: PercentageOutput) -> Self {
        Self {
            value: rustling_output.0 as f64,
        }
    }
}

impl From<TimeOutput> for ::InstantTimeValue {
    fn from(rustling_output: TimeOutput) -> Self {
        Self {
            value: rustling_output.moment.to_string(),
            grain: ::Grain::from(rustling_output.grain),
            precision: ::Precision::from(rustling_output.precision),
        }
    }
}

impl From<TimeIntervalOutput> for ::TimeIntervalValue {
    fn from(rustling_output: TimeIntervalOutput) -> Self {
        match rustling_output {
            TimeIntervalOutput::After(after) => Self {
                from: Some(after.moment.to_string()),
                to: None,
            },
            TimeIntervalOutput::Before(before) => Self {
                from: None,
                to: Some(before.moment.to_string()),
            },
            TimeIntervalOutput::Between { start, end, precision: _, latent: _ } => Self {
                from: Some(start.to_string()),
                to: Some(end.to_string()),
            },
        }
    }
}

impl From<AmountOfMoneyOutput> for ::AmountOfMoneyValue {
    fn from(rustling_output: AmountOfMoneyOutput) -> Self {
        Self {
            value: rustling_output.value,
            precision: rustling_output.precision.into(),
            unit: rustling_output.unit.map(|s| s.to_string()),
        }
    }
}

impl From<TemperatureOutput> for ::TemperatureValue {
    fn from(rustling_output: TemperatureOutput) -> Self {
        Self {
            value: rustling_output.value,
            unit: rustling_output.unit.map(|s| s.to_string()),
        }
    }
}

impl From<DurationOutput> for ::DurationValue {
    fn from(rustling_output: DurationOutput) -> Self {
        let mut years: i64 = 0;
        let mut quarters: i64 = 0;
        let mut months: i64 = 0;
        let mut weeks: i64 = 0;
        let mut days: i64 = 0;
        let mut hours: i64 = 0;
        let mut minutes: i64 = 0;
        let mut seconds: i64 = 0;
        for comp in rustling_output.period.comps().iter() {
            match comp.grain {
                RustlingGrain::Year => years = comp.quantity,
                RustlingGrain::Quarter => quarters = comp.quantity,
                RustlingGrain::Month => months = comp.quantity,
                RustlingGrain::Week => weeks = comp.quantity,
                RustlingGrain::Day => days = comp.quantity,
                RustlingGrain::Hour => hours = comp.quantity,
                RustlingGrain::Minute => minutes = comp.quantity,
                RustlingGrain::Second => seconds = comp.quantity,
            }
        }

        Self {
            years,
            quarters,
            months,
            weeks,
            days,
            hours,
            minutes,
            seconds,
            precision: rustling_output.precision.into(),
        }
    }
}

impl From<RustlingGrain> for ::Grain {
    fn from(rustling_output: RustlingGrain) -> Self {
        match rustling_output {
            RustlingGrain::Year => ::Grain::Year,
            RustlingGrain::Quarter => ::Grain::Quarter,
            RustlingGrain::Month => ::Grain::Month,
            RustlingGrain::Week => ::Grain::Week,
            RustlingGrain::Day => ::Grain::Day,
            RustlingGrain::Hour => ::Grain::Hour,
            RustlingGrain::Minute => ::Grain::Minute,
            RustlingGrain::Second => ::Grain::Second,
        }
    }
}

impl From<RustlingPrecision> for ::Precision {
    fn from(rustling_output: RustlingPrecision) -> Self {
        match rustling_output {
            RustlingPrecision::Approximate => ::Precision::Approximate,
            RustlingPrecision::Exact => ::Precision::Exact,
        }
    }
}

impl From<Output> for ::SlotValue {
    fn from(rustling_output: Output) -> Self {
        match rustling_output {
            Output::AmountOfMoney(v) => ::SlotValue::AmountOfMoney(v.into()),
            Output::Percentage(v) => ::SlotValue::Percentage(v.into()),
            Output::Duration(v) => ::SlotValue::Duration(v.into()),
            Output::Float(v) => ::SlotValue::Number(v.into()),
            Output::Integer(v) => ::SlotValue::Number(v.into()),
            Output::Ordinal(v) => ::SlotValue::Ordinal(v.into()),
            Output::Temperature(v) => ::SlotValue::Temperature(v.into()),
            Output::Time(v) => ::SlotValue::InstantTime(v.into()),
            Output::TimeInterval(v) => ::SlotValue::TimeInterval(v.into()),
        }
    }
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

impl<'a> From<&'a Output> for BuiltinEntityKind {
    fn from(v: &Output) -> Self {
        match *v {
            Output::AmountOfMoney(_) => BuiltinEntityKind::AmountOfMoney,
            Output::Duration(_) => BuiltinEntityKind::Duration,
            Output::Float(_) => BuiltinEntityKind::Number,
            Output::Integer(_) => BuiltinEntityKind::Number,
            Output::Ordinal(_) => BuiltinEntityKind::Ordinal,
            Output::Temperature(_) => BuiltinEntityKind::Temperature,
            Output::Time(_) => BuiltinEntityKind::Time,
            Output::TimeInterval(_) => BuiltinEntityKind::Time,
            Output::Percentage(_) => BuiltinEntityKind::Percentage,
        }
    }
}

impl<'a> From<&'a BuiltinEntityKind> for OutputKind {
    fn from(v: &BuiltinEntityKind) -> Self {
        match *v {
            BuiltinEntityKind::AmountOfMoney => OutputKind::AmountOfMoney,
            BuiltinEntityKind::Duration => OutputKind::Duration,
            BuiltinEntityKind::Number => OutputKind::Number,
            BuiltinEntityKind::Ordinal => OutputKind::Ordinal,
            BuiltinEntityKind::Temperature => OutputKind::Temperature,
            BuiltinEntityKind::Time => OutputKind::Time,
            BuiltinEntityKind::Percentage => OutputKind::Percentage,
        }
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
