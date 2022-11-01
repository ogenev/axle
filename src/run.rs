#![allow(dead_code)]

use libdocker::builder::DockerBuilder;
use libdocker::container::ContainerBackend;

#[derive(Debug)]
pub struct Runner {
    builder: DockerBuilder,
    container: ContainerBackend,
}

impl Runner {
    pub fn new(builder: DockerBuilder, container: ContainerBackend) -> Self {
        Runner { builder, container }
    }

    /// Build clients and simulators images
    pub fn build(&self) {
        todo!()
    }

    /// Runs one simulation
    pub fn run(&self) {
        todo!()
    }
}
