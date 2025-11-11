use {::http::*, kutil::tls::*, read_url::*, rustls::client::*, std::result::Result, tokio_rustls::*};

const ROOT_CERTIFICATES_HEADER: &str = "xx-root-certificates";
const CERTIFICATES_HEADER: &str = "xx-certificates";
const PRIVATE_KEY_HEADER: &str = "xx-private-key";

/// TLS client configuration.
pub fn tls_client_configuration(
    headers: &mut HeaderMap<HeaderValue>,
    url_context: &UrlContextRef,
) -> Result<ClientConfig, String> {
    let root_certificates = {
        match read_url_from_header(headers, ROOT_CERTIFICATES_HEADER, &url_context)? {
            Some(certificates) => Some(parse_certificates_pem(&certificates).map_err(|error| error.to_string())?),
            None => None,
        }
    };

    let builder =
        ClientConfig::builder().with_standard_verifier(root_certificates).map_err(|error| error.to_string())?;

    if let Some(certificates) = read_url_from_header(headers, CERTIFICATES_HEADER, &url_context)?
        && let Some(private_key) = read_url_from_header(headers, PRIVATE_KEY_HEADER, &url_context)?
    {
        let certificates = parse_certificates_pem(&certificates).map_err(|error| error.to_string())?;
        let private_key = parse_private_key_pem(&private_key).map_err(|error| error.to_string())?;
        return builder.with_client_auth_cert(certificates, private_key).map_err(|error| error.to_string());
    }

    Ok(builder.with_no_client_auth())
}

fn read_url_from_header(
    headers: &mut HeaderMap<HeaderValue>,
    header_name: &str,
    url_context: &UrlContextRef,
) -> Result<Option<Vec<u8>>, String> {
    let Some(url) = headers.remove(header_name) else {
        return Ok(None);
    };

    let url = url.to_str().map_err(|error| error.to_string())?;
    let url = url_context.url(url).map_err(|error| error.to_string())?;
    let mut reader = url.open().map_err(|error| error.to_string())?;
    let mut bytes = Vec::default();
    reader.read_to_end(&mut bytes).map_err(|error| error.to_string())?;
    Ok(Some(bytes))
}
