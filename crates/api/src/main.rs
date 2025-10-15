mod route;
use clap::Parser;
use std::{error::Error, net::SocketAddr, result::Result, time::Duration};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Args {
    #[arg(short, long, env = "PORT", default_value_t = 8000)]
    pub port: u16,

    #[arg(short, long, env = "NAMESPACE", default_value = "default")]
    pub namespace: String,

    #[arg(short, long, env = "LOG_LEVEL", default_value = "info")]
    pub log_level: String,

    #[arg(short = 't', long, env = "TIMEOUT", default_value_t = 5)]
    pub timeout: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let listener =
        tokio::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], args.port))).await?;

    let state = route::AppState {
        client: k8s::Client::new(args.namespace).await?,
        timeout: Duration::from_secs(args.timeout),
    };

    let app = axum::Router::new().merge(route::routes()).with_state(state);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
