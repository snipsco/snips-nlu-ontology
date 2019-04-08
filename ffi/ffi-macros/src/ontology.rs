#![allow(non_camel_case_types)]

use std::ffi::CString;
use std::ptr::null;
use std::slice;

use failure::{bail, Fallible, ResultExt};
use ffi_utils::{
    create_optional_rust_string_from, create_rust_string_from, take_back_c_string,
    take_back_nullable_c_string, AsRust, RawPointerConverter,
};
use libc;
use snips_nlu_ontology::*;

/// Result of intent parsing
#[repr(C)]
#[derive(Debug)]
pub struct CIntentParserResult {
    /// The text that was parsed
    pub input: *const libc::c_char,
    /// The result of intent classification
    pub intent: *const CIntentClassifierResult,
    /// The slots extracted
    pub slots: *const CSlotList,
}

impl From<IntentParserResult> for CIntentParserResult {
    fn from(input: IntentParserResult) -> Self {
        Self {
            input: CString::new(input.input).unwrap().into_raw(),
            intent: CIntentClassifierResult::from(input.intent).into_raw_pointer(),
            slots: CSlotList::from(input.slots).into_raw_pointer(),
        }
    }
}

impl AsRust<IntentParserResult> for CIntentParserResult {
    fn as_rust(&self) -> Fallible<IntentParserResult> {
        Ok(IntentParserResult {
            input: create_rust_string_from!(self.input),
            intent: unsafe { &*self.intent }.as_rust()?,
            slots: unsafe { &*self.slots }.as_rust()?,
        })
    }
}

impl Drop for CIntentParserResult {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        let _ = unsafe { CIntentClassifierResult::from_raw_pointer(self.intent) };
        let _ = unsafe { CSlotList::from_raw_pointer(self.slots) };
    }
}

/// Results of the intent classifier
#[repr(C)]
#[derive(Debug)]
pub struct CIntentClassifierResult {
    /// Name of the intent detected
    pub intent_name: *const libc::c_char,
    /// Between 0 and 1
    pub confidence_score: libc::c_float,
}

impl From<IntentClassifierResult> for CIntentClassifierResult {
    fn from(input: IntentClassifierResult) -> Self {
        let intent_name = input
            .intent_name
            .map(|name| CString::new(name).unwrap().into_raw() as *const _)
            .unwrap_or_else(|| null());
        Self {
            intent_name,
            confidence_score: input.confidence_score,
        }
    }
}

impl AsRust<IntentClassifierResult> for CIntentClassifierResult {
    fn as_rust(&self) -> Fallible<IntentClassifierResult> {
        Ok(IntentClassifierResult {
            intent_name: create_optional_rust_string_from!(self.intent_name),
            confidence_score: self.confidence_score as f32,
        })
    }
}

impl Drop for CIntentClassifierResult {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.intent_name);
    }
}

/// Wrapper around a list of IntentClassifierResult
#[repr(C)]
#[derive(Debug)]
pub struct CIntentClassifierResultArray {
    /// Pointer to the first result of the list
    pub intent_classifier_results: *const CIntentClassifierResult,
    /// Number of results in the list
    pub size: libc::int32_t,
}

impl From<Vec<IntentClassifierResult>> for CIntentClassifierResultArray {
    fn from(input: Vec<IntentClassifierResult>) -> Self {
        Self {
            size: input.len() as libc::int32_t,
            intent_classifier_results: Box::into_raw(
                input
                    .into_iter()
                    .map(CIntentClassifierResult::from)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ) as *const CIntentClassifierResult,
        }
    }
}

impl AsRust<Vec<IntentClassifierResult>> for CIntentClassifierResultArray {
    fn as_rust(&self) -> Fallible<Vec<IntentClassifierResult>> {
        let mut result = vec![];
        let ic_results = unsafe {
            std::slice::from_raw_parts_mut(
                self.intent_classifier_results as *mut CIntentClassifierResult,
                self.size as usize,
            )
        };

        for ic_result in ic_results {
            result.push(ic_result.as_rust()?)
        }
        Ok(result)
    }
}

