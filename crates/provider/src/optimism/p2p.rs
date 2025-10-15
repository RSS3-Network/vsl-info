use alloy::{network::Network, providers::Provider, transports::TransportResult};
use rpc_types_optimism::p2p::{
    PeerDump, PeerInfo, PeerStats, RPC_METHOD_INFO, RPC_METHOD_PEERS, RPC_METHOD_STATS,
};

#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
pub trait P2P<N: Network>: Send + Sync {
    async fn info(&self) -> TransportResult<PeerInfo>;

    async fn peers(&self, connected: bool) -> TransportResult<PeerDump>;

    async fn stats(&self) -> TransportResult<PeerStats>;
}

#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
impl<N, P> P2P<N> for P
where
    N: Network,
    P: Provider<N>,
{
    async fn info(&self) -> TransportResult<PeerInfo> {
        self.client().request_noparams(RPC_METHOD_INFO).await
    }

    async fn peers(&self, connected: bool) -> TransportResult<PeerDump> {
        self.client().request(RPC_METHOD_PEERS, (connected,)).await
    }

    async fn stats(&self) -> TransportResult<PeerStats> {
        self.client().request_noparams(RPC_METHOD_STATS).await
    }
}
