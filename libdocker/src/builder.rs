#![allow(dead_code)]
use bollard::Docker;

#[derive(Debug, Clone)]
pub struct DockerBuilder {
    client: Docker,
}

impl DockerBuilder {
    pub fn new(client: Docker) -> Self {
        DockerBuilder { client }
    }
}
