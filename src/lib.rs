#![allow(dead_code)]

use std::collections::HashMap;
use std::path::PathBuf;
use tokio::time::Duration;
pub mod cli;
pub mod run;
pub mod testmanager;

/// Identifies a test suite context.
type TestSuiteID = u32;

/// identifies a test case context.
type TestID = u32;

/// Single run of a simulator, a collection of testcases.
#[derive(Debug, Clone)]
pub struct TestSuite {
    id: TestSuiteID,
    name: String,
    description: String,
    client_versions: HashMap<String, String>,
    test_cases: HashMap<TestID, TestCase>,
    /// Log-file pertaining to the simulator. (may encompass more than just one TestSuite)
    simulator_log: String,
}

// Represents a single test case in a test suite.
#[derive(Debug, Clone)]
pub struct TestCase {
    /// Test case short name
    name: String,
    /// Test case long description
    description: String,
    // TODO: Add start and end time?
    /// The result of the whole test case
    summary_result: TestResult,
    /// Info about each client
    client_info: HashMap<String, ClientInfo>,
}

/// The payload submitted to the EndTest endpoint.
#[derive(Debug, Clone)]
pub struct TestResult {
    pass: bool,
    details: String,
}

/// Describes a client that participated in a test case
#[derive(Debug, Clone)]
pub struct ClientInfo {
    id: String,
    ip: String,
    name: String,
    /// Absolute path to logfile
    log_file: String,
}

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
