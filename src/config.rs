use std::fs;

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

pub fn load_config() -> AppConfig {
    let path = "config.txt";
    if !std::path::Path::new(path).exists() {
        let config = AppConfig::default();
        save_config(&config);
        return config;
    }

    let mut config = AppConfig::default();
    if let Ok(contents) = fs::read_to_string(path) {
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
    let content = format!(
        "controller_url={}\nssh_username={}\nssh_password={}\nalt_ssh_username={}\nalt_ssh_password={}\n",
        config.controller_url, config.ssh_username, config.ssh_password, config.alt_ssh_username, config.alt_ssh_password
    );
    let _ = fs::write("config.txt", content);
}
