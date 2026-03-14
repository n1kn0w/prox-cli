use anyhow::{bail, Context, Result};
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
        // Try active profile
        if let Some(profile_path) = active_profile_path() {
            if profile_path.exists() {
                return profile_path;
            }
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

// ── Profile management ──────────────────────────────────────────────────

fn config_dir() -> Result<PathBuf> {
    let home = std::env::var_os("HOME").context("HOME not set")?;
    let dir = PathBuf::from(home).join(".config/prox-cli");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn active_file() -> Option<PathBuf> {
    let home = std::env::var_os("HOME")?;
    Some(PathBuf::from(home).join(".config/prox-cli/active"))
}

fn active_profile_path() -> Option<PathBuf> {
    let active = active_file()?;
    let name = std::fs::read_to_string(&active).ok()?.trim().to_string();
    if name.is_empty() {
        return None;
    }
    let home = std::env::var_os("HOME")?;
    Some(PathBuf::from(home).join(format!(".config/prox-cli/{}.toml", name)))
}

pub fn profile_list() -> Result<()> {
    let dir = config_dir()?;
    let active_name = active_file()
        .and_then(|f| std::fs::read_to_string(f).ok())
        .unwrap_or_default()
        .trim()
        .to_string();

    let mut found = false;
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".toml") {
            let profile = name.trim_end_matches(".toml");
            let marker = if profile == active_name { " *" } else { "" };
            println!("{}{}", profile, marker);
            found = true;
        }
    }
    if !found {
        eprintln!("No profiles found in {}", dir.display());
        eprintln!("Add one with: prox-cli conf add <name>");
    }
    Ok(())
}

pub fn profile_use(name: &str) -> Result<()> {
    let dir = config_dir()?;
    let profile = dir.join(format!("{}.toml", name));
    if !profile.exists() {
        bail!(
            "Profile '{}' not found. Expected: {}\nAvailable profiles: run `prox-cli conf list`",
            name,
            profile.display()
        );
    }
    let active = dir.join("active");
    std::fs::write(&active, name)?;
    eprintln!("Active profile: {}", name);
    Ok(())
}

pub fn profile_show() -> Result<()> {
    let active_name = active_file()
        .and_then(|f| std::fs::read_to_string(f).ok())
        .unwrap_or_default()
        .trim()
        .to_string();

    if active_name.is_empty() {
        eprintln!("No active profile. Using default config resolution.");
        return Ok(());
    }
    let dir = config_dir()?;
    let profile = dir.join(format!("{}.toml", active_name));
    eprintln!("Active profile: {} ({})", active_name, profile.display());

    let content = std::fs::read_to_string(&profile)
        .with_context(|| format!("Cannot read {}", profile.display()))?;
    // Mask the password
    for line in content.lines() {
        if line.trim_start().starts_with("password") {
            println!("password = \"********\"");
        } else {
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn profile_add(name: &str, source: &Path) -> Result<()> {
    let dir = config_dir()?;
    let dest = dir.join(format!("{}.toml", name));
    // Resolve the source: could be explicit path or ./config.toml
    let src = if source.exists() {
        source.to_path_buf()
    } else {
        bail!("Source config not found: {}", source.display());
    };

    // Validate it parses
    let content = std::fs::read_to_string(&src)
        .with_context(|| format!("Cannot read {}", src.display()))?;
    let _: Config = toml::from_str(&content).context("Invalid config format")?;

    std::fs::copy(&src, &dest)?;
    eprintln!("Profile '{}' saved to {}", name, dest.display());
    Ok(())
}

pub fn profile_remove(name: &str) -> Result<()> {
    let dir = config_dir()?;
    let profile = dir.join(format!("{}.toml", name));
    if !profile.exists() {
        bail!("Profile '{}' not found.", name);
    }
    std::fs::remove_file(&profile)?;

    // If it was the active profile, clear active
    let active_name = active_file()
        .and_then(|f| std::fs::read_to_string(f).ok())
        .unwrap_or_default()
        .trim()
        .to_string();
    if active_name == name {
        if let Some(active) = active_file() {
            let _ = std::fs::remove_file(active);
        }
    }
    eprintln!("Profile '{}' removed.", name);
    Ok(())
}
