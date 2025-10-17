mod route;
mod service;
use clap::Parser;
use std::{error::Error, net::SocketAddr, result::Result, time::Duration};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Args {
    #[arg(short, long, env = "PORT", default_value_t = 8000)]
    pub port: u16,

    #[arg(short, long, env = "NAMESPACE", default_value = "default")]
    pub namespace: String,

    #[arg(long, env = "LOG_LEVEL", default_value = "info")]
    pub log_level: String,

    #[arg(long, env = "LOG_FORMAT", default_value = "info")]
    pub log_format: String,

    #[arg(short = 't', long, env = "TIMEOUT", default_value_t = 5)]
    pub timeout: u64,
}

fn get_level_filter(level: &str) -> LevelFilter {
    match level {
        "error" => LevelFilter::ERROR,
        "warn" => LevelFilter::WARN,
        "info" => LevelFilter::INFO,
        "debug" => LevelFilter::DEBUG,
        "trace" => LevelFilter::TRACE,
        _ => LevelFilter::INFO,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let fmt_layer = match args.log_format.as_str() {
        "json" => tracing_subscriber::fmt::layer()
            .json()
            .flatten_event(true)
            .with_thread_names(true)
            .boxed(),
        "info" => tracing_subscriber::fmt::layer()
            .with_thread_names(true)
            .boxed(),
        _ => panic!("Invalid log format"),
    }
    .with_filter(get_level_filter(&args.log_level));

    tracing_subscriber::registry().with(fmt_layer).init();
    tracing::info!("Starting VSL Node Info Aggregator API");

    let state = route::AppState {
        client: k8s::Client::new(args.namespace).await?,
        timeout: Duration::from_secs(args.timeout),
    };

    let app = axum::Router::new().merge(route::routes()).with_state(state);

    let listener =
        tokio::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], args.port))).await?;

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

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
