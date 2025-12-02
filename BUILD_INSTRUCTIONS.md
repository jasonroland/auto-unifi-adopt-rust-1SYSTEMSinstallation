# Building Windows Installer for UniFi Auto Adopt

This guide explains how to create a professional Windows installer (.msi) for the UniFi Auto Adopt application.

## Prerequisites

### 1. Install Rust (if not already installed)
Download and install from: https://rustup.rs/

### 2. Install WiX Toolset
Download and install WiX Toolset v3.11 or later from:
https://wixtoolset.org/releases/

**Important:** After installation, ensure the WiX bin folder is in your PATH:
- Default location: `C:\Program Files (x86)\WiX Toolset v3.11\bin`
- Verify by running `candle.exe -?` in Command Prompt

### 3. Install cargo-wix
Open Command Prompt or PowerShell and run:
```bash
cargo install cargo-wix
```

### 4. Convert Icon to ICO Format

Your icon file `Gemini_Generated_Image_1bgfa51bgfa51bgf.png` needs to be converted to `.ico` format.

**Option A: Online Converter (Easiest)**
1. Go to https://convertio.co/png-ico/ or https://www.icoconverter.com/
2. Upload `Gemini_Generated_Image_1bgfa51bgfa51bgf.png`
3. Download the converted file
4. Save it as `app_icon.ico` in the project root directory

**Option B: Using ImageMagick**
```bash
# Install ImageMagick from: https://imagemagick.org/script/download.php
# Then run:
magick convert Gemini_Generated_Image_1bgfa51bgfa51bgf.png -define icon:auto-resize=256,128,64,48,32,16 app_icon.ico
```

## Building the Installer

### Step 1: Build the Application
Open a terminal in the project directory and run:
```bash
cargo build --release
```

### Step 2: Create the MSI Installer
```bash
cargo wix --nocapture
```

The installer will be created at:
```
target\wix\auto-unifi-adopt-rust-0.1.0-x86_64.msi
```

## What's Included

The installer will:
- ✅ Install the application to Program Files
- ✅ Use your custom icon for the executable
- ✅ Hide the console window (GUI-only mode)
- ✅ Create a Start Menu shortcut
- ✅ Create a Desktop shortcut
- ✅ Add proper uninstaller to Windows Settings
- ✅ Show your custom icon in Windows Explorer
- ✅ Include product information and version

## Customization

### Update Company Information
Edit `Cargo.toml` and update:
- `authors` - Your name and email
- `description` - Application description
- `license` - Your license type

### Update Installer Details
Edit `wix/main.wxs` and update:
- `Manufacturer` - Your company name
- `ARPHELPLINK` - Your support/GitHub URL

### Change Application Icon
Replace `app_icon.ico` with your own icon file (must be .ico format)

## Testing the Installer

1. Navigate to `target\wix\`
2. Double-click the `.msi` file
3. Follow the installation wizard
4. The application will be installed and shortcuts created
5. Test launching from Start Menu or Desktop

## Uninstalling

Users can uninstall via:
- Windows Settings > Apps > UniFi Auto Adopt
- Or by running the MSI installer again and choosing "Remove"

## Troubleshooting

### "candle.exe not found"
- Make sure WiX Toolset is installed
- Add WiX bin folder to your PATH environment variable
- Restart your terminal/IDE

### "app_icon.ico not found"
- Ensure the icon file is in the project root directory
- The filename must be exactly `app_icon.ico`

### Console window still appears
- Make sure you're building in release mode: `cargo build --release`
- The console window will appear in debug builds

### Application won't start
- Check that all dependencies are included
- Try running from terminal to see error messages: `target\release\auto-unifi-adopt-rust.exe`

## Building on macOS/Linux (Cross-Compilation)

To build Windows installers from macOS/Linux, you'll need to set up cross-compilation:

1. Install cross-compilation tools:
```bash
cargo install cross
```

2. Build for Windows:
```bash
cross build --release --target x86_64-pc-windows-gnu
```

Note: Creating the MSI installer still requires running `cargo wix` on a Windows machine.

## Distribution

Once built, you can distribute the `.msi` file to users. They simply:
1. Download the .msi file
2. Double-click to install
3. Follow the installation wizard
4. Launch from Start Menu or Desktop shortcut

No additional dependencies are needed!
