#![allow(unused_variables)]

use axle::cli::Opt;
use axle::inventory::Inventory;
use axle::run::Runner;
use libdocker::docker::Docker;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;
use tracing::error;
use tracing_subscriber::EnvFilter;

fn main() {
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

    // Get simulators list
    let simulators = inventory.match_simulators(&opt.sim_pattern).unwrap();
    if simulators.is_empty() {
        error!("No simulators for pattern: {}", opt.sim_pattern);
        process::exit(1);
    }

    // Create docker backends
    let (docker_builder, container_backend) = Docker::connect(None);

    // Run:
    // - new runner
    let runner = Runner::new(docker_builder, container_backend);
    // - get client list and build runner
    runner.build();

    // - Iterates over all simulators and run environment
    for sim in simulators {
        runner.run();
    }

    println!("Hello Portal Network");
}
