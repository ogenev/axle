use crate::testmanager::TestManager;
use crate::{ClientDefinition, SimulatorConfig};
use anyhow::anyhow;
use docker::builder::DockerBuilder;
use docker::container::ContainerBackend;
use inventory::Inventory;
use std::collections::HashMap;
use tracing::{debug, info};

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
    pub async fn build(
        &mut self,
        client_list: Vec<String>,
        sim_list: Vec<String>,
    ) -> anyhow::Result<()> {
        self.build_clients(client_list).await?;
        self.build_simulators(sim_list).await?;
        Ok(())
    }

    /// Runs one simulation
    pub async fn run(&self, sim: String, cfg: SimulatorConfig) -> anyhow::Result<()> {
        // TODO: Create workspace

        info!("Running simulation: {sim}");
        let mut client_def = HashMap::new();

        match cfg.clone().client_list {
            Some(clients) => {
                for name in clients {
                    match self.client_defs.clone().remove(&name) {
                        Some(def) => {
                            client_def.insert(name, def);
                        }
                        None => {
                            return Err(anyhow!("Unknown client {name} in simulation client list"))
                        }
                    }
                }
            }
            None => {
                for (name, def) in self.client_defs.clone() {
                    client_def.insert(name, def);
                }
            }
        }
        // TODO: Create test manager
        let _test_manager = TestManager::new(self.container.clone(), client_def, cfg);

        // Create the simulator container.
        let image = self.sim_images.get(&sim).unwrap();
        let container_name = self.container.create_container(image).await?;

        // TODO: Set the log file, and notify TestManager about the container.

        // Start container
        debug!("starting simulator container");
        self.container.start_container(&container_name).await?;

        debug!("deleting simulator container");
        self.container.delete_container(&container_name).await?;

        // TODO: Wait for simulation to end.
        // TODO: Count the results.
        Ok(())
    }

    /// Builds client images
    async fn build_clients(&mut self, client_list: Vec<String>) -> anyhow::Result<()> {
        if client_list.is_empty() {
            return Err(anyhow!("Client list is empty, cannot simulate"));
        }
        info!("Building {} clients ...", client_list.len());

        for client in client_list {
            if !self.inventory.has_client(&client) {
                return Err(anyhow!("UNknown client {client}"));
            }

            // TODO: Read client metadata if available

            let image = self.builder.build_client_image(client.clone()).await?;

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

    // Builds simulator images
    async fn build_simulators(&mut self, sim_list: Vec<String>) -> anyhow::Result<()> {
        info!("Building {} simulators ...", sim_list.len());

        for sim in sim_list {
            let image = self.builder.build_simulator_image(sim.clone()).await?;
            self.sim_images.insert(sim, image);
        }

        Ok(())
    }
}
