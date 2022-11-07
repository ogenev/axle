#![allow(dead_code)]
use crate::docker::DockerConfig;
use bollard::Docker;

#[derive(Debug, Clone)]
pub struct DockerBuilder {
    client: Docker,
    config: DockerConfig,
}

impl DockerBuilder {
    pub fn new(client: Docker, config: DockerConfig) -> Self {
        Self { client, config }
    }

    /// Builds a docker image of the given client
    pub fn build_client_image(&self, _name: String) -> anyhow::Result<&str> {
        todo!()
    }

    fn build_image(&self) -> anyhow::Result<&str> {
        todo!()
    }
}
