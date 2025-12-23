use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct SpearConfig {
    pub peacock_github_repo: String,
}

impl Default for SpearConfig {
    fn default() -> Self {
        Self {
            peacock_github_repo: "thepeacockproject/peacock".to_string(),
        }
    }
}

pub fn get_spear_config_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap();
    path.push("spear");
    path.push("config.toml");
    path
}

pub fn load_spear_config() -> SpearConfig {
    let path = get_spear_config_path();
    log::info!("[+] Loading Spear config from: {:?}", path);
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        let config: SpearConfig = toml::from_str(&content).unwrap_or_default();
        log::info!(
            "[+] Loaded Spear config: peacock_github_repo={}",
            config.peacock_github_repo
        );
        config
    } else {
        log::info!("[+] Spear config not found, creating default");
        let default = SpearConfig::default();
        save_spear_config(&default);
        default
    }
}

pub fn save_spear_config(config: &SpearConfig) {
    let path = get_spear_config_path();
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let content = toml::to_string(config).unwrap();
    fs::write(&path, content).unwrap();
}
