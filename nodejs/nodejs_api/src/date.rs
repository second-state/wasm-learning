use wasm_bindgen::prelude::*;

#[wasm_bindgen(module="/node/date-helper.js")]
extern "C" {
    pub fn timestamp_unix() -> i32;
    pub fn timestamp() -> String;
    pub fn utc_string() -> String;
    pub fn format_date(locale: &str, weekday: &str, year: &str, month: &str, day: &str, time_zone: &str, time_zone_name: &str) -> String;
}
