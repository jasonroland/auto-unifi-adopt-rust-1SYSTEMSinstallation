use std::collections::HashMap;
use std::sync::OnceLock;

static OUI_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn get_manufacturer(mac: &str) -> String {
    let oui_map = OUI_MAP.get_or_init(|| load_oui_database());

    if mac == "Unknown" {
        return String::from("Unknown");
    }

    // Extract OUI (first 3 octets) from MAC address
    // Normalize each octet to 2 digits by padding with leading zero if needed
    let oui = mac
        .split(':')
        .take(3)
        .map(|s| {
            let s = s.to_uppercase();
            if s.len() == 1 {
                format!("0{}", s)
            } else {
                s
            }
        })
        .collect::<Vec<_>>()
        .join("");

    // Look up manufacturer in database
    if let Some(manufacturer) = oui_map.get(&oui) {
        manufacturer.clone()
    } else {
        format!("Unknown ({})", mac.split(':').take(3).collect::<Vec<&str>>().join(":"))
    }
}

fn load_oui_database() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // Always load the fallback database first (embedded in binary)
    // This ensures it works regardless of installation location
    load_fallback_database(&mut map);

    // Optional: Try to load additional entries from external file if present
    // This allows users to add custom OUI entries without recompiling
    if let Ok(contents) = std::fs::read_to_string("oui-database.txt") {
        for line in contents.lines() {
            // Look for lines with (base 16) or (hex) which contain the OUI and company name
            if line.contains("(base 16)") || line.contains("(hex)") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let oui = parts[0].replace("-", "").to_uppercase();
                    // Extract company name (everything after the OUI and type marker)
                    let marker = if line.contains("(base 16)") { "(base 16)" } else { "(hex)" };
                    if let Some(pos) = line.find(marker) {
                        let company = line[pos + marker.len()..].trim().to_string();
                        if !company.is_empty() {
                            map.insert(oui, company);
                        }
                    }
                }
            }
        }
    }

    map
}

fn load_fallback_database(map: &mut HashMap<String, String>) {
    // Ubiquiti (all known OUIs)
    map.insert("002722".to_string(), "Ubiquiti Networks".to_string());
    map.insert("FCECDA".to_string(), "Ubiquiti Inc".to_string());
    map.insert("B4FBE4".to_string(), "Ubiquiti Inc".to_string());
    map.insert("74ACB9".to_string(), "Ubiquiti Networks".to_string());
    map.insert("0418D6".to_string(), "Ubiquiti Networks".to_string());
    map.insert("DC9FDB".to_string(), "Ubiquiti Inc".to_string());
    map.insert("68D79A".to_string(), "Ubiquiti Networks".to_string());
    map.insert("802AA8".to_string(), "Ubiquiti Inc".to_string());
    map.insert("F09FC2".to_string(), "Ubiquiti Networks".to_string());
    map.insert("18E829".to_string(), "Ubiquiti Networks".to_string());
    map.insert("44D9E7".to_string(), "Ubiquiti Networks".to_string());
    map.insert("687251".to_string(), "Ubiquiti Networks".to_string());
    map.insert("24A43C".to_string(), "Ubiquiti Networks".to_string());
    map.insert("E063DA".to_string(), "Ubiquiti Inc".to_string());
    map.insert("78453C".to_string(), "Ubiquiti Inc".to_string());
    map.insert("788A20".to_string(), "Ubiquiti Inc".to_string());
    map.insert("D0217C".to_string(), "Ubiquiti Inc".to_string());
    map.insert("A42BB0".to_string(), "Ubiquiti Inc".to_string());

    // Common network equipment manufacturers
    map.insert("001B44".to_string(), "D-Link".to_string());
    map.insert("001EC2".to_string(), "D-Link".to_string());
    map.insert("002191".to_string(), "D-Link".to_string());
    map.insert("000C42".to_string(), "Linksys".to_string());
    map.insert("001310".to_string(), "Linksys".to_string());
    map.insert("0015E9".to_string(), "Linksys".to_string());
    map.insert("00145E".to_string(), "TP-Link".to_string());
    map.insert("0C80DA".to_string(), "TP-Link".to_string());
    map.insert("A07A0C".to_string(), "TP-Link".to_string());
    map.insert("002686".to_string(), "Cisco".to_string());
    map.insert("000D3A".to_string(), "Cisco".to_string());
    map.insert("001644".to_string(), "Cisco Linksys".to_string());
    map.insert("00055D".to_string(), "NetGear".to_string());
    map.insert("001B2F".to_string(), "NetGear".to_string());
    map.insert("0009B7".to_string(), "NetGear".to_string());

    // Virtualization
    map.insert("005056".to_string(), "VMware".to_string());
    map.insert("000C29".to_string(), "VMware".to_string());
    map.insert("080027".to_string(), "Oracle VirtualBox".to_string());
    map.insert("00155D".to_string(), "Microsoft Hyper-V".to_string());

    // Common devices
    map.insert("001CB3".to_string(), "Apple".to_string());
    map.insert("00236C".to_string(), "Apple".to_string());
    map.insert("3C0754".to_string(), "Apple".to_string());
    map.insert("B827EB".to_string(), "Raspberry Pi Foundation".to_string());
    map.insert("DCA632".to_string(), "Raspberry Pi Foundation".to_string());
    map.insert("E45F01".to_string(), "Raspberry Pi Trading".to_string());
    map.insert("001EC0".to_string(), "Intel Corporate".to_string());
    map.insert("00215C".to_string(), "Intel Corporate".to_string());
    map.insert("0026C7".to_string(), "Intel Corporate".to_string());
}
