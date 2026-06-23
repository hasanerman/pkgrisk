use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    pub general: GeneralConfig,
    pub thresholds: ThresholdsConfig,
    pub license: LicenseConfig,
    pub ecosystems: EcosystemsConfig,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct GeneralConfig {
    pub cache_ttl_hours: u64,
    pub default_format: String,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct ThresholdsConfig {
    pub fail_under: u8,
    pub warn_under: u8,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct LicenseConfig {
    pub project_license: String,
    pub blocklist: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct EcosystemsConfig {
    pub disabled: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                cache_ttl_hours: 24,
                default_format: "terminal".to_string(),
            },
            thresholds: ThresholdsConfig {
                fail_under: 40,
                warn_under: 65,
            },
            license: LicenseConfig {
                project_license: "MIT".to_string(),
                blocklist: vec!["AGPL-3.0".to_string(), "GPL-2.0".to_string()],
            },
            ecosystems: EcosystemsConfig {
                disabled: vec![],
            },
        }
    }
}

pub fn load_config() -> anyhow::Result<Config> {
    // Try to load from ~/.config/pkgrisk/config.toml or .pkgriskrc.toml
    let mut config_paths = vec![PathBuf::from(".pkgriskrc.toml")];
    if let Some(home) = dirs::home_dir() {
        config_paths.push(home.join(".config").join("pkgrisk").join("config.toml"));
    }

    for path in config_paths {
        if path.exists() {
            let contents = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&contents)?;
            return Ok(config);
        }
    }

    // Default config
    Ok(Config::default())
}
