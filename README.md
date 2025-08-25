# steamworkswinx64-withlogs

A modified version of [steamworks.js](https://github.com/ceifa/steamworks.js) with additional logging for Windows x64, including environment details, DLL loading, and Steam API initialization. This module is designed for developers working on Windows x64 environments who need robust Steamworks functionality with detailed diagnostic logs.

## Features 

- **Enhanced Logging**: Uses winapi to log the exact path of the loaded steam_api64.dll, along with environment details and Steam API initialization, to diagnose issues like [ERR_DLOPEN_FAILED] (e.g., Rust: Found steam_api64.dll at: "C:\...\node_modules\steamworkswinx64-withlogs\dist\win64\steam_api64.dll", Rust: SteamAPI_Init succeeded).
- **Windows x64 Only**: Optimized exclusively for Windows x64, ensuring compatibility with `steamworksjs.win32-x64-msvc.node` and the customized `steam_api64.dll`.
- **Pre-Built Binaries**: Ships with all necessary files, including `dist\win64\steam_api64.dll`, `steam_api64.lib`, and `steamworksjs.win32-x64-msvc.node`, so no `npm install` is required for dependencies to use the module in your project.
- **Rebuild Support**: Includes source files (`src`, `Cargo.toml`, `build.js`, `sdk/redistributable_bin`) for users who want to rebuild the module using `npm run build`, though this is optional as pre-built binaries are provided.
- **Steam Overlay Support**: Retains `electronEnableSteamOverlay` for potential future use, though not currently recommended due to known issues (not enabled by default in example usage).
- **Full Steamworks API**: Provides access to all `steamworks.js` features, including achievements, matchmaking, networking, and more, as detailed in the [steamworks.js documentation](https://github.com/ceifa/steamworks.js).

## Installation 

The module includes all required files for immediate use on Windows x64. No additional npm install is needed to use the module, as it includes pre-built binaries. The pre-built dist\win64 files (dist\win64\steam_api64.dll, dist\win64\steam_api64.lib, dist\win64\steamworksjs.win32-x64-msvc.node) ensure the module works immediately on Windows x64 without additional installations for runtime use. Build is also not needed. 

To install in your project:

```bash
npm install steamworkswinx64-withlogs
```

## Usage

```bash

const steamworks = require('steamworkswinx64-withlogs');

// Initialize Steamworks with your App ID
const steam = steamworks.init(XXXXXXX); // Replace with your Steam App ID

// Example: Unlock an achievement
steam.achievement.activate('ACH_TEST');
console.log('Activated achievement: ACH_TEST');
console.log('Achievement ACH_TEST status:', steam.achievement.isActivated('ACH_TEST') ? 'Unlocked' : 'Locked');

// Note: Avoid calling steamworks.electronEnableSteamOverlay() until overlay issues are resolved

```