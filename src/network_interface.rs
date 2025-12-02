use get_if_addrs::{get_if_addrs, IfAddr};
use std::net::Ipv4Addr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkInterface {
    pub name: String,
    pub ip: String,
    pub start_ip: String,
    pub end_ip: String,
    pub cidr: String,
}

impl std::fmt::Display for NetworkInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} ({})", self.name, self.ip, self.cidr)
    }
}

pub fn get_local_networks() -> Vec<NetworkInterface> {
    let mut networks = Vec::new();

    if let Ok(interfaces) = get_if_addrs() {
        for iface in interfaces {
            if let IfAddr::V4(addr) = iface.addr {
                // Skip loopback interfaces (127.0.0.1)
                if addr.ip.is_loopback() {
                    continue;
                }

                // Calculate network range from IP and netmask
                let (start_ip, end_ip, cidr) = calculate_network_range(addr.ip, addr.netmask);

                networks.push(NetworkInterface {
                    name: iface.name,
                    ip: addr.ip.to_string(),
                    start_ip,
                    end_ip,
                    cidr,
                });
            }
        }
    }

    networks
}

fn calculate_network_range(ip: Ipv4Addr, netmask: Ipv4Addr) -> (String, String, String) {
    let ip_u32 = u32::from(ip);
    let mask_u32 = u32::from(netmask);

    // Calculate network address (IP AND netmask)
    let network_u32 = ip_u32 & mask_u32;

    // Calculate broadcast address (network OR inverted netmask)
    let broadcast_u32 = network_u32 | !mask_u32;

    // Start IP is network address + 1 (skip network address itself)
    let start_u32 = network_u32 + 1;

    // End IP is broadcast - 1 (skip broadcast address itself)
    let end_u32 = broadcast_u32 - 1;

    // Convert back to IP addresses
    let start_ip = Ipv4Addr::from(start_u32).to_string();
    let end_ip = Ipv4Addr::from(end_u32).to_string();

    // Calculate CIDR notation (count the number of 1s in netmask)
    let cidr_bits = mask_u32.count_ones();
    let cidr = format!("{}/{}", Ipv4Addr::from(network_u32), cidr_bits);

    (start_ip, end_ip, cidr)
}

pub fn get_default_network() -> Option<NetworkInterface> {
    let networks = get_local_networks();

    // Prefer non-virtual interfaces and common home network ranges
    // Priority: 192.168.x.x > 10.x.x.x > 172.16-31.x.x > others

    // First, try to find a 192.168.x.x network
    for net in &networks {
        if net.ip.starts_with("192.168.") {
            return Some(net.clone());
        }
    }

    // Then try 10.x.x.x
    for net in &networks {
        if net.ip.starts_with("10.") {
            return Some(net.clone());
        }
    }

    // Then try 172.16-31.x.x
    for net in &networks {
        if net.ip.starts_with("172.") {
            if let Some(second_octet) = net.ip.split('.').nth(1) {
                if let Ok(octet) = second_octet.parse::<u8>() {
                    if (16..=31).contains(&octet) {
                        return Some(net.clone());
                    }
                }
            }
        }
    }

    // Return first available network
    networks.first().cloned()
}
