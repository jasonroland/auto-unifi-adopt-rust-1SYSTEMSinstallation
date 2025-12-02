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

    // Try to load from the OUI database file
    if let Ok(contents) = std::fs::read_to_string("oui-database.txt") {
        for line in contents.lines() {
            // Look for lines with (base 16) which contain the OUI and company name
            if line.contains("(base 16)") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let oui = parts[0].replace("-", "").to_uppercase();
                    // Extract company name (everything after the OUI and "(base 16)")
                    if let Some(pos) = line.find("(base 16)") {
                        let company = line[pos + 9..].trim().to_string();
                        if !company.is_empty() {
                            map.insert(oui, company);
                        }
                    }
                }
            }
        }
    }

    // If we couldn't load the file or it's empty, fall back to a small hardcoded list
    if map.is_empty() {
        load_fallback_database(&mut map);
    }

    map
}

fn load_fallback_database(map: &mut HashMap<String, String>) {
    // Ubiquiti
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

    // Other common manufacturers
    map.insert("001B44".to_string(), "D-Link".to_string());
    map.insert("001EC2".to_string(), "D-Link".to_string());
    map.insert("002191".to_string(), "D-Link".to_string());
    map.insert("000C42".to_string(), "Linksys".to_string());
    map.insert("001310".to_string(), "Linksys".to_string());
    map.insert("0015E9".to_string(), "Linksys".to_string());
    map.insert("005056".to_string(), "VMware".to_string());
    map.insert("000C29".to_string(), "VMware".to_string());
    map.insert("080027".to_string(), "Oracle VirtualBox".to_string());
    map.insert("00155D".to_string(), "Microsoft Hyper-V".to_string());
    map.insert("001CB3".to_string(), "Apple".to_string());
    map.insert("00236C".to_string(), "Apple".to_string());
    map.insert("B827EB".to_string(), "Raspberry Pi Foundation".to_string());
    map.insert("DCA632".to_string(), "Raspberry Pi Foundation".to_string());
}
