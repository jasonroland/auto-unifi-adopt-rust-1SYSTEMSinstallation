#[derive(Debug, Clone, PartialEq)]
pub enum SettingsTab {
    General,
    DefaultCredentials,
    AlternativeCredentials,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceStatus {
    Pending,
    InProgress,
    Success,
    Error,
}

#[derive(Debug, Clone)]
pub struct Device {
    pub ip: String,
    pub mac: String,
    pub company: String,
    pub selected: bool,
    pub status: DeviceStatus,
    pub logs: String,
    pub has_ssh: bool,
}
