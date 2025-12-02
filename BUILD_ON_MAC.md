# Building Windows Executable on macOS

Since you're developing on macOS, here are your options for creating Windows builds:

## ✅ Icon Conversion (DONE)
The icon has been converted to `app_icon.ico` using ImageMagick.

## Option 1: Cross-Compile Windows .exe on Mac (No Installer)

### Setup
```bash
# Add Windows target
rustup target add x86_64-pc-windows-gnu

# Install mingw-w64 for cross-compilation
brew install mingw-w64
```

### Build Windows executable
```bash
cargo build --release --target x86_64-pc-windows-gnu
```

**Limitation:** The icon won't be embedded and the console window behavior won't work because `winres` (the build dependency) only runs on Windows.

### Distribute
The `.exe` will be at: `target/x86_64-pc-windows-gnu/release/auto-unifi-adopt-rust.exe`

Users can just run the .exe file directly (no installer needed), but:
- ❌ No custom icon
- ❌ Console window will appear
- ❌ No Start Menu/Desktop shortcuts

## Option 2: Use GitHub Actions (Recommended)

Create automated Windows builds on every commit using GitHub Actions. The builds happen on Windows runners in the cloud.

I can set this up for you - it will automatically:
- Build the Windows .exe with proper icon
- Hide console window
- Create MSI installer
- Upload as release artifacts

Would you like me to create a GitHub Actions workflow?

## Option 3: Use a Windows Environment

### A. Windows VM on Mac
- **Parallels Desktop** (paid, best performance)
- **VMware Fusion** (paid)
- **UTM** (free, ARM-based)
- **VirtualBox** (free, x86)

### B. Cloud Windows Instance
- AWS EC2 Windows instance
- Azure Windows VM
- Google Cloud Windows VM

Once in Windows:
```bash
# Install Rust
# Visit: https://rustup.rs/

# Install WiX Toolset
# Visit: https://wixtoolset.org/releases/

# Install cargo-wix
cargo install cargo-wix

# Build installer
cargo wix --nocapture
```

Result: `target\wix\auto-unifi-adopt-rust-0.1.0-x86_64.msi`

## Option 4: Simple Portable Distribution

Build a portable Windows app without installer:

1. Transfer these files to a Windows machine:
   - Your entire project folder
   - Or use GitHub to sync code

2. On Windows, run:
   ```bash
   cargo build --release
   ```

3. Distribute the contents of `target/release/`:
   - `auto-unifi-adopt-rust.exe`
   - Any `.dll` files if present

Users can just run the .exe - this will:
- ✅ Have your custom icon
- ✅ No console window
- ❌ No installer or shortcuts (manual placement)

## Recommended Approach

**For Development:** Use Option 1 (cross-compile) for quick testing

**For Distribution:** Use Option 2 (GitHub Actions) for automated professional builds, or Option 3 if you prefer local control

Would you like me to:
1. Set up GitHub Actions for automated Windows builds?
2. Create a simple portable distribution setup?
3. Update the build configuration for better cross-platform support?
