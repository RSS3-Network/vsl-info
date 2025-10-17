use std::time::Duration;

use provider::optimism::p2p::P2P;
use rpc_types_optimism::p2p::{PeerDump, PeerInfo};

pub async fn info(url: String, timeout: Duration) -> Option<PeerInfo> {
    tokio::time::timeout(timeout, async {
        let p = provider::create_provider(&url);
        p.info().await
    })
    .await
    .ok()
    .and_then(|r| r.ok())
}

pub async fn peers(url: String, connected: bool, timeout: Duration) -> Option<PeerDump> {
    tokio::time::timeout(timeout, async {
        let p = provider::create_provider(&url);
        p.peers(connected).await
    })
    .await
    .ok()
    .and_then(|r| r.ok())
}