impl Drop for CIntentClassifierResultArray {
    fn drop(&mut self) {
        let _ = unsafe {
            Box::from_raw(slice::from_raw_parts_mut(
                self.intent_classifier_results as *mut CIntentClassifierResult,
                self.size as usize,
            ))
        };
    }
}

/// Wrapper around a slot list
#[repr(C)]
#[derive(Debug)]
pub struct CSlotList {
    /// Pointer to the first slot of the list
    pub slots: *const CSlot,
    /// Number of slots in the list
    pub size: libc::int32_t, // Note: we can't use `libc::size_t` because it's not supported by JNA
}

impl From<Vec<Slot>> for CSlotList {
    fn from(input: Vec<Slot>) -> Self {
        Self {
            size: input.len() as libc::int32_t,
            slots: Box::into_raw(
                input
                    .into_iter()
                    .map(CSlot::from)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ) as *const CSlot,
        }
    }
}

impl AsRust<Vec<Slot>> for CSlotList {
    fn as_rust(&self) -> Fallible<Vec<Slot>> {
        let mut result = vec![];
        let slots =
            unsafe { std::slice::from_raw_parts_mut(self.slots as *mut CSlot, self.size as usize) };

        for slot in slots {
            result.push(slot.as_rust()?)
        }
        Ok(result)
    }
}

impl Drop for CSlotList {
    fn drop(&mut self) {
        let _ = unsafe {
            Box::from_raw(slice::from_raw_parts_mut(
                self.slots as *mut CSlot,
                self.size as usize,
            ))
        };
    }
}

/// Struct describing a Slot
#[repr(C)]
#[derive(Debug)]
pub struct CSlot {
    /// The resolved value of the slot
    pub value: CSlotValue,
    /// The raw value as it appears in the input text
    pub raw_value: *const libc::c_char,
    /// Name of the entity type of the slot
    pub entity: *const libc::c_char,
    /// Name of the slot
    pub slot_name: *const libc::c_char,
    /// Start index of raw value in input text
    pub range_start: libc::int32_t,
    /// End index of raw value in input text
    pub range_end: libc::int32_t,
    /// Confidence score of the slot
    pub confidence_score: libc::c_float,
}

impl From<Slot> for CSlot {
    fn from(input: Slot) -> Self {
        Self {
            raw_value: CString::new(input.raw_value).unwrap().into_raw(),
            value: CSlotValue::from(input.value),
            range_start: input.range.start as libc::int32_t,
            range_end: input.range.end as libc::int32_t,
            entity: CString::new(input.entity).unwrap().into_raw(),
            slot_name: CString::new(input.slot_name).unwrap().into_raw(),
            confidence_score: input
                .confidence_score
                .map(|v| v as libc::c_float)
                .unwrap_or(-1.),
        }
    }
}

impl AsRust<Slot> for CSlot {
    fn as_rust(&self) -> Fallible<Slot> {
        Ok(Slot {
            raw_value: create_rust_string_from!(self.raw_value),
            value: self.value.as_rust()?,
            range: (self.range_start as usize..self.range_end as usize),
            entity: create_rust_string_from!(self.entity),
            slot_name: create_rust_string_from!(self.slot_name),
            confidence_score: if self.confidence_score < 0.0 {
                None
            } else {
                Some(self.confidence_score)
            },
        })
    }
}

impl Drop for CSlot {
    fn drop(&mut self) {
        take_back_c_string!(self.raw_value);
        take_back_c_string!(self.entity);
        take_back_c_string!(self.slot_name);
    }
}

