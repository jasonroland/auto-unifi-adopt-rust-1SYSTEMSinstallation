mod config;
mod messages;
mod models;
mod network_interface;
mod network_scanner;
mod oui_database;
mod ssh_handler;
mod styles;
mod views;

use iced::{executor, Application, Command, Element, Settings, Subscription, Theme};
use iced::widget::text_editor;
use messages::Message;
use models::{Device, DeviceStatus, SettingsTab};
use std::sync::Arc;

fn main() -> iced::Result {
    UnifiAdoption::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(700.0, 600.0),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    })
}

struct UnifiAdoption {
    ip_address: String,
    log_output: String,
    log_editor_content: text_editor::Content,
    is_running: bool,
    show_settings: bool,
    show_scan: bool,
    active_tab: SettingsTab,
    config: config::AppConfig,
    controller_url_input: String,
    username_input: String,
    password_input: String,
    alt_username_input: String,
    alt_password_input: String,
    ip_range_start: String,
    ip_range_end: String,
    devices: Vec<Device>,
    expanded_device_index: Option<usize>,
    is_scanning: bool,
    progress_receiver: Option<Arc<tokio::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<String>>>>,
    device_progress_receivers: std::collections::HashMap<usize, Arc<tokio::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<String>>>>,
}

impl Application for UnifiAdoption {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let config = config::load_config();

        // Auto-detect network and set IP range (like Angry IP Scanner)
        let (ip_range_start, ip_range_end) = if let Some(network) = network_interface::get_default_network() {
            (network.start_ip, network.end_ip)
        } else {
            (String::from("192.168.1.1"), String::from("192.168.1.254"))
        };

