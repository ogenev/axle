use crate::ClientDefinition;
use anyhow::anyhow;
use inventory::Inventory;
use libdocker::builder::DockerBuilder;
use libdocker::container::ContainerBackend;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::info;

/// Runner executes a simulation runs
#[derive(Debug)]
pub struct Runner {
    inventory: Inventory,
    builder: DockerBuilder,
    container: ContainerBackend,
    /// Holds the image names of all built simulators
    sim_images: HashMap<String, String>,
    client_defs: HashMap<String, ClientDefinition>,
}

impl Runner {
    pub fn new(inventory: Inventory, builder: DockerBuilder, container: ContainerBackend) -> Self {
        Runner {
            inventory,
            builder,
            container,
            sim_images: HashMap::new(),
            client_defs: HashMap::new(),
        }
    }

    /// Build clients and simulators images
    pub fn build(
        &mut self,
        client_list: Vec<String>,
        _sim_list: Vec<String>,
    ) -> anyhow::Result<()> {
        self.build_clients(client_list)?;
        // self.build_simulators(sim_list);
        Ok(())
    }

    /// Runs one simulation
    pub fn run(&self) {
        todo!()
    }

    /// Builds client images
    fn build_clients(&mut self, client_list: Vec<String>) -> anyhow::Result<()> {
        if client_list.is_empty() {
            return Err(anyhow!("Client list is empty, cannot simulate"));
        }
        info!("Building {} clients ...", client_list.len());

        for client in client_list {
            let image = self.builder.build_client_image(client.clone()).unwrap();

            //TODO: Read version

            self.client_defs.insert(
                client.clone(),
                ClientDefinition {
                    name: client,
                    version: "".to_owned(),
                    image: image.to_owned(),
                },
            );
        }

        Ok(())
    }

    fn build_simulators(_sim_list: Vec<PathBuf>) {
        todo!()
    }
}
