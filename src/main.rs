pub mod common;
pub mod constants;
pub mod modules;
pub mod router;

use std::{net::SocketAddr, path::PathBuf};

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use dotenv_plus::DotEnv;
use tokio::net::TcpListener;
use workspace_root::tokio::get_workspace_root_async;

use crate::{
    constants::env::{get_port, is_dev_https},
    router::create_router,
};

/// Server options for HTTP and HTTPS servers.
#[derive(Debug, Clone)]
struct ServerOptions {
    router: Router,
    address: SocketAddr,
}

/// HTTP server.
async fn http(options: ServerOptions) {
    // listener
    let listener: TcpListener =
        match tokio::net::TcpListener::bind(&options.address).await {
            | Ok(_listener) => _listener,
            | Err(e) => {
                panic!("Unable to bind address {}: {}", &options.address, e)
            },
        };

    println!("Server running on {}", &options.address);

    // serve
    if let Err(e) = axum::serve(listener, options.router).await {
        panic!("Unable to start server: {}", e)
    };
}

/// HTTPS server.
async fn https(options: ServerOptions) {
    // certificate and private key
    let cert_path: PathBuf =
        get_workspace_root_async().await.join("target").join(".cert");

    let config: RustlsConfig = match RustlsConfig::from_pem_file(
        cert_path.join("cert.pem"),
        cert_path.join("key.pem"),
    )
    .await
    {
        | Ok(config) => config,
        | Err(_) => panic!("Unable to load certificate"),
    };

    println!("Server running on {}", &options.address);

    // serve
    if let Err(e) = axum_server::bind_rustls(options.address, config)
        .serve(options.router.into_make_service())
        .await
    {
        panic!("Unable to start server: {}", e)
    };
}

/// Main function.
#[tokio::main]
async fn main() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    DotEnv::new().run();

    // router
    let router: Router = create_router().await;

    // port
    let port: u16 = match get_port().parse() {
        | Ok(port) => port,
        | Err(_) => panic!("Unable to parse port"),
    };

    // address
    let address: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port));

    // server
    if is_dev_https() {
        https(ServerOptions { router, address }).await
    } else {
        http(ServerOptions { router, address }).await
    }
}
