use super::{super::tls::*, http::*, tcp::*, uri::*};

use {
    ::http::*,
    http_body_util::*,
    hyper::Request,
    read_url::*,
    std::{result::Result, sync::*},
    tokio::time::*,
    tokio_rustls::*,
    wasmtime_wasi_http::{
        bindings::http::types::*,
        body::*,
        types::{IncomingResponse, OutgoingRequestConfig},
        *,
    },
};

// Derived from:
// https://github.com/bytecodealliance/wasmtime/blob/c8a5acd9831f4ec3780f34d01965b45ddf44a297/crates/wasi-http/src/types.rs#L304
// In turn derived from:
// https://github.com/rustls/rustls/blob/main/examples/src/bin/simpleclient.rs

// See: https://github.com/bytecodealliance/wasmtime/issues/8748

/// Send request.
pub async fn send_request(
    mut request: Request<HyperOutgoingBody>,
    config: OutgoingRequestConfig,
    url_context: UrlContextRef,
) -> Result<IncomingResponse, ErrorCode> {
    let authority = authority(&request, config.use_tls)?;
    let tcp_stream = connect_tcp(&authority, config.connect_timeout).await?;

    let (mut sender, worker) = if config.use_tls {
        let tls_client_configuration = tls_client_configuration(request.headers_mut(), &url_context)
            .map_err(|error| ErrorCode::InternalError(Some(error)))?;
        let tls_connector: TlsConnector = Arc::new(tls_client_configuration).into();
        let domain = domain(&authority)?;

        let tls_stream = tls_connector.connect(domain, tcp_stream).await.map_err(|error| {
            tracing::warn!("TLS protocol error: {:?}", error);
            ErrorCode::TlsProtocolError
        })?;

        connect_http(tls_stream, config.connect_timeout).await?
    } else {
        connect_http(tcp_stream, config.connect_timeout).await?
    };

    // At this point, the request contains the scheme and the authority, but
    // the HTTP packet should only include those if addressing a proxy, so
    // remove them here, since SendRequest::send_request does not do it for us

    let path_and_query = request.uri().path_and_query().map(|path_and_query| path_and_query.as_str()).unwrap_or("/");
    *request.uri_mut() = Uri::builder().path_and_query(path_and_query).build().expect("valid URI");

    let response = timeout(config.first_byte_timeout, sender.send_request(request))
        .await
        .map_err(|_| ErrorCode::ConnectionReadTimeout)?
        .map_err(hyper_request_error)?
        .map(|incoming| incoming.map_err(hyper_request_error).boxed());

    Ok(IncomingResponse { resp: response, worker: Some(worker), between_bytes_timeout: config.between_bytes_timeout })
}
