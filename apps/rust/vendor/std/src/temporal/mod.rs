#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
use crate::json::JsonProducerTrait;
mod support;
pub use support::*;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            DAYS_IN_MONTH.set(std::sync::Arc::new(vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31])).unwrap_or_else(| _ | panic!());
            DAY_OF_WEEK_LOOKUP_TABLE_LEAPY.set(std::sync::Arc::new(vec![0, 0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6])).unwrap_or_else(| _ | panic!());
            DAY_OF_WEEK_LOOKUP_TABLE_NOT_LEAPY.set(std::sync::Arc::new(vec![0, 0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5])).unwrap_or_else(| _ | panic!());
            Ok(())
    }).clone()
}
static DAYS_IN_MONTH: std::sync::OnceLock<temper_core::List<i32>> = std::sync::OnceLock::new();
fn days_in_month() -> temper_core::List<i32> {
    ( * DAYS_IN_MONTH.get().unwrap()).clone()
}
static DAY_OF_WEEK_LOOKUP_TABLE_LEAPY: std::sync::OnceLock<temper_core::List<i32>> = std::sync::OnceLock::new();
fn day_of_week_lookup_table_leapy() -> temper_core::List<i32> {
    ( * DAY_OF_WEEK_LOOKUP_TABLE_LEAPY.get().unwrap()).clone()
}
static DAY_OF_WEEK_LOOKUP_TABLE_NOT_LEAPY: std::sync::OnceLock<temper_core::List<i32>> = std::sync::OnceLock::new();
fn day_of_week_lookup_table_not_leapy() -> temper_core::List<i32> {
    ( * DAY_OF_WEEK_LOOKUP_TABLE_NOT_LEAPY.get().unwrap()).clone()
}
struct DateJsonAdapterStruct {}
#[derive(Clone)]
pub (crate) struct DateJsonAdapter(std::sync::Arc<DateJsonAdapterStruct>);
impl DateJsonAdapter {
    pub fn encode_to_json(& self, x__116: Date, p__117: crate::json::JsonProducer) {
        x__116.encode_to_json(p__117.clone());
    }
    pub fn decode_from_json(& self, t__118: crate::json::JsonSyntaxTree, ic__119: crate::json::InterchangeContext) -> temper_core::Result<Date> {
        let return__129: Date;
        return__129 = Date::decode_from_json(t__118.clone(), ic__119.clone()) ? ;
        return Ok(return__129.clone());
    }
    pub fn new() -> DateJsonAdapter {
        let selfish = DateJsonAdapter(std::sync::Arc::new(DateJsonAdapterStruct {}));
        return selfish;
    }
}
impl crate::json::JsonAdapterTrait<Date> for DateJsonAdapter {
    fn clone_boxed(& self) -> crate::json::JsonAdapter<Date> {
        crate::json::JsonAdapter::new(self.clone())
    }
    fn encode_to_json(& self, x__116: Date, p__117: crate::json::JsonProducer) {
        self.encode_to_json(x__116, p__117)
    }
    fn decode_from_json(& self, t__118: crate::json::JsonSyntaxTree, ic__119: crate::json::InterchangeContext) -> temper_core::Result<Date> {
        self.decode_from_json(t__118, ic__119)
    }
}
temper_core::impl_any_value_trait!(DateJsonAdapter, [crate::json::JsonAdapter<Date>]);
struct DateStruct {
    year: i32, month: i32, day: i32
}
#[derive(Clone)]
pub struct Date(std::sync::Arc<DateStruct>);
#[derive(Clone)]
pub struct DateBuilder {
    pub year: i32, pub month: i32, pub day: i32
}
impl DateBuilder {
    pub fn build(self) -> temper_core::Result<Date> {
        Date::new(self.year, self.month, self.day)
    }
}
impl Date {
    pub fn new(year__55: i32, month__56: i32, day__57: i32) -> temper_core::Result<Date> {
        let year;
        let month;
        let day;
        let mut t___343: i32;
        let mut t___243: bool;
        let mut t___245: bool;
        let mut t___246: bool;
        let mut t___247: bool;
        let mut t___248: bool;
        if Some(1) <= Some(month__56) {
            if Some(month__56) <= Some(12) {
                if Some(1) <= Some(day__57) {
                    if Some(month__56) != Some(2) {
                        t___243 = true;
                    } else {
                        t___243 = Some(day__57) != Some(29);
                    }
                    if t___243 {
                        t___343 = temper_core::ListedTrait::get( & days_in_month(), month__56);
                        t___245 = Some(day__57) <= Some(t___343);
                    } else {
                        t___245 = isLeapYear__32(year__55);
                    }
                    t___246 = t___245;
                } else {
                    t___246 = false;
                }
                t___247 = t___246;
            } else {
                t___247 = false;
            }
            t___248 = t___247;
        } else {
            t___248 = false;
        }
        if t___248 {
            year = year__55;
            month = month__56;
            day = day__57;
        } else {
            return Err(temper_core::Error::new());
        }
        let selfish = Date(std::sync::Arc::new(DateStruct {
                    year, month, day
        }));
        return Ok(selfish);
    }
    pub fn to_string(& self) -> std::sync::Arc<String> {
        let sb__61: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        padTo__33(4, self.0.year, sb__61.clone());
        temper_core::string::builder::append( & sb__61, "-");
        padTo__33(2, self.0.month, sb__61.clone());
        temper_core::string::builder::append( & sb__61, "-");
        padTo__33(2, self.0.day, sb__61.clone());
        return temper_core::string::builder::to_string( & sb__61);
    }
    pub fn from_iso_string(isoString__63: impl temper_core::ToArcString) -> temper_core::Result<Date> {
        let isoString__63 = isoString__63.to_arc_string();
        let return__26: Date;
        let mut t___327: i32;
        let mut t___332: bool;
        let mut t___222: bool;
        let mut t___228: bool;
        let end__65: usize = isoString__63.len();
        let mut t___323: usize = temper_core::string::prev( & isoString__63, end__65);
        let mut t___324: usize = temper_core::string::prev( & isoString__63, t___323);
        let mut strIndex__66: usize = t___324;
        let beforeDay__67: usize = strIndex__66;
        let mut t___325: usize = temper_core::string::prev( & isoString__63, strIndex__66);
        strIndex__66 = t___325;
        let afterMonth__68: usize = strIndex__66;
        if ! temper_core::string::has_index( & isoString__63, afterMonth__68) {
            t___222 = true;
        } else {
            t___327 = temper_core::string::get( & isoString__63, strIndex__66);
            t___222 = Some(t___327) != Some(45);
        }
        if t___222 {
            return Err(temper_core::Error::new());
        }
        let mut t___328: usize = temper_core::string::prev( & isoString__63, strIndex__66);
        let mut t___329: usize = temper_core::string::prev( & isoString__63, t___328);
        strIndex__66 = t___329;
        let beforeMonth__69: usize = strIndex__66;
        let mut t___330: usize = temper_core::string::prev( & isoString__63, strIndex__66);
        strIndex__66 = t___330;
        if Some(temper_core::string::get( & isoString__63, strIndex__66)) != Some(45) {
            t___228 = true;
        } else {
            t___332 = temper_core::string::has_at_least( & isoString__63, 0usize, strIndex__66, 4);
            t___228 = ! t___332;
        }
        if t___228 {
            return Err(temper_core::Error::new());
        }
        let day__70: i32;
        day__70 = temper_core::string::to_int( & temper_core::string::slice( & isoString__63, beforeDay__67, end__65), Some(10)) ? ;
        let month__71: i32;
        month__71 = temper_core::string::to_int( & temper_core::string::slice( & isoString__63, beforeMonth__69, afterMonth__68), Some(10)) ? ;
        let year__72: i32;
        year__72 = temper_core::string::to_int( & temper_core::string::slice( & isoString__63, 0usize, strIndex__66), Some(10)) ? ;
        return__26 = Date::new(year__72, month__71, day__70) ? ;
        return Ok(return__26.clone());
    }
    pub fn years_between(start__74: Date, end__75: Date) -> i32 {
        let mut t___320: i32;
        let mut t___321: i32;
        let mut t___213: bool;
        let mut t___214: bool;
        let mut t___215: i32;
        let yearDelta__77: i32 = end__75.year().wrapping_sub(start__74.year());
        let monthDelta__78: i32 = end__75.month().wrapping_sub(start__74.month());
        if Some(monthDelta__78) < Some(0) {
            t___214 = true;
        } else {
            if Some(monthDelta__78) == Some(0) {
                t___320 = end__75.day();
                t___321 = start__74.day();
                t___213 = Some(t___320) < Some(t___321);
            } else {
                t___213 = false;
            }
            t___214 = t___213;
        }
        if t___214 {
            t___215 = 1;
        } else {
            t___215 = 0;
        }
        return yearDelta__77.wrapping_sub(t___215);
    }
    pub fn day_of_week(& self) -> i32 {
        let return__29: i32;
        let mut t___196: i32;
        let mut t___197: i32;
        let y__83: i32 = self.0.year;
        let c__84: i32;
        if Some(y__83) >= Some(0) {
            t___196 = y__83.wrapping_div(100);
            c__84 = t___196;
        } else {
            t___197 = y__83.wrapping_neg().wrapping_div(100);
            c__84 = t___197.wrapping_neg();
        }
        let yy__85: i32 = y__83.wrapping_sub(c__84.wrapping_mul(100));
        let janFirst__86: i32 = (8 as i32).wrapping_add((5 as i32).wrapping_mul(yy__85.wrapping_add(3).wrapping_rem(4))).wrapping_add((3 as i32).wrapping_mul(yy__85.wrapping_sub(1))).wrapping_add((5 as i32).wrapping_mul(c__84.wrapping_rem(4))).wrapping_rem(7);
        let table__87: temper_core::List<i32>;
        if isLeapYear__32(y__83) {
            table__87 = day_of_week_lookup_table_leapy().clone();
        } else {
            table__87 = day_of_week_lookup_table_not_leapy().clone();
        }
        let monthOffset__88: i32 = temper_core::ListedTrait::get( & table__87, self.0.month);
        let gaussWeekday__89: i32 = janFirst__86.wrapping_add(self.0.day.wrapping_add(6)).wrapping_add(monthOffset__88).wrapping_rem(7);
        if Some(gaussWeekday__89) == Some(0) {
            return__29 = 7;
        } else {
            return__29 = gaussWeekday__89;
        }
        return return__29;
    }
    pub fn encode_to_json(& self, p__91: crate::json::JsonProducer) {
        let mut t___313: std::sync::Arc<String> = self.to_string();
        p__91.string_value(t___313.clone());
    }
    pub fn decode_from_json(t__94: crate::json::JsonSyntaxTree, ic__95: crate::json::InterchangeContext) -> temper_core::Result<Date> {
        let return__31: Date;
        let mut t___190: crate::json::JsonString;
        t___190 = temper_core::cast::<crate::json::JsonString>(t__94.clone()).ok_or_else(| | temper_core::Error::new()) ? ;
        return__31 = Date::from_iso_string(t___190.content()) ? ;
        return Ok(return__31.clone());
    }
    pub fn year(& self) -> i32 {
        return self.0.year;
    }
    pub fn month(& self) -> i32 {
        return self.0.month;
    }
    pub fn day(& self) -> i32 {
        return self.0.day;
    }
    pub fn json_adapter() -> crate::json::JsonAdapter<Date> {
        return crate::json::JsonAdapter::new(DateJsonAdapter::new());
    }
}
temper_core::impl_any_value_trait!(Date, []);
fn isLeapYear__32(year__41: i32) -> bool {
    let return__21: bool;
    let mut t___263: i32;
    if Some(year__41.wrapping_rem(4)) == Some(0) {
        if Some(year__41.wrapping_rem(100)) != Some(0) {
            return__21 = true;
        } else {
            t___263 = year__41.wrapping_rem(400);
            return__21 = Some(t___263) == Some(0);
        }
    } else {
        return__21 = false;
    }
    return return__21;
}
fn padTo__33(minWidth__43: i32, num__44: i32, sb__45: std::sync::Arc<std::sync::RwLock<String>>) {
    let mut t___346: i32;
    let mut t___348: usize;
    let mut t___257: bool;
    let decimal__47: std::sync::Arc<String> = temper_core::int_to_string(num__44, Some(10));
    let mut decimalIndex__48: usize = 0usize;
    let decimalEnd__49: usize = decimal__47.len();
    if Some(decimalIndex__48) < Some(decimalEnd__49) {
        t___346 = temper_core::string::get( & decimal__47, decimalIndex__48);
        t___257 = Some(t___346) == Some(45);
    } else {
        t___257 = false;
    }
    if t___257 {
        temper_core::string::builder::append( & sb__45, "-");
        t___348 = temper_core::string::next( & decimal__47, decimalIndex__48);
        decimalIndex__48 = t___348;
    }
    let mut t___349: i32 = temper_core::string::count_between( & decimal__47, decimalIndex__48, decimalEnd__49);
    let mut nNeeded__50: i32 = minWidth__43.wrapping_sub(t___349);
    'loop___2663: while Some(nNeeded__50) > Some(0) {
        temper_core::string::builder::append( & sb__45, "0");
        nNeeded__50 = nNeeded__50.wrapping_sub(1);
    }
    temper_core::string::builder::append_between( & sb__45, decimal__47.clone(), decimalIndex__48, decimalEnd__49);
}
