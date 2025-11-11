use super::errors::*;

use {
    std::{io, result::Result, time::Duration},
    tokio::{net::*, time::*},
    wasmtime_wasi_http::bindings::http::types::*,
};

/// Connect TCP.
pub async fn connect_tcp(authority: &str, connect_timeout: Duration) -> Result<TcpStream, ErrorCode> {
    timeout(connect_timeout, TcpStream::connect(&authority)).await.map_err(|_| ErrorCode::ConnectionTimeout)?.map_err(
        |error| match error.kind() {
            io::ErrorKind::AddrNotAvailable => dns_error("address not available".into(), 0),

            _ => {
                // TODO: ugh, must be a better way
                if error.to_string().starts_with("failed to lookup address information") {
                    dns_error("address not available".into(), 0)
                } else {
                    ErrorCode::ConnectionRefused
                }
            }
        },
    )
}
