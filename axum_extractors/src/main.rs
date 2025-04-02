use axum::{extract::ConnectInfo, routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/average_joe_absolutely_needs_vpn", get(average_joe_absolutely_needs_vpn));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

async fn index(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!(
        "Your ip address is \"{addr}\".\n\
        You are in immediate danger of getting identified by bad people.\n\
        Thankfully we have a VPN service to hide your ip. \n\
        Visit this link to download it \"http://localhost:3000/average_joe_absolutely_needs_vpn\""
    )
}

async fn average_joe_absolutely_needs_vpn() -> &'static str {
    "ThatFireVpn: Get the best vpn for only $4.95"
}