/// Enum type describing how to cast the value of a CSlotValue
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum SNIPS_SLOT_VALUE_TYPE {
    /// Custom type represented by a char *
    SNIPS_SLOT_VALUE_TYPE_CUSTOM = 1,
    /// Number type represented by a CNumberValue
    SNIPS_SLOT_VALUE_TYPE_NUMBER = 2,
    /// Ordinal type represented by a COrdinalValue
    SNIPS_SLOT_VALUE_TYPE_ORDINAL = 3,
    /// Instant type represented by a CInstantTimeValue
    SNIPS_SLOT_VALUE_TYPE_INSTANTTIME = 4,
    /// Interval type represented by a CTimeIntervalValue
    SNIPS_SLOT_VALUE_TYPE_TIMEINTERVAL = 5,
    /// Amount of money type represented by a CAmountOfMoneyValue
    SNIPS_SLOT_VALUE_TYPE_AMOUNTOFMONEY = 6,
    /// Temperature type represented by a CTemperatureValue
    SNIPS_SLOT_VALUE_TYPE_TEMPERATURE = 7,
    /// Duration type represented by a CDurationValue
    SNIPS_SLOT_VALUE_TYPE_DURATION = 8,
    /// Percentage type represented by a CPercentageValue
    SNIPS_SLOT_VALUE_TYPE_PERCENTAGE = 9,
    /// Music Album type represented by a char *
    SNIPS_SLOT_VALUE_TYPE_MUSICALBUM = 10,
    /// Music Artist type represented by a char *
    SNIPS_SLOT_VALUE_TYPE_MUSICARTIST = 11,
    /// Music Track type represented by a char *
    SNIPS_SLOT_VALUE_TYPE_MUSICTRACK = 12,
}

