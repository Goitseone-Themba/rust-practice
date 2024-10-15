use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_rustls::TlsConnector;
use rustls::{ClientConfig, RootCertStore};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up TLS configuration
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(
        webpki_roots::TLS_SERVER_ROOTS
            .0
            .iter()
            .map(|ta| {
                rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            })
    );

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    // Create TLS connector
    let connector = TlsConnector::from(Arc::new(config));

    // Connect to the server
    let domain = rustls::ServerName::try_from("api.github.com")
        .map_err(|_| "invalid dnsname")?;
    let stream = tokio::net::TcpStream::connect("api.github.com:443").await?;
    let mut stream = connector.connect(domain, stream).await?;

    // Format and send the HTTP request
    let request = format!(
        "GET /users/Goitseone-Themba/events HTTP/1.1\r\n\
        Host: api.github.com\r\n\
        User-Agent: RustHTTPClient/1.0\r\n\
        Accept: application/json\r\n\
        Connection: close\r\n\
        \r\n"
    );
    stream.write_all(request.as_bytes()).await?;

    // Read the response
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;

    // Convert the response to a string and print it
    let response_string = String::from_utf8_lossy(&response);
    println!("Response:\n{}", response_string);

    Ok(())
}
