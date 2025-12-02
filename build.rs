fn main() {
    // Only compile Windows-specific resources on Windows
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("app_icon.ico");
        res.set("ProductName", "UniFi Auto Adopt");
        res.set("FileDescription", "Automatic UniFi Device Adoption Tool");
        res.set("CompanyName", "1 SYSTEMS installation");
        res.set("LegalCopyright", "Copyright © 2025");

        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resources: {}", e);
        }
    }
}