impl<'a> From<&'a SlotValue> for SNIPS_SLOT_VALUE_TYPE {
    fn from(slot_value: &SlotValue) -> Self {
        match slot_value {
            &crate::SlotValue::Custom(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_CUSTOM,
            &SlotValue::Number(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_NUMBER,
            &SlotValue::Ordinal(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_ORDINAL,
            &SlotValue::InstantTime(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_INSTANTTIME,
            &SlotValue::TimeInterval(_) => {
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_TIMEINTERVAL
            }
            &SlotValue::AmountOfMoney(_) => {
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_AMOUNTOFMONEY
            }
            &SlotValue::Temperature(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_TEMPERATURE,
            &SlotValue::Duration(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_DURATION,
            &SlotValue::Percentage(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_PERCENTAGE,
            &SlotValue::MusicAlbum(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_MUSICALBUM,
            &SlotValue::MusicArtist(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_MUSICARTIST,
            &SlotValue::MusicTrack(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_MUSICTRACK,
        }
    }
}

/// Enum describing the precision of a resolved value
#[repr(C)]
#[derive(Debug)]
pub enum SNIPS_PRECISION {
    /// The resolved value is approximate
    SNIPS_PRECISION_APPROXIMATE = 0,
    /// The resolved value is exact
    SNIPS_PRECISION_EXACT = 1,
}

impl From<Precision> for SNIPS_PRECISION {
    fn from(value: Precision) -> Self {
        match value {
            Precision::Approximate => SNIPS_PRECISION::SNIPS_PRECISION_APPROXIMATE,
            Precision::Exact => SNIPS_PRECISION::SNIPS_PRECISION_EXACT,
        }
    }
}

impl AsRust<Precision> for SNIPS_PRECISION {
    fn as_rust(&self) -> Fallible<Precision> {
        Ok(match self {
            SNIPS_PRECISION::SNIPS_PRECISION_APPROXIMATE => Precision::Approximate,
            SNIPS_PRECISION::SNIPS_PRECISION_EXACT => Precision::Exact,
        })
    }
}

/// Representation of a number value
pub type CNumberValue = libc::c_double;
/// Representation of a percentage value
pub type CPercentageValue = libc::c_double;
/// Representation of an ordinal value
pub type COrdinalValue = libc::int64_t;

/// Enum representing the grain of a resolved date related value
#[repr(C)]
#[derive(Debug)]
pub enum SNIPS_GRAIN {
    /// The resolved value has a granularity of a year
    SNIPS_GRAIN_YEAR = 0,
    /// The resolved value has a granularity of a quarter
    SNIPS_GRAIN_QUARTER = 1,
    /// The resolved value has a granularity of a mount
    SNIPS_GRAIN_MONTH = 2,
    /// The resolved value has a granularity of a week
    SNIPS_GRAIN_WEEK = 3,
    /// The resolved value has a granularity of a day
    SNIPS_GRAIN_DAY = 4,
    /// The resolved value has a granularity of an hour
    SNIPS_GRAIN_HOUR = 5,
    /// The resolved value has a granularity of a minute
    SNIPS_GRAIN_MINUTE = 6,
    /// The resolved value has a granularity of a second
    SNIPS_GRAIN_SECOND = 7,
}

impl From<Grain> for SNIPS_GRAIN {
    fn from(value: Grain) -> Self {
        match value {
            Grain::Year => SNIPS_GRAIN::SNIPS_GRAIN_YEAR,
            Grain::Quarter => SNIPS_GRAIN::SNIPS_GRAIN_QUARTER,
            Grain::Month => SNIPS_GRAIN::SNIPS_GRAIN_MONTH,
            Grain::Week => SNIPS_GRAIN::SNIPS_GRAIN_WEEK,
            Grain::Day => SNIPS_GRAIN::SNIPS_GRAIN_DAY,
            Grain::Hour => SNIPS_GRAIN::SNIPS_GRAIN_HOUR,
            Grain::Minute => SNIPS_GRAIN::SNIPS_GRAIN_MINUTE,
            Grain::Second => SNIPS_GRAIN::SNIPS_GRAIN_SECOND,
        }
    }
}

impl AsRust<Grain> for SNIPS_GRAIN {
    fn as_rust(&self) -> Fallible<Grain> {
        Ok(match self {
            SNIPS_GRAIN::SNIPS_GRAIN_YEAR => Grain::Year,
            SNIPS_GRAIN::SNIPS_GRAIN_QUARTER => Grain::Quarter,
            SNIPS_GRAIN::SNIPS_GRAIN_MONTH => Grain::Month,
            SNIPS_GRAIN::SNIPS_GRAIN_WEEK => Grain::Week,
            SNIPS_GRAIN::SNIPS_GRAIN_DAY => Grain::Day,
            SNIPS_GRAIN::SNIPS_GRAIN_HOUR => Grain::Hour,
            SNIPS_GRAIN::SNIPS_GRAIN_MINUTE => Grain::Minute,
            SNIPS_GRAIN::SNIPS_GRAIN_SECOND => Grain::Second,
        })
    }
}

/// Representation of an instant value
#[repr(C)]
#[derive(Debug)]
pub struct CInstantTimeValue {
    /// String representation of the instant
    pub value: *const libc::c_char,
    /// The grain of the resolved instant
    pub grain: SNIPS_GRAIN,
    /// The precision of the resolved instant
    pub precision: SNIPS_PRECISION,
}

impl From<InstantTimeValue> for CInstantTimeValue {
    fn from(value: InstantTimeValue) -> Self {
        Self {
            value: CString::new(value.value).unwrap().into_raw(),
            grain: SNIPS_GRAIN::from(value.grain),
            precision: SNIPS_PRECISION::from(value.precision),
        }
    }
}

impl AsRust<InstantTimeValue> for CInstantTimeValue {
    fn as_rust(&self) -> Fallible<InstantTimeValue> {
        Ok(InstantTimeValue {
            value: create_rust_string_from!(self.value),
            grain: self.grain.as_rust()?,
            precision: self.precision.as_rust()?,
        })
    }
}

impl Drop for CInstantTimeValue {
    fn drop(&mut self) {
        take_back_c_string!(self.value);
    }
}

/// Representation of an interval value
#[repr(C)]
#[derive(Debug)]
pub struct CTimeIntervalValue {
    /// String representation of the beginning of the interval
    pub from: *const libc::c_char,
    /// String representation of the end of the interval
    pub to: *const libc::c_char,
}

impl From<TimeIntervalValue> for CTimeIntervalValue {
    fn from(value: TimeIntervalValue) -> Self {
        Self {
            from: if let Some(s) = value.from {
                CString::new(s).unwrap().into_raw()
            } else {
                null()
            },
            to: if let Some(s) = value.to {
                CString::new(s).unwrap().into_raw()
            } else {
                null()
            },
        }
    }
}

impl AsRust<TimeIntervalValue> for CTimeIntervalValue {
    fn as_rust(&self) -> Fallible<TimeIntervalValue> {
        Ok(TimeIntervalValue {
            from: create_optional_rust_string_from!(self.from),
            to: create_optional_rust_string_from!(self.to),
        })
    }
}

impl Drop for CTimeIntervalValue {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.from);
        take_back_nullable_c_string!(self.to);
    }
}

/// Representation of an amount of money value
#[repr(C)]
#[derive(Debug)]
pub struct CAmountOfMoneyValue {
    /// The currency
    pub unit: *const libc::c_char,
    /// The amount of money
    pub value: libc::c_float,
    /// The precision of the resolved value
    pub precision: SNIPS_PRECISION,
}

impl From<AmountOfMoneyValue> for CAmountOfMoneyValue {
    fn from(value: AmountOfMoneyValue) -> Self {
        Self {
            value: value.value as libc::c_float,
            precision: SNIPS_PRECISION::from(value.precision),
            unit: if let Some(s) = value.unit {
                CString::new(s).unwrap().into_raw()
            } else {
                null()
            },
        }
    }
}

impl AsRust<AmountOfMoneyValue> for CAmountOfMoneyValue {
    fn as_rust(&self) -> Fallible<AmountOfMoneyValue> {
        Ok(AmountOfMoneyValue {
            value: self.value as f32,
            precision: self.precision.as_rust()?,
            unit: create_optional_rust_string_from!(self.unit),
        })
    }
}

impl Drop for CAmountOfMoneyValue {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.unit)
    }
}

/// Representation of a temperature value
#[repr(C)]
#[derive(Debug)]
pub struct CTemperatureValue {
    /// The unit used
    pub unit: *const libc::c_char,
    /// The temperature resolved
    pub value: libc::c_float,
}

impl From<TemperatureValue> for CTemperatureValue {
    fn from(value: TemperatureValue) -> Self {
        Self {
            value: value.value as libc::c_float,
            unit: if let Some(s) = value.unit {
                CString::new(s).unwrap().into_raw()
            } else {
                null()
            },
        }
    }
}

impl AsRust<TemperatureValue> for CTemperatureValue {
    fn as_rust(&self) -> Fallible<TemperatureValue> {
        Ok(TemperatureValue {
            value: self.value as f32,
            unit: create_optional_rust_string_from!(self.unit),
        })
    }
}

impl Drop for CTemperatureValue {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.unit);
    }
}

/// Representation of a duration value
#[repr(C)]
#[derive(Debug)]
pub struct CDurationValue {
    /// Number of years in the duration
    pub years: libc::int64_t,
    /// Number of quarters in the duration
    pub quarters: libc::int64_t,
    /// Number of months in the duration
    pub months: libc::int64_t,
    /// Number of weeks in the duration
    pub weeks: libc::int64_t,
    /// Number of days in the duration
    pub days: libc::int64_t,
    /// Number of hours in the duration
    pub hours: libc::int64_t,
    /// Number of minutes in the duration
    pub minutes: libc::int64_t,
    /// Number of seconds in the duration
    pub seconds: libc::int64_t,
    /// Precision of the resolved value
    pub precision: SNIPS_PRECISION,
}

impl From<DurationValue> for CDurationValue {
    fn from(value: DurationValue) -> Self {
        Self {
            years: value.years as libc::int64_t,
            quarters: value.quarters as libc::int64_t,
            months: value.months as libc::int64_t,
            weeks: value.weeks as libc::int64_t,
            days: value.days as libc::int64_t,
            hours: value.hours as libc::int64_t,
            minutes: value.minutes as libc::int64_t,
            seconds: value.seconds as libc::int64_t,
            precision: SNIPS_PRECISION::from(value.precision),
        }
    }
}

impl AsRust<DurationValue> for CDurationValue {
    fn as_rust(&self) -> Fallible<DurationValue> {
        Ok(DurationValue {
            years: self.years as i64,
            quarters: self.quarters as i64,
            months: self.months as i64,
            weeks: self.weeks as i64,
            days: self.days as i64,
            hours: self.hours as i64,
            minutes: self.minutes as i64,
            seconds: self.seconds as i64,
            precision: self.precision.as_rust()?,
        })
    }
}

/// A slot value
#[repr(C)]
#[derive(Debug)]
pub struct CSlotValue {
    /// Points to either a *const char, a CNumberValue, a COrdinalValue,
    /// a CInstantTimeValue, a CTimeIntervalValue, a CAmountOfMoneyValue,
    /// a CTemperatureValue or a CDurationValue depending on value_type
    value: *const libc::c_void,
    /// The type of the value
    value_type: SNIPS_SLOT_VALUE_TYPE,
}

impl From<SlotValue> for CSlotValue {
    fn from(slot_value: SlotValue) -> Self {
        let value_type = SNIPS_SLOT_VALUE_TYPE::from(&slot_value);
        let value: *const libc::c_void = match slot_value {
            SlotValue::Custom(v) => CString::new(v.value).unwrap().into_raw() as _,
            SlotValue::Number(v) => (v.value as CNumberValue).into_raw_pointer() as _,
            SlotValue::Ordinal(v) => (v.value as COrdinalValue).into_raw_pointer() as _,
            SlotValue::InstantTime(v) => CInstantTimeValue::from(v).into_raw_pointer() as _,
            SlotValue::TimeInterval(v) => CTimeIntervalValue::from(v).into_raw_pointer() as _,
            SlotValue::AmountOfMoney(v) => CAmountOfMoneyValue::from(v).into_raw_pointer() as _,
            SlotValue::Temperature(v) => CTemperatureValue::from(v).into_raw_pointer() as _,
            SlotValue::Duration(v) => CDurationValue::from(v).into_raw_pointer() as _,
            SlotValue::Percentage(v) => (v.value as CPercentageValue).into_raw_pointer() as _,
            SlotValue::MusicAlbum(v) => CString::new(v.value).unwrap().into_raw() as _,
            SlotValue::MusicArtist(v) => CString::new(v.value).unwrap().into_raw() as _,
            SlotValue::MusicTrack(v) => CString::new(v.value).unwrap().into_raw() as _,
        };
        Self { value_type, value }
    }
}

impl AsRust<SlotValue> for CSlotValue {
    fn as_rust(&self) -> Fallible<SlotValue> {
        match self.value_type {
            SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_CUSTOM => Ok(SlotValue::Custom(
                create_rust_string_from!(self.value as *const libc::c_char).into(),
            )),
            SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_NUMBER => {
                let number_value: f64 = unsafe { *(self.value as *const CNumberValue) };
                Ok(SlotValue::Number(NumberValue {
                    value: number_value,
                }))
            }
            SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_ORDINAL => {
                let ordinal_value: i64 = unsafe { *(self.value as *const COrdinalValue) };
                Ok(SlotValue::Ordinal(OrdinalValue {
                    value: ordinal_value,
                }))
            }
            SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_INSTANTTIME => {
                let c_instant_time_value = unsafe { &*(self.value as *const CInstantTimeValue) };
                let instant_time_value = c_instant_time_value.as_rust()?;
                Ok(SlotValue::InstantTime(instant_time_value))
            }
            SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_TIMEINTERVAL => {
                let c_time_interval_value = unsafe { &*(self.value as *const CTimeIntervalValue) };
                let time_interval_value = c_time_interval_value.as_rust()?;
                Ok(SlotValue::TimeInterval(time_interval_value))
            }
            SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_AMOUNTOFMONEY => {
                let c_amount_of_money_value =
                    unsafe { &*(self.value as *const CAmountOfMoneyValue) };
                let amount_of_money_value = c_amount_of_money_value.as_rust()?;
                Ok(SlotValue::AmountOfMoney(amount_of_money_value))
            }
            SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_TEMPERATURE => {
                let c_temperature_value = unsafe { &*(self.value as *const CTemperatureValue) };
                let temperature_value = c_temperature_value.as_rust()?;
                Ok(SlotValue::Temperature(temperature_value))
            }
            SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_DURATION => {
                let c_duration_value = unsafe { &*(self.value as *const CDurationValue) };
                let duration_value = c_duration_value.as_rust()?;
                Ok(SlotValue::Duration(duration_value))
            }
            SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_MUSICALBUM => Ok(SlotValue::MusicAlbum(
                create_rust_string_from!(self.value as *const libc::c_char).into(),
            )),
            SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_MUSICARTIST => Ok(SlotValue::MusicArtist(
                create_rust_string_from!(self.value as *const libc::c_char).into(),
            )),
            SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_MUSICTRACK => Ok(SlotValue::MusicTrack(
                create_rust_string_from!(self.value as *const libc::c_char).into(),
            )),
            _ => bail!(
                "Unknown slot value type: {:?}. Cannot perform conversion to Rust object.",
                self.value_type
            ),
        }
    }
}

impl Drop for CSlotValue {
    fn drop(&mut self) {
        let _ = unsafe {
            match self.value_type {
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_CUSTOM => {
                    CString::drop_raw_pointer(self.value)
                }
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_NUMBER => {
                    CNumberValue::drop_raw_pointer(self.value as _)
                }
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_ORDINAL => {
                    COrdinalValue::drop_raw_pointer(self.value as _)
                }
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_INSTANTTIME => {
                    CInstantTimeValue::drop_raw_pointer(self.value as _)
                }
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_TIMEINTERVAL => {
                    CTimeIntervalValue::drop_raw_pointer(self.value as _)
                }
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_AMOUNTOFMONEY => {
                    CAmountOfMoneyValue::drop_raw_pointer(self.value as _)
                }
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_TEMPERATURE => {
                    CTemperatureValue::drop_raw_pointer(self.value as _)
                }
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_DURATION => {
                    CDurationValue::drop_raw_pointer(self.value as _)
                }
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_PERCENTAGE => {
                    CPercentageValue::drop_raw_pointer(self.value as _)
                }
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_MUSICALBUM => {
                    CString::drop_raw_pointer(self.value)
                }
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_MUSICARTIST => {
                    CString::drop_raw_pointer(self.value)
                }
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_MUSICTRACK => {
                    CString::drop_raw_pointer(self.value)
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn round_trip_test<T, U>(input: T)
    where
        T: Clone + PartialEq + std::fmt::Debug,
        U: From<T> + AsRust<T>,
    {
        let c = U::from(input.clone());

        let result = c.as_rust().expect("could not convert back to rust");
        assert_eq!(result, input);
    }

    #[test]
    fn round_trip_c_duration_value() {
        round_trip_test::<_, CDurationValue>(DurationValue {
            years: 1,
            quarters: 2,
            months: 3,
            weeks: 4,
            days: 5,
            hours: 6,
            minutes: 7,
            seconds: 8,
            precision: Precision::Approximate,
        })
    }

    #[test]
    fn round_trip_c_temperature_value() {
        round_trip_test::<_, CTemperatureValue>(TemperatureValue {
            value: 20.0,
            unit: Some("°C".to_string()),
        })
    }

    #[test]
    fn round_trip_c_amount_of_value() {
        round_trip_test::<_, CAmountOfMoneyValue>(AmountOfMoneyValue {
            value: 1234.0,
            precision: Precision::Exact,
            unit: Some("€".to_string()),
        })
    }

    #[test]
    fn round_trip_c_time_interval_value() {
        round_trip_test::<_, CTimeIntervalValue>(TimeIntervalValue {
            from: Some("from".to_string()),
            to: Some("to".to_string()),
        })
    }

    #[test]
    fn round_trip_c_instant_time_value() {
        round_trip_test::<_, CInstantTimeValue>(InstantTimeValue {
            value: "value".to_string(),
            grain: Grain::Year,
            precision: Precision::Approximate,
        })
    }

    #[test]
    fn round_trip_snips_grain() {
        round_trip_test::<_, SNIPS_GRAIN>(Grain::Second)
    }

    #[test]
    fn round_trip_snips_precision() {
        round_trip_test::<_, SNIPS_PRECISION>(Precision::Approximate)
    }

    #[test]
    fn round_trip_c_slot() {
        round_trip_test::<_, CSlot>(Slot {
            raw_value: "raw_value".to_string(),
            value: SlotValue::Custom("slot_value".to_string().into()),
            range: 0..1,
            entity: "entity".to_string(),
            slot_name: "slot_name".to_string(),
            confidence_score: Some(0.5),
        });

        round_trip_test::<_, CSlot>(Slot {
            raw_value: "raw_value".to_string(),
            value: SlotValue::Custom("slot_value".to_string().into()),
            range: 0..1,
            entity: "entity".to_string(),
            slot_name: "slot_name".to_string(),
            confidence_score: None,
        });

        let instant_time_value = InstantTimeValue {
            value: "value".to_string(),
            grain: Grain::Year,
            precision: Precision::Approximate,
        };
        round_trip_test::<_, CSlot>(Slot {
            raw_value: "raw_value".to_string(),
            value: SlotValue::InstantTime(instant_time_value),
            range: 0..1,
            entity: "entity".to_string(),
            slot_name: "slot_name".to_string(),
            confidence_score: Some(0.5),
        });

        let instant_time_value = TimeIntervalValue {
            from: Some("lol".to_string()),
            to: Some("lol".to_string()),
        };
        round_trip_test::<_, CSlot>(Slot {
            raw_value: "raw_value".to_string(),
            value: SlotValue::TimeInterval(instant_time_value),
            range: 0..1,
            entity: "entity".to_string(),
            slot_name: "slot_name".to_string(),
            confidence_score: Some(0.5),
        });
    }

    #[test]
    fn round_trip_c_slot_list() {
        let temperature_value = TemperatureValue {
            value: 20.0,
            unit: Some("°C".to_string()),
        };

        round_trip_test::<_, CSlotList>(vec![
            Slot {
                raw_value: "raw_value_slot".to_string(),
                value: SlotValue::Custom("custom_value".to_string().into()),
                range: 0..42,
                entity: "entity".to_string(),
                slot_name: "slot_name".to_string(),
                confidence_score: Some(1.0),
            },
            Slot {
                raw_value: "".to_string(),
                value: SlotValue::Temperature(temperature_value),
                range: (0..42),
                entity: "entity".to_string(),
                slot_name: "slot_name".to_string(),
                confidence_score: Some(0.5),
            },
        ])
    }

    #[test]
    fn round_trip_c_intent_classifier_result() {
        round_trip_test::<_, CIntentClassifierResult>(IntentClassifierResult {
            intent_name: Some("intent_name".to_string()),
            confidence_score: 0.5,
        })
    }

    #[test]
    fn round_trip_c_intent_classifier_result_array() {
        round_trip_test::<_, CIntentClassifierResultArray>(vec![
            IntentClassifierResult {
                intent_name: Some("intent_name".to_string()),
                confidence_score: 0.5,
            },
            IntentClassifierResult {
                intent_name: None,
                confidence_score: 0.5,
            },
        ])
    }

    #[test]
    fn round_trip_c_intent_parser_result() {
        round_trip_test::<_, CIntentParserResult>(IntentParserResult {
            input: "input".to_string(),
            intent: IntentClassifierResult {
                intent_name: Some("intent_name".to_string()),
                confidence_score: 0.5,
            },
            slots: vec![Slot {
                raw_value: "raw_value".to_string(),
                value: SlotValue::Custom(StringValue {
                    value: "custom_slot".to_string(),
                }),
                range: 0..42,
                entity: "entity".to_string(),
                slot_name: "slot_name".to_string(),
                confidence_score: Some(1.0),
            }],
        })
    }
}
