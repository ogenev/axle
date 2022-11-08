#![allow(dead_code)]

use crate::docker::DockerConfig;
use bollard::image::BuildImageOptions;
use bollard::Docker;
use std::path::PathBuf;
use tokio::task;
use tokio_stream::StreamExt;
use tracing::info;

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
    pub async fn build_client_image(&self, name: String) -> anyhow::Result<String> {
        let dir = self.config.inventory.client_directory(&name);
        // TODO: Split branch if available
        let tag = format!("axle/clients/{name}:latest");
        self.build_image(dir, &tag).await?;

        Ok(tag)
    }

    /// Builds a docker image of a simulator
    pub async fn build_simulator_image(&self, name: String) -> anyhow::Result<String> {
        let dir = self.config.inventory.simulator_directory(&name);
        let tag = format!("axle/simulators/{name}:latest");
        self.build_image(dir, &tag).await?;

        Ok(tag)
    }

    /// Creates a container by archiving the given file system,
    /// which must contain a file called "Dockerfile".
    async fn build_image(&self, dir: PathBuf, name: &str) -> anyhow::Result<()> {
        let archive_task = task::spawn_blocking(|| {
            let mut buf = vec![];
            let mut tar = tar::Builder::new(&mut buf);
            tar.append_dir_all("", dir)?;
            tar.into_inner().cloned()
        });

        let tar = archive_task.await.unwrap().unwrap();

        let build_options = BuildImageOptions {
            dockerfile: "Dockerfile".to_owned(),
            t: name.to_string(),
            rm: true,
            ..Default::default()
        };

        // info!("Building image", "image", name, "nocache", nocache, "pull", b.config.PullEnabled)
        info!(image = %name, "Building image");
        let mut build_stream = self
            .client
            .build_image(build_options, None, Some(tar.into()));

        while build_stream.next().await.is_some() {}

        Ok(())
    }
}
