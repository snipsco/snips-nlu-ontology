use nlu_ontology::*;
use rustling_ontology::Grain as RustlingGrain;
use rustling_ontology::dimension::Precision as RustlingPrecision;
use rustling_ontology::Lang as RustlingLanguage;
use rustling_ontology::output::{AmountOfMoneyOutput, DurationOutput, FloatOutput, IntegerOutput,
                                OrdinalOutput, Output, OutputKind, PercentageOutput,
                                TemperatureOutput, TimeIntervalOutput, TimeOutput};

pub trait FromRustling<T> {
    fn from_rustling(T) -> Self;
}

pub trait IntoBuiltin<T>: Sized {
    fn into_builtin(self) -> T;
}

impl<T, U> IntoBuiltin<U> for T
where
    U: FromRustling<T>,
{
    fn into_builtin(self) -> U {
        U::from_rustling(self)
    }
}

// From (and thus Into) is reflexive
impl<T> FromRustling<T> for T {
    fn from_rustling(t: T) -> T {
        t
    }
}

impl FromRustling<IntegerOutput> for NumberValue {
    fn from_rustling(rustling_output: IntegerOutput) -> Self {
        Self {
            value: rustling_output.0 as f64,
        }
    }
}

impl FromRustling<FloatOutput> for NumberValue {
    fn from_rustling(rustling_output: FloatOutput) -> Self {
        Self {
            value: rustling_output.0 as f64,
        }
    }
}

impl FromRustling<OrdinalOutput> for OrdinalValue {
    fn from_rustling(rustling_output: OrdinalOutput) -> Self {
        Self {
            value: rustling_output.0,
        }
    }
}

impl FromRustling<PercentageOutput> for PercentageValue {
    fn from_rustling(rustling_output: PercentageOutput) -> Self {
        Self {
            value: rustling_output.0 as f64,
        }
    }
}

impl FromRustling<TimeOutput> for InstantTimeValue {
    fn from_rustling(rustling_output: TimeOutput) -> Self {
        Self {
            value: rustling_output.moment.to_string(),
            grain: Grain::from_rustling(rustling_output.grain),
            precision: Precision::from_rustling(rustling_output.precision),
        }
    }
}

impl FromRustling<TimeIntervalOutput> for TimeIntervalValue {
    fn from_rustling(rustling_output: TimeIntervalOutput) -> Self {
        match rustling_output {
            TimeIntervalOutput::After(after) => Self {
                from: Some(after.moment.to_string()),
                to: None,
            },
            TimeIntervalOutput::Before(before) => Self {
                from: None,
                to: Some(before.moment.to_string()),
            },
            TimeIntervalOutput::Between {
                start,
                end,
                precision: _,
                latent: _,
            } => Self {
                from: Some(start.to_string()),
                to: Some(end.to_string()),
            },
        }
    }
}

impl FromRustling<AmountOfMoneyOutput> for AmountOfMoneyValue {
    fn from_rustling(rustling_output: AmountOfMoneyOutput) -> Self {
        Self {
            value: rustling_output.value,
            precision: rustling_output.precision.into_builtin(),
            unit: rustling_output.unit.map(|s| s.to_string()),
        }
    }
}

impl FromRustling<TemperatureOutput> for TemperatureValue {
    fn from_rustling(rustling_output: TemperatureOutput) -> Self {
        Self {
            value: rustling_output.value,
            unit: rustling_output.unit.map(|s| s.to_string()),
        }
    }
}

impl FromRustling<DurationOutput> for DurationValue {
    fn from_rustling(rustling_output: DurationOutput) -> Self {
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
            precision: rustling_output.precision.into_builtin(),
        }
    }
}

impl FromRustling<RustlingGrain> for Grain {
    fn from_rustling(rustling_output: RustlingGrain) -> Self {
        match rustling_output {
            RustlingGrain::Year => Grain::Year,
            RustlingGrain::Quarter => Grain::Quarter,
            RustlingGrain::Month => Grain::Month,
            RustlingGrain::Week => Grain::Week,
            RustlingGrain::Day => Grain::Day,
            RustlingGrain::Hour => Grain::Hour,
            RustlingGrain::Minute => Grain::Minute,
            RustlingGrain::Second => Grain::Second,
        }
    }
}

impl FromRustling<RustlingPrecision> for Precision {
    fn from_rustling(rustling_output: RustlingPrecision) -> Self {
        match rustling_output {
            RustlingPrecision::Approximate => Precision::Approximate,
            RustlingPrecision::Exact => Precision::Exact,
        }
    }
}

impl FromRustling<Output> for SlotValue {
    fn from_rustling(rustling_output: Output) -> Self {
        match rustling_output {
            Output::AmountOfMoney(v) => SlotValue::AmountOfMoney(v.into_builtin()),
            Output::Percentage(v) => SlotValue::Percentage(v.into_builtin()),
            Output::Duration(v) => SlotValue::Duration(v.into_builtin()),
            Output::Float(v) => SlotValue::Number(v.into_builtin()),
            Output::Integer(v) => SlotValue::Number(v.into_builtin()),
            Output::Ordinal(v) => SlotValue::Ordinal(v.into_builtin()),
            Output::Temperature(v) => SlotValue::Temperature(v.into_builtin()),
            Output::Time(v) => SlotValue::InstantTime(v.into_builtin()),
            Output::TimeInterval(v) => SlotValue::TimeInterval(v.into_builtin()),
        }
    }
}

impl<'a> FromRustling<&'a Output> for BuiltinEntityKind {
    fn from_rustling(v: &Output) -> Self {
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

impl<'a> FromRustling<&'a OutputKind> for BuiltinEntityKind {
    fn from_rustling(v: &OutputKind) -> Self {
        match *v {
            OutputKind::AmountOfMoney => BuiltinEntityKind::AmountOfMoney,
            OutputKind::Duration => BuiltinEntityKind::Duration,
            OutputKind::Number => BuiltinEntityKind::Number,
            OutputKind::Ordinal => BuiltinEntityKind::Ordinal,
            OutputKind::Temperature => BuiltinEntityKind::Temperature,
            OutputKind::Time => BuiltinEntityKind::Time,
            OutputKind::Percentage => BuiltinEntityKind::Percentage,
        }
    }
}

impl<'a> FromRustling<&'a BuiltinEntityKind> for OutputKind {
    fn from_rustling(v: &BuiltinEntityKind) -> Self {
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

impl FromRustling<Language> for RustlingLanguage {
    fn from_rustling(lang: Language) -> Self {
        match lang {
            Language::EN => RustlingLanguage::EN,
            Language::FR => RustlingLanguage::FR,
            Language::ES => RustlingLanguage::ES,
            Language::KO => RustlingLanguage::KO,
            Language::DE => RustlingLanguage::DE,
            Language::JA => RustlingLanguage::JA,
        }
    }
}
