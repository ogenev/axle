use anyhow::anyhow;
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Keeps names of clients and simulators found under root directory
#[derive(Clone, Debug)]
pub struct Inventory {
    base_dir: PathBuf,
    clients: HashMap<String, PathBuf>,
    simulators: HashMap<String, PathBuf>,
}

impl Inventory {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Inventory {
            base_dir: PathBuf::from(base_dir.as_ref()),
            clients: HashMap::new(),
            simulators: HashMap::new(),
        }
    }

    pub fn load_inventory(&mut self) {
        self.clients = find_dockerfiles(self.base_dir.join("clients"));
        self.simulators = find_dockerfiles(self.base_dir.join("simulators"));
    }

    /// Returns matching simulator names
    pub fn match_simulators(&self, expr: &str) -> anyhow::Result<Vec<String>> {
        let re = Regex::new(expr).map_err(|err| anyhow!(err))?;
        let mut matched_simulators = Vec::new();
        for (sim, _) in self.simulators.clone().into_iter() {
            if re.is_match(&sim) {
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

fn find_dockerfiles<P: AsRef<Path>>(dir: P) -> HashMap<String, PathBuf> {
    let mut dockerfiles: HashMap<String, PathBuf> = HashMap::new();
    for entry in WalkDir::new(dir.as_ref())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.starts_with("Dockerfile") {
            if let Ok(name) = entry.path().strip_prefix(dir.as_ref()) {
                if let Some(name) = name.parent() {
                    dockerfiles.insert(name.to_string_lossy().parse().unwrap(), entry.into_path());
                }
            }
        }
    }

    dockerfiles
}
