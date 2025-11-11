use wasmtime_wasi_http::bindings::http::types::*;

/// DNS error.
pub fn dns_error(rcode: String, info_code: u16) -> ErrorCode {
    ErrorCode::DnsError(DnsErrorPayload { rcode: Some(rcode), info_code: Some(info_code) })
}
