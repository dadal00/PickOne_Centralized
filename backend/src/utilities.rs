pub fn convert_i8_to_u8(payload: &i8) -> u8 {
    payload.checked_abs().unwrap_or(0) as u8
}

pub fn convert_i16_to_u16(payload: &i16) -> u16 {
    payload.checked_abs().unwrap_or(0) as u16
}

pub fn convert_i64_to_u64(payload: &i64) -> u64 {
    payload.checked_abs().unwrap_or(0) as u64
}
