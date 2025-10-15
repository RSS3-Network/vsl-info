use std::collections::BTreeMap;

use alloy::primitives::ChainId;
use serde::{Deserialize, Serialize};

pub const RPC_METHOD_INFO: &str = "opp2p_self";
pub const RPC_METHOD_PEERS: &str = "opp2p_peers";
pub const RPC_METHOD_STATS: &str = "opp2p_peerStats";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    #[serde(rename = "peerID")]
    pub peer_id: String,

    #[serde(rename = "nodeID")]
    pub node_id: String,

    #[serde(rename = "userAgent")]
    pub user_agent: String,

    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,

    #[serde(rename = "ENR")]
    pub enr: String,

    pub addresses: Vec<String>,
    pub protocols: Option<Vec<String>>,

    pub connectedness: i32,
    pub direction: i32,
    pub protected: bool,

    #[serde(rename = "chainID")]
    pub chain_id: ChainId,

    pub latency: u64,

    #[serde(rename = "gossipBlocks")]
    pub gossip_blocks: bool,

    #[serde(rename = "scores")]
    pub peer_scores: PeerScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerScore {
    pub gossip: GossipScores,

    #[serde(rename = "reqResp")]
    pub req_resp: ReqRespScores,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipScores {
    pub total: f64,
    pub blocks: TopicScores,
    #[serde(rename = "IPColocationFactor")]
    ip_colocation_factor: f64,
    #[serde(rename = "behavioralPenalty")]
    behavioral_penalty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicScores {
    #[serde(rename = "timeInMesh")]
    time_in_mesh: f64,
    #[serde(rename = "firstMessageDeliveries")]
    first_message_deliveries: f64,
    #[serde(rename = "meshMessageDeliveries")]
    mesh_message_deliveries: f64,
    #[serde(rename = "invalidMessageDeliveries")]
    invalid_message_deliveries: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReqRespScores {
    #[serde(rename = "validResponses")]
    valid_responses: f64,
    #[serde(rename = "errorResponses")]
    error_responses: f64,
    #[serde(rename = "rejectedPayloads")]
    rejected_payloads: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerDump {
    #[serde(rename = "totalConnected")]
    pub total_connected: u64,

    pub peers: BTreeMap<String, PeerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerStats {
    pub connected: u64,
    pub table: u64,
    #[serde(rename = "blocksTopic")]
    pub blocks_topic: u64,
    #[serde(rename = "blocksTopicV2")]
    pub blocks_topic_v2: u64,
    #[serde(rename = "blocksTopicV3")]
    pub blocks_topic_v3: u64,
    #[serde(rename = "blocksTopicV4")]
    pub blocks_topic_v4: u64,
    pub banned: u64,
    pub known: u64,
}
