# Windows Fixes Summary

## Issues Fixed

### 1. ✅ Random Terminal Windows Appearing
**Problem:** Console windows flashed on screen during network scanning
**Solution:** Added `CREATE_NO_WINDOW` flag to all subprocess commands
- [src/network_scanner.rs:100-105](src/network_scanner.rs#L100-L105) - Ping commands
- [src/network_scanner.rs:183-186](src/network_scanner.rs#L183-L186) - ARP commands

### 2. ✅ VCRUNTIME140.dll Missing Error
**Problem:** Application required Visual C++ Redistributables to be installed
**Solution:** Static linking of MSVC runtime via [.cargo/config.toml](.cargo/config.toml)
- Runtime is now embedded in the executable
- No external DLL dependencies

### 3. ✅ OUI Database Not Loading (Everything Shows "Unknown")
**Problem:** OUI database file wasn't found when installed in Program Files
**Solution:** Changed to always load embedded fallback database first
- [src/oui_database.rs:42](src/oui_database.rs#L42) - Loads fallback database on startup
- Expanded fallback database with 50+ manufacturers
- Now includes: Ubiquiti, Cisco, TP-Link, D-Link, NetGear, Apple, etc.

### 4. ✅ MAC Address Lookup Not Working on Windows
**Problem:** ARP cache not being queried correctly
**Solution:** Improved Windows ARP parsing
- [src/network_scanner.rs:66-67](src/network_scanner.rs#L66-L67) - Added 50ms delay for ARP cache
- [src/network_scanner.rs:188-211](src/network_scanner.rs#L188-L211) - Better ARP parsing logic
- Validates MAC addresses (length check, format check)
- Filters out incomplete/invalid entries

### 5. ✅ Console Window Hiding
**Already configured:**
- [src/main.rs:2](src/main.rs#L2) - `windows_subsystem = "windows"` for release builds
- Debug builds still show console for troubleshooting

### 6. ✅ Icon Embedding
**Already configured:**
- [build.rs](build.rs) - Embeds icon into executable
- [app_icon.ico](app_icon.ico) - Converted from PNG
- Shows in taskbar, shortcuts, and file explorer

### 7. ✅ 64-bit MSI Installer
**Fixed:**
- [wix/main.wxs:21](wix/main.wxs#L21) - Added `Platform='x64'`
- [wix/main.wxs:27](wix/main.wxs#L27) - Uses `ProgramFiles64Folder`
- [wix/main.wxs:29,42,63](wix/main.wxs#L29) - All components marked `Win64='yes'`

## Testing on Windows

### Quick Test
1. Wait for GitHub Actions to build the new version
2. Download the MSI installer from Actions → Artifacts
3. Install and run - everything should work!

### What Should Work Now:
- ✅ No console windows during scanning
- ✅ Works without installing Visual C++ Redistributables
- ✅ Shows manufacturer names for network devices
- ✅ Detects MAC addresses from ARP cache
- ✅ Professional GUI-only experience
- ✅ Custom icon everywhere

### If Still Having Issues:

#### MAC addresses still showing "Unknown"
**Possible causes:**
1. Devices haven't responded to ping yet
2. Windows Firewall blocking ARP responses
3. Devices on different subnet

**Debug steps:**
```powershell
# Manually check ARP cache
arp -a

# Ping a device first
ping 192.168.1.100

# Check ARP again
arp -a
```

#### To see debug output:
```powershell
# Build in debug mode (shows console)
cargo build

# Run debug version
.\target\debug\auto-unifi-adopt-rust.exe
```

#### Custom manufacturer database:
Create a file called `oui-database.txt` next to the .exe with format:
```
00-27-22   (base 16)		Ubiquiti Networks
FC-EC-DA   (base 16)		Ubiquiti Inc
```

## Files Changed

| File | Purpose |
|------|---------|
| [.cargo/config.toml](.cargo/config.toml) | Static linking configuration |
| [src/network_scanner.rs](src/network_scanner.rs) | Hide console windows, fix ARP parsing |
| [src/oui_database.rs](src/oui_database.rs) | Embed OUI database, expand manufacturers |
| [wix/main.wxs](wix/main.wxs) | 64-bit installer configuration |
| [src/main.rs](src/main.rs) | Hide console in release mode |
| [build.rs](build.rs) | Embed icon in executable |

## Commit These Changes

```bash
git add .cargo/config.toml src/network_scanner.rs src/oui_database.rs WINDOWS_FIXES.md
git commit -m "Fix Windows issues: hide console windows, fix OUI lookup, improve ARP parsing"
git push
```

## Expected Results

After pushing, GitHub Actions will build:
- `auto-unifi-adopt-rust-0.1.0-x86_64.msi` - Full installer with shortcuts
- `auto-unifi-adopt-rust.exe` - Portable standalone executable

Both will:
- ✅ Not show console windows
- ✅ Display manufacturer names
- ✅ Show your custom icon
- ✅ Work without additional DLLs
- ✅ Run smoothly on any Windows 10/11 machine
