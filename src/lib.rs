#![allow(dead_code)]

use std::path::PathBuf;
use tokio::time::Duration;
pub mod cli;
pub mod run;

/// Contains the simulation parameters.
#[derive(Debug, Clone)]
pub struct SimulatorConfig {
    pub log_dir: PathBuf,

    // Parameters of simulation.
    log_level: u8,
    parallelism: u8,
    test_pattern: String,

    /// This is the time limit for the simulation run.
    /// There is no default limit.
    duration_limit: Duration,

    // These are the clients which are made available to the simulator.
    // If unset (i.e. None), all built clients are used.
    client_list: Option<Vec<String>>,

    // This configures the amount of time the simulation waits
    // for the client to open port 8545 after launching the container.
    client_start_timeout: Duration,
}

impl SimulatorConfig {
    pub fn new(test_result_root: PathBuf) -> Self {
        Self {
            log_dir: test_result_root,
            log_level: 3,
            parallelism: 1,
            test_pattern: "".to_string(),
            duration_limit: Default::default(),
            client_list: None,
            client_start_timeout: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientDefinition {
    name: String,
    version: String,
    image: String,
}
