use super::core::BASE_DATE;
use crate::utilities::{convert_i8_to_u8, convert_i16_to_u16, convert_i64_to_u64};
use chrono::Duration as chronoDuration;
use scylla_cdc::consumer::CDCRow;
use uuid::Uuid;

pub fn get_cdc_id(data: &CDCRow<'_>, id_name: &str) -> Uuid {
    data.get_value(id_name)
        .as_ref()
        .and_then(|v| v.as_uuid())
        .expect("Missing item id")
}

pub fn get_cdc_tinyint(data: &CDCRow<'_>, column: &str) -> i8 {
    data.get_value(column)
        .as_ref()
        .and_then(|v| v.as_tinyint())
        .expect("Missing tinyint attribute")
}

pub fn get_cdc_smallint(data: &CDCRow<'_>, column: &str) -> i16 {
    data.get_value(column)
        .as_ref()
        .and_then(|v| v.as_smallint())
        .expect("Missing smallint attribute")
}

pub fn get_cdc_counter(data: &CDCRow<'_>, column: &str) -> i64 {
    data.get_value(column)
        .as_ref()
        .and_then(|v| v.as_counter())
        .map(|c| c.0)
        .expect("Missing counter attribute")
}

pub fn get_cdc_u8(data: &CDCRow<'_>, column: &str) -> u8 {
    convert_i8_to_u8(&get_cdc_tinyint(data, column))
}

pub fn get_cdc_u16(data: &CDCRow<'_>, column: &str) -> u16 {
    convert_i16_to_u16(&get_cdc_smallint(data, column))
}

pub fn get_cdc_u64(data: &CDCRow<'_>, column: &str) -> u64 {
    convert_i64_to_u64(&get_cdc_counter(data, column))
}

pub fn get_cdc_text(data: &CDCRow<'_>, column: &str) -> String {
    data.get_value(column)
        .as_ref()
        .and_then(|v| v.as_text())
        .expect("Missing text attribute")
        .to_string()
}

pub fn get_cdc_date(data: &CDCRow<'_>, column: &str) -> String {
    let days = data
        .get_value(column)
        .as_ref()
        .and_then(|v| v.as_cql_date())
        .expect("Missing date attribute")
        .0 as i64;

    BASE_DATE
        .checked_add_signed(chronoDuration::days(days - 2_147_483_648))
        .map(|d| d.format("%Y-%m-%d").to_string())
        .expect("Missing the date attribute!")
}