        let app = UnifiAdoption {
            ip_address: String::new(),
            log_output: String::new(),
            log_editor_content: text_editor::Content::new(),
            is_running: false,
            show_settings: false,
            show_scan: true,
            active_tab: SettingsTab::General,
            controller_url_input: config.controller_url.clone(),
            username_input: config.ssh_username.clone(),
            password_input: config.ssh_password.clone(),
            alt_username_input: config.alt_ssh_username.clone(),
            alt_password_input: config.alt_ssh_password.clone(),
            ip_range_start,
            ip_range_end,
            devices: Vec::new(),
            expanded_device_index: None,
            is_scanning: false,
            progress_receiver: None,
            device_progress_receivers: std::collections::HashMap::new(),
            config,
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("UniFi Adoption Wizard - Created by 1 SYSTEMS")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IpAddressChanged(value) => {
                self.ip_address = value;
                Command::none()
            }
            Message::AdoptClicked => {
                if self.is_running || self.ip_address.is_empty() {
                    return Command::none();
                }

                self.is_running = true;
                self.log_output = String::new();
                self.log_editor_content = text_editor::Content::new();

                let ip = self.ip_address.clone();
                let username = self.config.ssh_username.clone();
                let password = self.config.ssh_password.clone();
                let controller_url = self.config.controller_url.clone();

                // Create channel for progress updates
                let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                self.progress_receiver = Some(Arc::new(tokio::sync::Mutex::new(rx)));

                Command::perform(
                    async move {
                        // Run SSH in a blocking task so it doesn't block the async runtime
                        tokio::task::spawn_blocking(move || {
                            ssh_handler::execute_adoption(
                                &ip,
                                &username,
                                &password,
                                &controller_url,
                                Some(tx),
                            )
                        })
                        .await
                        .unwrap()
                        .map(|output| output)
                        .map_err(|e| format!("{}", e))
                    },
                    Message::AdoptionComplete,
                )
            }
            Message::AdoptAgainClicked => {
                if self.is_running || self.ip_address.is_empty() {
                    return Command::none();
                }

                self.is_running = true;
                self.log_output = String::new();
                self.log_editor_content = text_editor::Content::new();

                let ip = self.ip_address.clone();
                let username = self.config.alt_ssh_username.clone();
                let password = self.config.alt_ssh_password.clone();
                let controller_url = self.config.controller_url.clone();

                // Create channel for progress updates
                let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                self.progress_receiver = Some(Arc::new(tokio::sync::Mutex::new(rx)));

                Command::perform(
                    async move {
                        // Run SSH in a blocking task so it doesn't block the async runtime
                        tokio::task::spawn_blocking(move || {
                            ssh_handler::execute_adoption(
                                &ip,
                                &username,
                                &password,
                                &controller_url,
                                Some(tx),
                            )
                        })
                        .await
                        .unwrap()
                        .map(|output| output)
                        .map_err(|e| format!("{}", e))
                    },
                    Message::AdoptionComplete,
                )
            }
            Message::AdoptionComplete(result) => {
                self.is_running = false;
                self.progress_receiver = None; // Stop subscription
                match result {
                    Ok(log) => {
                        self.log_output = log;
                        self.log_editor_content = text_editor::Content::with_text(&self.log_output);
                    }
                    Err(log) => {
                        self.log_output = log;
                        self.log_editor_content = text_editor::Content::with_text(&self.log_output);
                    }
                }
                Command::none()
            }
            Message::LogUpdate(line) => {
                self.log_output.push_str(&line);
                // Also update the editor content for display
                self.log_editor_content = text_editor::Content::with_text(&self.log_output);
                Command::none()
            }
            Message::LogEditorAction(action) => {
                // Allow navigation and selection, block edits
                use text_editor::Action;
                match action {
                    // Block all editing actions (Edit, Insert, Delete, Paste, etc.)
                    Action::Edit(_) => {
                        // Silently ignore edits
                    }
                    // Allow all other actions (navigation, selection, scrolling, copy)
                    _ => {
                        self.log_editor_content.perform(action);
                    }
                }
                Command::none()
            }
            Message::SettingsClicked => {
                self.show_settings = true;
                Command::none()
            }
            Message::CloseSettings => {
                self.show_settings = false;
                Command::none()
            }
            Message::SaveSettings => {
                self.config.controller_url = self.controller_url_input.clone();
                self.config.ssh_username = self.username_input.clone();
                self.config.ssh_password = self.password_input.clone();
                self.config.alt_ssh_username = self.alt_username_input.clone();
                self.config.alt_ssh_password = self.alt_password_input.clone();
                config::save_config(&self.config);
                self.show_settings = false;
                Command::none()
            }
            Message::ControllerUrlChanged(value) => {
                self.controller_url_input = value;
                Command::none()
            }
            Message::UsernameChanged(value) => {
                self.username_input = value;
                Command::none()
            }
            Message::PasswordChanged(value) => {
                self.password_input = value;
                Command::none()
            }
            Message::AltUsernameChanged(value) => {
                self.alt_username_input = value;
                Command::none()
            }
            Message::AltPasswordChanged(value) => {
                self.alt_password_input = value;
                Command::none()
            }
            Message::TabSelected(tab) => {
                self.active_tab = tab;
                Command::none()
            }
            Message::ScanClicked => {
                self.show_scan = true;
                Command::none()
            }
            Message::CloseScan => {
                self.show_scan = false;
                Command::none()
            }
            Message::ManualEntryClicked => {
                self.show_scan = false;
                Command::none()
            }
            Message::CloseManualEntry => {
                self.show_scan = true;
                Command::none()
            }
            Message::IpRangeStartChanged(value) => {
                self.ip_range_start = value;
                Command::none()
            }
            Message::IpRangeEndChanged(value) => {
                self.ip_range_end = value;
                Command::none()
            }
            Message::ScanDevices => {
                self.is_scanning = true;
                self.devices.clear();
                self.expanded_device_index = None;

                let start = self.ip_range_start.clone();
                let end = self.ip_range_end.clone();

                Command::perform(
                    async move {
                        network_scanner::scan_network(&start, &end).await
                    },
                    Message::ScanComplete,
                )
            }
            Message::ScanComplete(result) => {
                self.is_scanning = false;
                match result {
                    Ok(devices) => {
                        self.devices = devices;
                    }
                    Err(err) => {
                        // Could add error handling here
                        eprintln!("Scan error: {}", err);
                    }
                }
                Command::none()
            }
            Message::DeviceToggled(index, checked) => {
                if let Some(device) = self.devices.get_mut(index) {
                    device.selected = checked;
                }
                Command::none()
            }
            Message::DeviceRowClicked(index) => {
                if let Some(device) = self.devices.get(index) {
                    if device.status == DeviceStatus::Pending {
                        // Toggle checkbox for pending devices
                        if let Some(device) = self.devices.get_mut(index) {
                            device.selected = !device.selected;
                        }
                    } else {
                        // Toggle expansion for devices with output
                        self.expanded_device_index = if self.expanded_device_index == Some(index) {
                            None
                        } else {
                            Some(index)
                        };
                    }
                }
                Command::none()
            }
            Message::DeviceStatusIconClicked(_index) => {
                // No longer used - row click handles expansion
                Command::none()
            }
            Message::AdoptSelectedDefault => {
                // Mark all selected devices as in progress
                for device in &mut self.devices {
                    if device.selected {
                        device.status = DeviceStatus::InProgress;
                        device.logs = String::new();
                    }
                }

                // Collect selected devices for parallel adoption
                let selected_devices: Vec<(usize, String)> = self.devices
                    .iter()
                    .enumerate()
                    .filter(|(_, d)| d.selected)
                    .map(|(i, d)| (i, d.ip.clone()))
                    .collect();

                let username = self.config.ssh_username.clone();
                let password = self.config.ssh_password.clone();
                let controller_url = self.config.controller_url.clone();

                // Clear old receivers
                self.device_progress_receivers.clear();

                // Launch parallel adoption tasks with progress channels
                let commands: Vec<Command<Message>> = selected_devices
                    .into_iter()
                    .map(|(index, ip)| {
                        let username = username.clone();
                        let password = password.clone();
                        let controller_url = controller_url.clone();

                        // Create channel for this device
                        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                        self.device_progress_receivers.insert(index, Arc::new(tokio::sync::Mutex::new(rx)));

                        Command::perform(
                            async move {
                                tokio::task::spawn_blocking(move || {
                                    ssh_handler::execute_adoption(
                                        &ip,
                                        &username,
                                        &password,
                                        &controller_url,
                                        Some(tx),
                                    )
                                })
                                .await
                                .unwrap()
                            },
                            move |result| Message::DeviceAdoptionComplete(index, result),
                        )
                    })
                    .collect();

                Command::batch(commands)
            }
            Message::AdoptSelectedAlt => {
                // Mark all selected devices as in progress
                for device in &mut self.devices {
                    if device.selected {
                        device.status = DeviceStatus::InProgress;
                        device.logs = String::new();
                    }
                }

                // Collect selected devices for parallel adoption
                let selected_devices: Vec<(usize, String)> = self.devices
                    .iter()
                    .enumerate()
                    .filter(|(_, d)| d.selected)
                    .map(|(i, d)| (i, d.ip.clone()))
                    .collect();

                let username = self.config.alt_ssh_username.clone();
                let password = self.config.alt_ssh_password.clone();
                let controller_url = self.config.controller_url.clone();

                // Clear old receivers
                self.device_progress_receivers.clear();

                // Launch parallel adoption tasks with progress channels
                let commands: Vec<Command<Message>> = selected_devices
                    .into_iter()
                    .map(|(index, ip)| {
                        let username = username.clone();
                        let password = password.clone();
                        let controller_url = controller_url.clone();

                        // Create channel for this device
                        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                        self.device_progress_receivers.insert(index, Arc::new(tokio::sync::Mutex::new(rx)));

                        Command::perform(
                            async move {
                                tokio::task::spawn_blocking(move || {
                                    ssh_handler::execute_adoption(
                                        &ip,
                                        &username,
                                        &password,
                                        &controller_url,
                                        Some(tx),
                                    )
                                })
                                .await
                                .unwrap()
                            },
                            move |result| Message::DeviceAdoptionComplete(index, result),
                        )
                    })
                    .collect();

                Command::batch(commands)
            }
            Message::DeviceAdoptionComplete(index, result) => {
                if let Some(device) = self.devices.get_mut(index) {
                    match result {
                        Ok(logs) => {
                            device.status = DeviceStatus::Success;
                            device.logs = logs;
                        }
                        Err(logs) => {
                            device.status = DeviceStatus::Error;
                            device.logs = logs;
                        }
                    }
                }
                // Clean up the receiver for this device
                self.device_progress_receivers.remove(&index);
                Command::none()
            }
            Message::DeviceLogUpdate(index, log_chunk) => {
                if let Some(device) = self.devices.get_mut(index) {
                    device.logs.push_str(&log_chunk);
                }
                Command::none()
            }
            Message::ClosePopup => {
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        if self.show_settings {
            views::settings_view(
                &self.active_tab,
                &self.controller_url_input,
                &self.username_input,
                &self.password_input,
                &self.alt_username_input,
                &self.alt_password_input,
            )
        } else if self.show_scan {
            views::scan_view(
                &self.ip_range_start,
                &self.ip_range_end,
                &self.devices,
                self.expanded_device_index,
                self.is_scanning,
            )
        } else {
            views::main_view(&self.ip_address, &self.log_editor_content, self.is_running)
        }
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut subscriptions = Vec::new();

        // Single device adoption subscription
        if let Some(rx) = &self.progress_receiver {
            let rx = Arc::clone(rx);
            let sub = iced::subscription::unfold(
                "adoption_progress",
                (rx, String::new()),
                move |(rx, mut buffer)| async move {
                    // Batch chunks for 100ms
                    let deadline = tokio::time::Instant::now() + tokio::time::Duration::from_millis(100);

                    loop {
                        match tokio::time::timeout_at(deadline, rx.lock().await.recv()).await {
                            Ok(Some(chunk)) => buffer.push_str(&chunk),
                            Ok(None) => break, // Channel closed
                            Err(_) => break,   // Timeout, send what we have
                        }
                    }

                    if !buffer.is_empty() {
                        let output = buffer.clone();
                        buffer.clear();
                        (Message::LogUpdate(output), (rx, buffer))
                    } else {
                        // Keep subscription alive even if no data
                        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                        (Message::LogUpdate(String::new()), (rx, buffer))
                    }
                },
            );
            subscriptions.push(sub);
        }

        // Device adoption subscriptions (bulk adoption)
        for (index, rx) in &self.device_progress_receivers {
            let rx = Arc::clone(rx);
            let device_index = *index;
            let sub = iced::subscription::unfold(
                format!("device_adoption_{}", device_index),
                (rx, String::new()),
                move |(rx, mut buffer)| async move {
                    // Batch chunks for 100ms
                    let deadline = tokio::time::Instant::now() + tokio::time::Duration::from_millis(100);

                    loop {
                        match tokio::time::timeout_at(deadline, rx.lock().await.recv()).await {
                            Ok(Some(chunk)) => buffer.push_str(&chunk),
                            Ok(None) => break, // Channel closed
                            Err(_) => break,   // Timeout, send what we have
                        }
                    }

                    if !buffer.is_empty() {
                        let output = buffer.clone();
                        buffer.clear();
                        (Message::DeviceLogUpdate(device_index, output), (rx, buffer))
                    } else {
                        // Keep subscription alive even if no data
                        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                        (Message::DeviceLogUpdate(device_index, String::new()), (rx, buffer))
                    }
                },
            );
            subscriptions.push(sub);
        }

        Subscription::batch(subscriptions)
    }
}
