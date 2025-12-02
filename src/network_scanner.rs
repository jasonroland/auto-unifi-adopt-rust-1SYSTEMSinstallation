use std::time::Duration;
use crate::models::Device;
use crate::models::DeviceStatus;

pub async fn scan_network(start_ip: &str, end_ip: &str) -> Result<Vec<Device>, String> {
    let start = parse_ip(start_ip)?;
    let end = parse_ip(end_ip)?;

    let mut devices = Vec::new();
    let mut handles = Vec::new();

    for ip_num in start..=end {
        let ip = format!("{}.{}.{}.{}",
            (ip_num >> 24) & 0xFF,
            (ip_num >> 16) & 0xFF,
            (ip_num >> 8) & 0xFF,
            ip_num & 0xFF
        );

        let handle = tokio::spawn(async move {
            check_device(ip).await
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Ok(Some(device)) = handle.await {
            devices.push(device);
        }
    }

    Ok(devices)
}

fn parse_ip(ip_str: &str) -> Result<u32, String> {
    let parts: Vec<&str> = ip_str.split('.').collect();
    if parts.len() != 4 {
        return Err(format!("Invalid IP address: {}", ip_str));
    }

    let mut ip_num: u32 = 0;
    for (i, part) in parts.iter().enumerate() {
        let octet = part.parse::<u32>()
            .map_err(|_| format!("Invalid IP octet: {}", part))?;
        if octet > 255 {
            return Err(format!("IP octet out of range: {}", octet));
        }
        ip_num |= octet << (24 - i * 8);
    }

    Ok(ip_num)
}

async fn check_device(ip: String) -> Option<Device> {
    // First, try to ping the device to see if it's alive
    let is_alive = ping_device(&ip).await;

    if !is_alive {
        return None;
    }

    // Device is alive, check if it has SSH
    let has_ssh = check_ssh(&ip).await;

    // On Windows, give a tiny delay for ARP cache to populate after ping
    #[cfg(target_os = "windows")]
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Try to get MAC address via ARP or use placeholder
    let mac = get_mac_address(&ip).await.unwrap_or_else(|| String::from("Unknown"));
    let company = crate::oui_database::get_manufacturer(&mac);

    Some(Device {
        ip,
        mac,
        company,
        selected: false,
        status: DeviceStatus::Pending,
        logs: String::new(),
        has_ssh,
    })
}

async fn ping_device(ip: &str) -> bool {
    #[cfg(target_os = "macos")]
    let ping_cmd = "ping";

    #[cfg(target_os = "linux")]
    let ping_cmd = "ping";

    #[cfg(target_os = "windows")]
    let ping_cmd = "ping";

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    let args = vec!["-c", "1", "-W", "1", ip];

    #[cfg(target_os = "windows")]
    let args = vec!["-n", "1", "-w", "1000", ip];

    let mut cmd = tokio::process::Command::new(ping_cmd);
    cmd.args(&args);

    // Hide console window on Windows
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    match cmd.output().await {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

async fn check_ssh(ip: &str) -> bool {
    let addr = format!("{}:22", ip);

    match tokio::time::timeout(
        Duration::from_millis(500),
        tokio::net::TcpStream::connect(&addr)
    ).await {
        Ok(Ok(_)) => true,
        _ => false,
    }
}

async fn get_mac_address(ip: &str) -> Option<String> {
    // Try to get MAC address from ARP cache
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = tokio::process::Command::new("arp")
            .arg("-n")
            .arg(ip)
            .output()
            .await
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Parse ARP output to extract MAC address
            for line in stdout.lines() {
                if line.contains(ip) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let mac = parts[3].to_string();
                        if mac.contains(':') {
                            return Some(mac.to_uppercase());
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = tokio::process::Command::new("ip")
            .arg("neigh")
            .arg("show")
            .output()
            .await
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains(ip) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for (i, part) in parts.iter().enumerate() {
                        if *part == "lladdr" && i + 1 < parts.len() {
                            return Some(parts[i + 1].to_uppercase());
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Try with full ARP table first (more reliable on Windows)
        let mut cmd = tokio::process::Command::new("arp");
        cmd.arg("-a");

        // Hide console window on Windows
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);

        if let Ok(output) = cmd.output().await {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Windows ARP output format:
            // Internet Address      Physical Address      Type
            // 192.168.1.1           aa-bb-cc-dd-ee-ff     dynamic
            for line in stdout.lines() {
                // Look for line containing our IP
                if line.contains(ip) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    // parts[0] = IP, parts[1] = MAC, parts[2] = type
                    if parts.len() >= 2 {
                        let mac_str = parts[1];
                        // Check if it's a valid MAC (not "incomplete" or other status)
                        if mac_str.contains('-') || mac_str.contains(':') {
                            let mac = mac_str.replace("-", ":");
                            // Filter out incomplete or invalid MACs
                            if mac.len() >= 17 {  // AA:BB:CC:DD:EE:FF is 17 chars
                                return Some(mac.to_uppercase());
                            }
                        }
                    }
                }
            }
        }
    }

    None
}
