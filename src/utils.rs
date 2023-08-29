use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use starknet::core::types::FieldElement;
use std::fmt::Write;

#[macro_export]
macro_rules! pub_struct {
    ($($derive:path),*; $name:ident {$($field:ident: $t:ty),* $(,)?}) => {
        #[derive($($derive),*)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub fn get_error(error: String) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, error).into_response()
}

pub fn get_specific_error(code: StatusCode, error: String) -> Response {
    (code, error).into_response()
}

pub fn to_hex(felt: FieldElement) -> String {
    let bytes = felt.to_bytes_be();
    let mut result = String::with_capacity(bytes.len() * 2 + 2);
    result.push_str("0x");
    for byte in bytes {
        write!(&mut result, "{:02x}", byte).unwrap();
    }
    result
}
