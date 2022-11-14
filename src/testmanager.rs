use crate::{ClientDefinition, SimulatorConfig, TestCase, TestID, TestSuite, TestSuiteID};
use docker::container::ContainerBackend;
use std::collections::HashMap;

// Collects test results during a simulation run.
pub struct TestManager {
    cfg: SimulatorConfig,
    backend: ContainerBackend,
    client_def: HashMap<String, ClientDefinition>,
    sim_container_id: String,
    sim_log_fle: String,
    running_test_suites: HashMap<TestSuiteID, TestSuite>,
    running_test_cases: HashMap<TestID, TestCase>,
    test_suite_counter: u32,
    test_case_counter: u32,
    results: HashMap<TestSuiteID, TestSuite>,
}

impl TestManager {
    pub fn new(
        backend: ContainerBackend,
        clients: HashMap<String, ClientDefinition>,
        cfg: SimulatorConfig,
    ) -> Self {
        Self {
            cfg,
            backend,
            client_def: clients,
            sim_container_id: "".to_string(),
            sim_log_fle: "".to_string(),
            running_test_suites: Default::default(),
            running_test_cases: Default::default(),
            test_suite_counter: 0,
            test_case_counter: 0,
            results: Default::default(),
        }
    }
}
