use crate::builder::DockerBuilder;
use crate::container::ContainerBackend;
use bollard::{Docker as DockerAPI, API_DEFAULT_VERSION};
use inventory::Inventory;

#[derive(Debug, Clone)]
pub struct DockerConfig {
    pub inventory: Inventory,
}

impl DockerConfig {
    pub fn new(inv: Inventory) -> Self {
        Self { inventory: inv }
    }
}

#[derive(Debug, Clone)]
pub struct Docker;

impl Docker {
    pub fn connect(
        docker_host: Option<String>,
        config: DockerConfig,
    ) -> (DockerBuilder, ContainerBackend) {
        let client = match docker_host {
            Some(addr) => DockerAPI::connect_with_http(&addr, 4, API_DEFAULT_VERSION).unwrap(),
            None => DockerAPI::connect_with_http_defaults().unwrap(),
        };
        (
            DockerBuilder::new(client.clone(), config),
            ContainerBackend::new(client),
        )
    }
}
