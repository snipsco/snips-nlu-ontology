#![allow(non_camel_case_types)]

use std::ffi::CString;
use std::ptr::null;
use std::slice;

use libc;

use ffi_utils::RawPointerConverter;

#[repr(C)]
#[derive(Debug)]
pub struct CIntentParserResult {
    pub input: *const libc::c_char,
    pub intent: *const CIntentClassifierResult,
    pub slots: *const CSlotList,
}

impl From<::IntentParserResult> for CIntentParserResult {
    fn from(input: ::IntentParserResult) -> Self {
        Self {
            input: CString::new(input.input).unwrap().into_raw(),
            intent: if let Some(intent) = input.intent {
                CIntentClassifierResult::from(intent).into_raw_pointer()
            } else {
                null()
            },
            slots: if let Some(slots) = input.slots {
                CSlotList::from(slots).into_raw_pointer()
            } else {
                null()
            },
        }
    }
}

impl Drop for CIntentParserResult {
    fn drop(&mut self) {
        take_back_c_string!(self.input);
        let _ = unsafe { CIntentClassifierResult::from_raw_pointer(self.intent) };
        let _ = unsafe { CSlotList::from_raw_pointer(self.slots) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CIntentClassifierResult {
    pub intent_name: *const libc::c_char,
    pub probability: libc::c_float,
}

impl From<::IntentClassifierResult> for CIntentClassifierResult {
    fn from(input: ::IntentClassifierResult) -> Self {
        Self {
            probability: input.probability,
            intent_name: CString::new(input.intent_name).unwrap().into_raw(), // String can not contains 0
        }
    }
}

impl Drop for CIntentClassifierResult {
    fn drop(&mut self) {
        take_back_c_string!(self.intent_name);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CSlotList {
    pub slots: *const CSlot,
    pub size: libc::int32_t, // Note: we can't use `libc::size_t` because it's not supported by JNA
}

impl From<Vec<::Slot>> for CSlotList {
    fn from(input: Vec<::Slot>) -> Self {
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

#[repr(C)]
#[derive(Debug)]
pub struct CSlot {
    pub value: CSlotValue,
    pub raw_value: *const libc::c_char,
    pub entity: *const libc::c_char,
    pub slot_name: *const libc::c_char,
    pub range_start: libc::int32_t,
    pub range_end: libc::int32_t,
}

impl From<::Slot> for CSlot {
    fn from(input: ::Slot) -> Self {
        let range = if let Some(range) = input.range {
            range.start as libc::int32_t..range.end as libc::int32_t
        } else {
            -1..-1
        };

        Self {
            raw_value: CString::new(input.raw_value).unwrap().into_raw(),
            value: CSlotValue::from(input.value),
            range_start: range.start,
            range_end: range.end,
            entity: CString::new(input.entity).unwrap().into_raw(),
            slot_name: CString::new(input.slot_name).unwrap().into_raw(),
        }
    }
}

impl Drop for CSlot {
    fn drop(&mut self) {
        take_back_c_string!(self.raw_value);
        take_back_c_string!(self.entity);
        take_back_c_string!(self.slot_name);
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum SNIPS_SLOT_VALUE_TYPE {
    SNIPS_SLOT_VALUE_TYPE_CUSTOM = 1,
    SNIPS_SLOT_VALUE_TYPE_NUMBER = 2,
    SNIPS_SLOT_VALUE_TYPE_ORDINAL = 3,
    SNIPS_SLOT_VALUE_TYPE_INSTANTTIME = 4,
    SNIPS_SLOT_VALUE_TYPE_TIMEINTERVAL = 5,
    SNIPS_SLOT_VALUE_TYPE_AMOUNTOFMONEY = 6,
    SNIPS_SLOT_VALUE_TYPE_TEMPERATURE = 7,
    SNIPS_SLOT_VALUE_TYPE_DURATION = 8,
    SNIPS_SLOT_VALUE_TYPE_PERCENTAGE = 9,
}

impl<'a> From<&'a ::SlotValue> for SNIPS_SLOT_VALUE_TYPE {
    fn from(slot_value: &::SlotValue) -> Self {
        match slot_value {
            &::SlotValue::Custom(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_CUSTOM,
            &::SlotValue::Number(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_NUMBER,
            &::SlotValue::Ordinal(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_ORDINAL,
            &::SlotValue::InstantTime(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_INSTANTTIME,
            &::SlotValue::TimeInterval(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_TIMEINTERVAL,
            &::SlotValue::AmountOfMoney(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_AMOUNTOFMONEY,
            &::SlotValue::Temperature(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_TEMPERATURE,
            &::SlotValue::Duration(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_DURATION,
            &::SlotValue::Percentage(_) => SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_PERCENTAGE,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum SNIPS_PRECISION {
    SNIPS_PRECISION_APPROXIMATE = 0,
    SNIPS_PRECISION_EXACT = 1,
}

impl From<::Precision> for SNIPS_PRECISION {
    fn from(value: ::Precision) -> Self {
        match value {
            ::Precision::Approximate => SNIPS_PRECISION::SNIPS_PRECISION_APPROXIMATE,
            ::Precision::Exact => SNIPS_PRECISION::SNIPS_PRECISION_EXACT,
        }
    }
}

pub type CNumberValue = libc::c_double;
pub type CPercentageValue = libc::c_double;
pub type COrdinalValue = libc::int64_t;

#[repr(C)]
#[derive(Debug)]
pub enum SNIPS_GRAIN {
    SNIPS_GRAIN_YEAR = 0,
    SNIPS_GRAIN_QUARTER = 1,
    SNIPS_GRAIN_MONTH = 2,
    SNIPS_GRAIN_WEEK = 3,
    SNIPS_GRAIN_DAY = 4,
    SNIPS_GRAIN_HOUR = 5,
    SNIPS_GRAIN_MINUTE = 6,
    SNIPS_GRAIN_SECOND = 7,
}

impl From<::Grain> for SNIPS_GRAIN {
    fn from(value: ::Grain) -> Self {
        match value {
            ::Grain::Year => SNIPS_GRAIN::SNIPS_GRAIN_YEAR,
            ::Grain::Quarter => SNIPS_GRAIN::SNIPS_GRAIN_QUARTER,
            ::Grain::Month => SNIPS_GRAIN::SNIPS_GRAIN_MONTH,
            ::Grain::Week => SNIPS_GRAIN::SNIPS_GRAIN_WEEK,
            ::Grain::Day => SNIPS_GRAIN::SNIPS_GRAIN_DAY,
            ::Grain::Hour => SNIPS_GRAIN::SNIPS_GRAIN_HOUR,
            ::Grain::Minute => SNIPS_GRAIN::SNIPS_GRAIN_MINUTE,
            ::Grain::Second => SNIPS_GRAIN::SNIPS_GRAIN_SECOND,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CInstantTimeValue {
    pub value: *const libc::c_char,
    pub grain: SNIPS_GRAIN,
    pub precision: SNIPS_PRECISION,
}

impl From<::InstantTimeValue> for CInstantTimeValue {
    fn from(value: ::InstantTimeValue) -> Self {
        Self {
            value: CString::new(value.value).unwrap().into_raw(),
            grain: SNIPS_GRAIN::from(value.grain),
            precision: SNIPS_PRECISION::from(value.precision),
        }
    }
}

impl Drop for CInstantTimeValue {
    fn drop(&mut self) {
        take_back_c_string!(self.value);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CTimeIntervalValue {
    pub from: *const libc::c_char,
    pub to: *const libc::c_char,
}

impl From<::TimeIntervalValue> for CTimeIntervalValue {
    fn from(value: ::TimeIntervalValue) -> Self {
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

impl Drop for CTimeIntervalValue {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.from);
        take_back_nullable_c_string!(self.to);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CAmountOfMoneyValue {
    pub unit: *const libc::c_char,
    pub value: libc::c_float,
    pub precision: SNIPS_PRECISION,
}

impl From<::AmountOfMoneyValue> for CAmountOfMoneyValue {
    fn from(value: ::AmountOfMoneyValue) -> Self {
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

impl Drop for CAmountOfMoneyValue {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.unit)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CTemperatureValue {
    pub unit: *const libc::c_char,
    pub value: libc::c_float,
}

impl From<::TemperatureValue> for CTemperatureValue {
    fn from(value: ::TemperatureValue) -> Self {
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

impl Drop for CTemperatureValue {
    fn drop(&mut self) {
        take_back_nullable_c_string!(self.unit);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CDurationValue {
    pub years: libc::int64_t,
    pub quarters: libc::int64_t,
    pub months: libc::int64_t,
    pub weeks: libc::int64_t,
    pub days: libc::int64_t,
    pub hours: libc::int64_t,
    pub minutes: libc::int64_t,
    pub seconds: libc::int64_t,
    pub precision: SNIPS_PRECISION,
}

impl From<::DurationValue> for CDurationValue {
    fn from(value: ::DurationValue) -> Self {
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

#[repr(C)]
#[derive(Debug)]
pub struct CSlotValue {
    /**
     * Points to either a *const char, a CNumberValue, a COrdinalValue,
     * a CInstantTimeValue, a CTimeIntervalValue, a CAmountOfMoneyValue,
     * a CTemperatureValue or a CDurationValue depending on value_type
     */
    value: *const libc::c_void,
    value_type: SNIPS_SLOT_VALUE_TYPE,
}

impl From<::SlotValue> for CSlotValue {
    fn from(slot_value: ::SlotValue) -> Self {
        let value_type = SNIPS_SLOT_VALUE_TYPE::from(&slot_value);
        let value: *const libc::c_void = match slot_value {
            ::SlotValue::Custom(v) => CString::new(v.value).unwrap().into_raw() as _,
            ::SlotValue::Number(v) => (v.value as CNumberValue).into_raw_pointer() as _,
            ::SlotValue::Ordinal(v) => (v.value as COrdinalValue).into_raw_pointer() as _,
            ::SlotValue::InstantTime(v) => CInstantTimeValue::from(v).into_raw_pointer() as _,
            ::SlotValue::TimeInterval(v) => CTimeIntervalValue::from(v).into_raw_pointer() as _,
            ::SlotValue::AmountOfMoney(v) => CAmountOfMoneyValue::from(v).into_raw_pointer() as _,
            ::SlotValue::Temperature(v) => CTemperatureValue::from(v).into_raw_pointer() as _,
            ::SlotValue::Duration(v) => CDurationValue::from(v).into_raw_pointer() as _,
            ::SlotValue::Percentage(v) => (v.value as CPercentageValue).into_raw_pointer() as _,
        };
        Self { value_type, value }
    }
}

impl Drop for CSlotValue {
    fn drop(&mut self) {
        let _ = unsafe {
            match self.value_type {
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_CUSTOM => CString::drop_raw_pointer(self.value),
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_NUMBER => CNumberValue::drop_raw_pointer(self.value as _),
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_ORDINAL => COrdinalValue::drop_raw_pointer(self.value as _),
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_INSTANTTIME => CInstantTimeValue::drop_raw_pointer(self.value as _),
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_TIMEINTERVAL => CTimeIntervalValue::drop_raw_pointer(self.value as _),
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_AMOUNTOFMONEY => CAmountOfMoneyValue::drop_raw_pointer(self.value as _),
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_TEMPERATURE => CTemperatureValue::drop_raw_pointer(self.value as _),
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_DURATION => CDurationValue::drop_raw_pointer(self.value as _),
                SNIPS_SLOT_VALUE_TYPE::SNIPS_SLOT_VALUE_TYPE_PERCENTAGE => CPercentageValue::drop_raw_pointer(self.value as _),
            }
        };
    }
}
