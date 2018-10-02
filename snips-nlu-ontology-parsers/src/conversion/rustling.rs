use conversion::*;
use errors::Result;
use nlu_ontology::*;
use rustling_ontology::ParserMatch;
use rustling_ontology::Grain as RustlingGrain;
use rustling_ontology::dimension::Precision as RustlingPrecision;
use rustling_ontology::Lang as RustlingLanguage;
use rustling_ontology::output::{AmountOfMoneyOutput, DurationOutput, FloatOutput, IntegerOutput,
                                OrdinalOutput, Output, OutputKind, PercentageOutput,
                                TemperatureOutput, TimeIntervalOutput, TimeOutput};

impl OntologyFrom<IntegerOutput> for NumberValue {
    fn ontology_from(rustling_output: IntegerOutput) -> Self {
        Self {
            value: rustling_output.0 as f64,
        }
    }
}

impl OntologyFrom<FloatOutput> for NumberValue {
    fn ontology_from(rustling_output: FloatOutput) -> Self {
        Self {
            value: rustling_output.0 as f64,
        }
    }
}

impl OntologyFrom<OrdinalOutput> for OrdinalValue {
    fn ontology_from(rustling_output: OrdinalOutput) -> Self {
        Self {
            value: rustling_output.0,
        }
    }
}

impl OntologyFrom<PercentageOutput> for PercentageValue {
    fn ontology_from(rustling_output: PercentageOutput) -> Self {
        Self {
            value: rustling_output.0 as f64,
        }
    }
}

impl OntologyFrom<TimeOutput> for InstantTimeValue {
    fn ontology_from(rustling_output: TimeOutput) -> Self {
        Self {
            value: rustling_output.moment.to_string(),
            grain: Grain::ontology_from(rustling_output.grain),
            precision: Precision::ontology_from(rustling_output.precision),
        }
    }
}

impl OntologyFrom<TimeIntervalOutput> for TimeIntervalValue {
    fn ontology_from(rustling_output: TimeIntervalOutput) -> Self {
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

impl OntologyFrom<AmountOfMoneyOutput> for AmountOfMoneyValue {
    fn ontology_from(rustling_output: AmountOfMoneyOutput) -> Self {
        Self {
            value: rustling_output.value,
            precision: rustling_output.precision.ontology_into(),
            unit: rustling_output.unit.map(|s| s.to_string()),
        }
    }
}

impl OntologyFrom<TemperatureOutput> for TemperatureValue {
    fn ontology_from(rustling_output: TemperatureOutput) -> Self {
        Self {
            value: rustling_output.value,
            unit: rustling_output.unit.map(|s| s.to_string()),
        }
    }
}

impl OntologyFrom<DurationOutput> for DurationValue {
    fn ontology_from(rustling_output: DurationOutput) -> Self {
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
            precision: rustling_output.precision.ontology_into(),
        }
    }
}

impl OntologyFrom<RustlingGrain> for Grain {
    fn ontology_from(rustling_output: RustlingGrain) -> Self {
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

impl OntologyFrom<RustlingPrecision> for Precision {
    fn ontology_from(rustling_output: RustlingPrecision) -> Self {
        match rustling_output {
            RustlingPrecision::Approximate => Precision::Approximate,
            RustlingPrecision::Exact => Precision::Exact,
        }
    }
}

impl OntologyFrom<Output> for SlotValue {
    fn ontology_from(rustling_output: Output) -> Self {
        match rustling_output {
            Output::AmountOfMoney(v) => SlotValue::AmountOfMoney(v.ontology_into()),
            Output::Percentage(v) => SlotValue::Percentage(v.ontology_into()),
            Output::Duration(v) => SlotValue::Duration(v.ontology_into()),
            Output::Float(v) => SlotValue::Number(v.ontology_into()),
            Output::Integer(v) => SlotValue::Number(v.ontology_into()),
            Output::Ordinal(v) => SlotValue::Ordinal(v.ontology_into()),
            Output::Temperature(v) => SlotValue::Temperature(v.ontology_into()),
            Output::Time(v) => SlotValue::InstantTime(v.ontology_into()),
            Output::TimeInterval(v) => SlotValue::TimeInterval(v.ontology_into()),
        }
    }
}

pub fn convert_to_builtin(input: &str, parser_match: ParserMatch<Output>) -> BuiltinEntity {
    BuiltinEntity {
        value: input[parser_match.byte_range.0..parser_match.byte_range.1].into(),
        range: parser_match.char_range.0..parser_match.char_range.1,
        entity: parser_match.value.clone().ontology_into(),
        entity_kind: BuiltinEntityKind::ontology_from(&parser_match.value),
    }
}

impl<'a> OntologyFrom<&'a Output> for BuiltinEntityKind {
    fn ontology_from(v: &Output) -> Self {
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

impl<'a> OntologyFrom<&'a OutputKind> for BuiltinEntityKind {
    fn ontology_from(v: &OutputKind) -> Self {
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

impl<'a> TryOntologyFrom<&'a BuiltinEntityKind> for OutputKind {
    fn try_ontology_from(v: &BuiltinEntityKind) -> Result<Self> {
        match *v {
            BuiltinEntityKind::AmountOfMoney => Ok(OutputKind::AmountOfMoney),
            BuiltinEntityKind::Duration => Ok(OutputKind::Duration),
            BuiltinEntityKind::Number => Ok(OutputKind::Number),
            BuiltinEntityKind::Ordinal => Ok(OutputKind::Ordinal),
            BuiltinEntityKind::Temperature => Ok(OutputKind::Temperature),
            BuiltinEntityKind::Time => Ok(OutputKind::Time),
            BuiltinEntityKind::Percentage => Ok(OutputKind::Percentage),
            _ => Err(format_err!("Cannot convert {:?} into rustling type", v)),
        }
    }
}

impl OntologyFrom<Language> for RustlingLanguage {
    fn ontology_from(lang: Language) -> Self {
        match lang {
            Language::DE => RustlingLanguage::DE,
            Language::EN => RustlingLanguage::EN,
            Language::ES => RustlingLanguage::ES,
            Language::FR => RustlingLanguage::FR,
            Language::IT => RustlingLanguage::IT,
            Language::JA => RustlingLanguage::JA,
            Language::KO => RustlingLanguage::KO,
        }
    }
}
