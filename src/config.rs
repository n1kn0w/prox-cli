use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct Config {
    pub proxmox: ProxmoxConfig,
}

#[derive(Deserialize)]
pub struct ProxmoxConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub node: String,
    #[serde(default)]
    pub verify_ssl: bool,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let resolved = Self::resolve_path(path);
        let content = std::fs::read_to_string(&resolved)
            .with_context(|| format!("Cannot read config file: {}", resolved.display()))?;
        toml::from_str(&content).context("Failed to parse config file")
    }

    fn resolve_path(path: &Path) -> PathBuf {
        // If explicit path exists, use it
        if path.exists() {
            return path.to_path_buf();
        }
        // Try ~/.config/prox-cli/config.toml
        if let Some(home) = std::env::var_os("HOME") {
            let global = PathBuf::from(home).join(".config/prox-cli/config.toml");
            if global.exists() {
                return global;
            }
        }
        // Fallback to original path (will produce a clear error)
        path.to_path_buf()
    }
}
