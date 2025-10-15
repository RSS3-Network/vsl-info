use std::time::Duration;

use axum::{Json, Router, extract::State, routing::get};
use k8s::Client;
use provider::optimism::p2p::P2P;
use rpc_types_optimism::p2p::PeerInfo;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub timeout: Duration,
}

pub async fn get_peers(State(state): State<AppState>) -> Json<Vec<PeerInfo>> {
    let endpoints = state.client.discover_rpc_endpoints().await.unwrap();

    let handles: Vec<_> = endpoints
        .into_iter()
        .map(|e| {
            let timeout = state.timeout;
            tokio::spawn(async move {
                tokio::time::timeout(timeout, async {
                    let p = provider::create_provider(&e);
                    p.info().await
                })
                .await
                .ok()
                .and_then(|r| r.ok())
            })
        })
        .collect();

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(Some(peer_info)) = handle.await {
            results.push(peer_info);
        }
    }

    Json(results)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/healthz", get(|| async { "OK" }))
        .route("/peers", get(get_peers))
}
