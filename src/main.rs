#![allow(unused_variables)]

use axle::cli::Opts;
use axle::run::Runner;
use libdocker::docker::Docker;
use structopt::StructOpt;

fn main() {
    let opts = Opts::from_args();

    // Set loggers

    // Get simulators list
    let simulators: Vec<&str> = Vec::new();

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
