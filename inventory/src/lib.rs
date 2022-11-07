use anyhow::anyhow;
use regex::Regex;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Keeps names of clients and simulators found under root directory
#[derive(Clone, Debug)]
pub struct Inventory {
    base_dir: PathBuf,
    clients: Vec<PathBuf>,
    simulators: Vec<PathBuf>,
}

impl Inventory {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Inventory {
            base_dir: PathBuf::from(base_dir.as_ref()),
            clients: Vec::new(),
            simulators: Vec::new(),
        }
    }

    pub fn load_inventory(&mut self) {
        self.clients = find_dockerfiles(self.base_dir.join("clients"));
        self.simulators = find_dockerfiles(self.base_dir.join("simulators"));
    }

    /// Returns matching simulator names
    pub fn match_simulators(&self, expr: &str) -> anyhow::Result<Vec<PathBuf>> {
        let re = Regex::new(expr).map_err(|err| anyhow!(err))?;
        let mut matched_simulators = Vec::new();
        for sim in self.simulators.clone() {
            if re.is_match(&sim.to_string_lossy()) {
                matched_simulators.push(sim);
            }
        }
        Ok(matched_simulators)
    }

    /// ClientDirectory returns the directory containing the given client's Dockerfile.
    pub fn client_directory(&self, name: String) -> PathBuf {
        self.base_dir.join("clients").join(name)
    }
}

fn find_dockerfiles<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let mut dockerfiles: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.starts_with("Dockerfile") {
            dockerfiles.push(entry.into_path());
        }
    }

    dockerfiles
}
