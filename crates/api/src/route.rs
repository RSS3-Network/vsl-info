use std::time::Duration;

use axum::{Json, Router, extract::State, routing::get};
use k8s::Client;
use rpc_types_optimism::p2p::{PeerDump, PeerInfo};
use serde::{Deserialize, Serialize};

use crate::service;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub timeout: Duration,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PeerInfoWithPeers {
    #[serde(flatten)]
    pub info: PeerInfo,
    #[serde(flatten)]
    pub peers: PeerDump,
}

pub async fn list_peers_info(State(state): State<AppState>) -> Json<Vec<PeerInfo>> {
    let endpoints = state.client.discover_rpc_endpoints().await.unwrap();

    let handles: Vec<_> = endpoints
        .into_iter()
        .map(|e| tokio::spawn(service::info(e, state.timeout)))
        .collect();

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(Some(peer_info)) = handle.await {
            results.push(peer_info);
        }
    }

    Json(results)
}

pub async fn get_topology(State(state): State<AppState>) -> Json<Vec<PeerInfoWithPeers>> {
    let endpoints = state.client.discover_rpc_endpoints().await.unwrap();
    let timeout = state.timeout;

    let handles: Vec<_> = endpoints
        .into_iter()
        .map({
            move |e| {
                let timeout = timeout;
                tokio::spawn(async move {
                    let info = service::info(e.clone(), timeout).await;
                    let peers = service::peers(e, true, timeout).await;
                    (info, peers)
                })
            }
        })
        .collect();

    let mut results = Vec::new();
    for handle in handles {
        if let Ok((Some(info), Some(peers))) = handle.await {
            results.push(PeerInfoWithPeers { info, peers });
        }
    }

    Json(results)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/healthz", get(|| async { "OK" }))
        .route("/peers", get(list_peers_info))
        .route("/topology", get(get_topology))
}
