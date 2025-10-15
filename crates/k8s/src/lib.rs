use anyhow::{Result, anyhow};
use k8s_openapi::api::core::v1::Pod;
use kube::{
    Client as KubeClient,
    api::{Api, ListParams},
};

/// Annotation key to mark pods for scraping (similar to prometheus.io/scrape)
pub const VSL_OP_NODE_SCRAPE: &str = "vsl.rss3.io/scrape";

/// Annotation key to specify the RPC request port (similar to prometheus.io/port)
pub const VSL_OP_NODE_RPC_PORT: &str = "vsl.rss3.io/port";

/// Annotation key to specify the RPC request path (similar to prometheus.io/path)
pub const VSL_OP_NODE_RPC_PATH: &str = "vsl.rss3.io/path";

/// Annotation key to specify the RPC request protocol (similar to prometheus.io/protocol)
pub const VSL_OP_NODE_RPC_PROTOCOL: &str = "vsl.rss3.io/protocol";

/// Default RPC port if not specified in annotations
pub const DEFAULT_RPC_PORT: i32 = 9545;

/// Default RPC path if not specified in annotations
pub const DEFAULT_RPC_PATH: &str = "/";

/// Default RPC protocol if not specified in annotations
pub const DEFAULT_RPC_PROTOCOL: &str = "http";

/// Kubernetes client for discovering pods and extracting RPC endpoints
#[derive(Clone)]
pub struct Client {
    inner: KubeClient,
    namespace: String,
}

impl Client {
    /// Create a new K8sClient for the specified namespace
    pub async fn new(namespace: impl Into<String>) -> Result<Self> {
        let inner = KubeClient::try_default().await?;
        Ok(Self {
            inner,
            namespace: namespace.into(),
        })
    }

    /// Discover all pods with scrape annotations and extract their RPC URLs
    ///
    /// This method discovers pods that have the annotation `vsl.rss3.io/scrape=true`
    /// and builds RPC URLs using the pod IP and port from annotations.
    ///
    /// # Returns
    /// A vector of Urls containing all discovered endpoints
    pub async fn discover_rpc_endpoints(&self) -> Result<Vec<String>> {
        let pods: Api<Pod> = Api::namespaced(self.inner.clone(), &self.namespace);
        let pod_list = pods.list(&ListParams::default()).await?;

        pod_list
            .items
            .into_iter()
            .filter_map(|pod| should_scrape_pod(&pod).then(|| extract_pod_rpc_info(&pod)))
            .collect()
    }
}

/// Check if a pod should be scraped based on annotations
fn should_scrape_pod(pod: &Pod) -> bool {
    pod.metadata
        .annotations
        .as_ref()
        .and_then(|annotations| annotations.get(VSL_OP_NODE_SCRAPE))
        .map(|value| value.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Get the RPC port from pod annotations, or use default
fn get_rpc_port(pod: &Pod) -> i32 {
    pod.metadata
        .annotations
        .as_ref()
        .and_then(|annotations| annotations.get(VSL_OP_NODE_RPC_PORT))
        .and_then(|port_str| port_str.parse::<i32>().ok())
        .unwrap_or(DEFAULT_RPC_PORT)
}

/// Get the RPC path from pod annotations, or use default
fn get_rpc_path(pod: &Pod) -> String {
    pod.metadata
        .annotations
        .as_ref()
        .and_then(|annotations| annotations.get(VSL_OP_NODE_RPC_PATH))
        .map(|s| s.as_str())
        .unwrap_or(DEFAULT_RPC_PATH)
        .to_string()
}

/// Get the RPC protocol from pod annotations, or use default
fn get_rpc_protocol(pod: &Pod) -> String {
    pod.metadata
        .annotations
        .as_ref()
        .and_then(|annotations| annotations.get(VSL_OP_NODE_RPC_PROTOCOL))
        .map(|s| s.as_str())
        .unwrap_or(DEFAULT_RPC_PROTOCOL)
        .to_string()
}

/// Extract RPC information from a pod
fn extract_pod_rpc_info(pod: &Pod) -> Result<String> {
    let ip = pod
        .status
        .as_ref()
        .and_then(|s| s.pod_ip.as_ref())
        .ok_or_else(|| anyhow!("Pod has no IP address"))?
        .clone();

    Ok(format!(
        "{}://{}:{}{}",
        get_rpc_protocol(pod),
        ip,
        get_rpc_port(pod),
        get_rpc_path(pod)
    ))
}
