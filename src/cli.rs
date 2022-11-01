use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "axle",
    version = "0.0.1",
    about = "Axle - Portal Network end-to-end test harness."
)]
pub struct Opts {
    #[structopt(
        parse(from_os_str),
        default_value = "workspace/logs",
        long,
        help = "Target `directory` for results files and logs."
    )]
    pub results_root: PathBuf,

    #[structopt(
    default_value = "debug",
    possible_values(&["info", "warn", "debug", "trace"]),
    short,
    long = "loglevel",
    help = "Log `level` for system events.")]
    pub log_level: String,

    #[structopt(long, help = "Endpoint of the local Docker daemon.")]
    pub docker_endpoint: Option<String>,

    #[structopt(
        short,
        long = "sim",
        help = "Regular `expression` selecting the simulators to run."
    )]
    pub sim_pattern: Option<String>,

    #[structopt(
        use_delimiter = true,
        default_value = "trin",
        short,
        long = "client",
        help = "Comma separated `list` of clients to use."
    )]
    pub clients: Vec<String>,
}
