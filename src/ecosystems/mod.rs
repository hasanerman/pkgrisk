pub mod crates;
pub mod npm;
pub mod osv;
pub mod pypi;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub repository_url: Option<String>,
    pub latest_release_date: Option<String>,
    pub dependencies: Vec<String>,
    pub weekly_downloads: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ecosystem {
    Npm,
    Pypi,
    Crates,
}

impl Ecosystem {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "npm" => Some(Ecosystem::Npm),
            "pypi" => Some(Ecosystem::Pypi),
            "crates" | "crates.io" | "cargo" => Some(Ecosystem::Crates),
            _ => None,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Ecosystem::Npm => "npm",
            Ecosystem::Pypi => "pypi",
            Ecosystem::Crates => "crates",
        }
    }
}
