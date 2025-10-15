use alloy::{
    network::Ethereum,
    providers::{Provider, ProviderBuilder},
};

pub mod optimism;

pub fn create_provider(endpoint: &str) -> impl Provider<Ethereum> {
    let url = endpoint.parse().expect("Invalid URL");
    ProviderBuilder::new().connect_http(url)
}
