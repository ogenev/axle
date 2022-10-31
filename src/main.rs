use axle::run::Runner;
use libdocker::docker::Docker;

fn main() {
    // Set loggers

    // Cli arguments parse
    // - docker-endpoint - Endpoint of the local Docker daemon, default ""
    // - sim -  Regular `expression` selecting the simulators to run, default ""
    // - client - Comma separated `list` of clients to use, default "trin"

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
