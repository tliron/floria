use {
    hyper::{body::*, client::conn::http1::*},
    std::{error::Error, result::Result, time::Duration},
    tokio::{io::*, time::*},
    wasmtime_wasi::{runtime::*, *},
    wasmtime_wasi_http::{bindings::http::types::*, io::*, *},
};

/// Connect HTTP.
pub async fn connect_http<StreamT, BodyT>(
    stream: StreamT,
    connect_timeout: Duration,
) -> Result<(SendRequest<BodyT>, AbortOnDropJoinHandle<()>), ErrorCode>
where
    StreamT: 'static + AsyncRead + AsyncWrite + Send + Unpin,
    BodyT: 'static + Body + Send,
    BodyT::Data: Send,
    BodyT::Error: Into<Box<dyn Error + Send + Sync>>,
{
    let (sender, connection) = timeout(
        connect_timeout,
        // TODO: we should plumb the builder through the HTTP context, and use it here
        handshake(TokioIo::new(stream)),
    )
    .await
    .map_err(|_| ErrorCode::ConnectionTimeout)?
    .map_err(hyper_request_error)?;

    let worker = runtime::spawn(async move {
        match connection.await {
            Ok(()) => {}

            // TODO: shouldn't throw away this error and ideally should surface somewhere
            Err(error) => tracing::warn!("dropping error: {}", error),
        }
    });

    Ok((sender, worker))
}
