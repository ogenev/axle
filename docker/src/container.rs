#![allow(dead_code)]

use bollard::Docker;

#[derive(Debug, Clone)]
pub struct ContainerBackend {
    client: Docker,
}

impl ContainerBackend {
    pub fn new(client: Docker) -> Self {
        ContainerBackend { client }
    }
}
