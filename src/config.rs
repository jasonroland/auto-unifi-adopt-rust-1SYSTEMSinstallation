use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub controller_url: String,
    pub ssh_username: String,
    pub ssh_password: String,
    pub alt_ssh_username: String,
    pub alt_ssh_password: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            controller_url: String::from("http://192.168.1.1:8080"),
            ssh_username: String::from("ubnt"),
            ssh_password: String::from("ubnt"),
            alt_ssh_username: String::from(""),
            alt_ssh_password: String::from(""),
        }
    }
}

fn get_config_path() -> PathBuf {
    // Use platform-appropriate config directory
    // Windows: C:\Users\<user>\AppData\Roaming\UniFiAdoption
    // macOS: ~/Library/Application Support/UniFiAdoption
    // Linux: ~/.config/UniFiAdoption
    let config_dir = get_config_dir();
    let app_dir = config_dir.join("UniFiAdoption");
    // Create directory if it doesn't exist
    let _ = fs::create_dir_all(&app_dir);
    app_dir.join("config.txt")
}

#[cfg(windows)]
fn get_config_dir() -> PathBuf {
    std::env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

#[cfg(target_os = "macos")]
fn get_config_dir() -> PathBuf {
    std::env::var("HOME")
        .map(|h| PathBuf::from(h).join("Library/Application Support"))
        .unwrap_or_else(|_| PathBuf::from("."))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn get_config_dir() -> PathBuf {
    std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(|h| PathBuf::from(h).join(".config"))
                .unwrap_or_else(|_| PathBuf::from("."))
        })
}

pub fn load_config() -> AppConfig {
    let path = get_config_path();
    if !path.exists() {
        let config = AppConfig::default();
        save_config(&config);
        return config;
    }

    let mut config = AppConfig::default();
    if let Ok(contents) = fs::read_to_string(&path) {
        for line in contents.lines() {
            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "controller_url" => config.controller_url = value.trim().to_string(),
                    "ssh_username" => config.ssh_username = value.trim().to_string(),
                    "ssh_password" => config.ssh_password = value.trim().to_string(),
                    "alt_ssh_username" => config.alt_ssh_username = value.trim().to_string(),
                    "alt_ssh_password" => config.alt_ssh_password = value.trim().to_string(),
                    _ => {}
                }
            }
        }
    }
    config
}

pub fn save_config(config: &AppConfig) {
    let path = get_config_path();
    let content = format!(
        "controller_url={}\nssh_username={}\nssh_password={}\nalt_ssh_username={}\nalt_ssh_password={}\n",
        config.controller_url, config.ssh_username, config.ssh_password, config.alt_ssh_username, config.alt_ssh_password
    );
    let _ = fs::write(path, content);
}
