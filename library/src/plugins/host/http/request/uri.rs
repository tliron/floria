use super::errors::*;

use {
    hyper::Request,
    rustls::pki_types::*,
    std::result::Result,
    tokio_rustls::*,
    wasmtime_wasi_http::{bindings::http::types::*, body::*},
};

/// Authority.
pub fn authority(request: &Request<HyperOutgoingBody>, use_tls: bool) -> Result<String, ErrorCode> {
    if let Some(authority) = request.uri().authority() {
        Ok(if authority.port().is_some() {
            authority.to_string()
        } else {
            let port = if use_tls { 443 } else { 80 };
            format!("{}:{}", authority.to_string(), port)
        })
    } else {
        Err(ErrorCode::HttpRequestUriInvalid)
    }
}

/// Domain.
pub fn domain(authority: &str) -> Result<ServerName<'static>, ErrorCode> {
    let host = authority.split_once(":").map(|(host, _)| host).unwrap_or(&authority);
    Ok(ServerName::try_from(host)
        .map_err(|error| {
            tracing::warn!("DNS lookup error: {error:?}");
            dns_error("invalid DNS name".to_string(), 0)
        })?
        .to_owned())
}
