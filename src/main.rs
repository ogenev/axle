#![allow(unused_variables)]

use axle::cli::Opt;
use axle::run::Runner;
use docker::docker::{Docker, DockerConfig};
use inventory::Inventory;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(opt.log_level))
        .unwrap();

    tracing_subscriber::fmt()
        .with_env_filter(filter_layer)
        .init();

    // Load inventory
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut inventory = Inventory::new(root_dir);
    inventory.load_inventory();

    println!("Inventory: {:?}", inventory);

    // Get simulators list
    let simulators = inventory.match_simulators(&opt.sim_pattern).unwrap();
    if simulators.is_empty() {
        eprintln!("No simulators for pattern: {}", opt.sim_pattern);
        process::exit(1);
    }

    println!("Matched simulators: {:?}", simulators);

    // Create docker backends
    let docker_cfg = DockerConfig::new(inventory.clone());
    let (docker_builder, container_backend) = Docker::connect(opt.docker_endpoint, docker_cfg);

    // Run
    let mut runner = Runner::new(inventory, docker_builder, container_backend);
    runner.build(opt.clients, simulators.clone()).await?;

    // Iterates over all simulators and run environment
    for sim in simulators {
        // runner.run();
    }

    println!("Hello Portal Network");
    Ok(())
}
