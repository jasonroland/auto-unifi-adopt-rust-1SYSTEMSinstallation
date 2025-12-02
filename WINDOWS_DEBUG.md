# Windows Debugging Guide

## Fixed: Random Terminal Windows

I've fixed the issue where terminal windows would randomly appear during network scanning. The problem was that `ping` and `arp` commands were spawning visible console windows.

**Changes made to [src/network_scanner.rs](src/network_scanner.rs):**
- Added `CREATE_NO_WINDOW` flag to all `tokio::process::Command` calls on Windows
- This hides console windows for ping and arp commands

## Setting Up Windows Development Environment

### Option 1: Quick Test (No Setup)
Just download the new build from GitHub Actions after pushing these changes!

### Option 2: Full Windows Development Setup

#### 1. Install Rust on Windows
```powershell
# Download and run rustup-init.exe from:
# https://rustup.rs/
```

#### 2. Install Visual Studio C++ Build Tools
```powershell
# Download from:
# https://visualstudio.microsoft.com/visual-cpp-build-tools/

# During installation, select "Desktop development with C++"
```

#### 3. Clone Your Repository
```powershell
git clone https://github.com/yourusername/auto-unifi-adopt-rust.git
cd auto-unifi-adopt-rust
```

#### 4. Build and Run
```powershell
# Build release version (no console window)
cargo build --release

# Run it
.\target\release\auto-unifi-adopt-rust.exe

# Build debug version (WITH console window for debugging)
cargo build
.\target\debug\auto-unifi-adopt-rust.exe
```

## Debugging Tips

### Show Console in Debug Mode
The app is configured to:
- ✅ **Release builds** (`cargo build --release`): NO console window
- ✅ **Debug builds** (`cargo build`): HAS console window for debugging

This is controlled by this line in [src/main.rs](src/main.rs):
```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

### Common Issues and Solutions

#### Issue: Still seeing console windows
**Solution:** Make sure you're building in release mode:
```powershell
cargo build --release
```

#### Issue: App crashes silently
**Solution:** Run in debug mode to see error messages:
```powershell
cargo build
.\target\debug\auto-unifi-adopt-rust.exe
```

Or redirect errors to a file in release mode:
```powershell
.\target\release\auto-unifi-adopt-rust.exe 2> error.log
```

#### Issue: VCRUNTIME140.dll missing
**Solution:** This should be fixed by static linking in [.cargo/config.toml](.cargo/config.toml)

If still having issues:
```powershell
# Install Visual C++ Redistributables
# Download from: https://aka.ms/vs/17/release/vc_redist.x64.exe
```

### Logging for Debugging

To add debug logging, you can use the `env_logger` crate:

1. Add to `Cargo.toml`:
```toml
[dependencies]
env_logger = "0.11"
log = "0.4"
```

2. In `main.rs`:
```rust
fn main() -> iced::Result {
    // Initialize logger only in debug mode
    #[cfg(debug_assertions)]
    env_logger::init();

    // ... rest of your code
}
```

3. Use logging:
```rust
log::info!("Starting network scan");
log::error!("Connection failed: {}", error);
```

### Testing the Installer

1. Build the MSI:
```powershell
# Install WiX Toolset first from: https://wixtoolset.org/releases/
# Then:
cargo install cargo-wix
cargo wix --nocapture
```

2. Test the installer:
```powershell
# Find the MSI in:
.\target\wix\auto-unifi-adopt-rust-0.1.0-x86_64.msi
```

### Remote Debugging via Windows VM

If you're using Parallels/VMware on Mac:

1. **Share folder** between Mac and Windows VM
2. **Edit on Mac**, build on Windows VM
3. **Enable shared clipboard** for easy error copying

## What Was Fixed

### Before:
- Ping commands showed console windows
- ARP commands showed console windows
- Jarring user experience with flashing windows

### After:
- All subprocess commands hidden
- Clean GUI-only experience
- Professional Windows app behavior

## Next Steps

1. Commit these changes:
```bash
git add src/network_scanner.rs WINDOWS_DEBUG.md
git commit -m "Fix random terminal windows on Windows"
git push
```

2. Download the new build from GitHub Actions

3. Test on Windows - no more random terminal windows! ✅
