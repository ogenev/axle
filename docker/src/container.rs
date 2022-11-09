#![allow(dead_code)]

use anyhow::anyhow;
use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
};
use bollard::Docker;
use tracing::{debug, error};

#[derive(Debug, Clone)]
pub struct ContainerBackend {
    client: Docker,
}

impl ContainerBackend {
    pub fn new(client: Docker) -> Self {
        ContainerBackend { client }
    }

    /// Creates a docker container
    pub async fn create_container(&self, image_name: &str) -> anyhow::Result<String> {
        let cont_name = image_name.replace('/', "-").replace(':', "-");
        let options = Some(CreateContainerOptions { name: &cont_name });

        let config = Config {
            image: Some(image_name),
            ..Default::default()
        };

        let _ = self.client.create_container(options, config).await.unwrap();
        debug!("Container {cont_name} created");

        Ok(cont_name)
    }

    /// StartContainer starts a docker container.
    pub async fn start_container(&self, name: &str) -> anyhow::Result<()> {
        if let Err(err) = self
            .client
            .start_container(name, None::<StartContainerOptions<String>>)
            .await
        {
            return Err(anyhow!("{err}"));
        }

        debug!("Container {name} started");
        Ok(())
    }

    /// Removes the given container. If the container is running, it is stopped.
    pub async fn delete_container(&self, name: &str) -> anyhow::Result<()> {
        debug!(container=%name,"Removing container");

        let options = Some(RemoveContainerOptions {
            force: true,
            ..Default::default()
        });

        if let Err(err) = self.client.remove_container(name, options).await {
            error!(container=%name, err=%err, "Can't remove container");
            return Err(anyhow!("{err}"));
        }

        Ok(())
    }
}
