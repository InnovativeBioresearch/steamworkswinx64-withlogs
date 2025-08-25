use napi::bindgen_prelude::Error;
use napi_derive::napi;
use steamworks::AppId;
use steamworks::Client;
use steamworks::SteamAPIInitError;
use std::env;
use std::path::Path;
use steamworks_sys; // Added for SteamAPI_IsSteamRunning

pub mod client;

#[macro_use]
extern crate lazy_static;

#[napi]
pub fn init(app_id: Option<u32>) -> Result<(), Error> {
    // Log environment details
    println!("Rust: Current directory: {:?}", env::current_dir().unwrap_or_default());
    println!("Rust: PATH: {:?}", env::var("PATH").unwrap_or_default());

    // Check Steam running status
    unsafe {
        println!("Rust: Is Steam running? {}", steamworks_sys::SteamAPI_IsSteamRunning());
    }

    // Check for steam_api64.dll in multiple relative locations
    let possible_paths = vec![
        Path::new("steam_api64.dll"), // Current working directory
        Path::new("./node_modules/electron/dist/steam_api64.dll"), // Electron dist folder
        Path::new("../my-game-electron/steam_api64.dll"), // Project root
        Path::new("../my-game-electron/node_modules/electron/dist/steam_api64.dll"), // Project Electron dist
        Path::new("./node_modules/steamworks.js/dist/win64/steam_api64.dll"), // Bundled DLL
    ];

    for path in possible_paths {
        if path.exists() {
            println!("Rust: Found steam_api64.dll at: {:?}", path.canonicalize().unwrap_or_default());
        } else {
            println!("Rust: steam_api64.dll not found at: {:?}", path);
        }
    }

    if client::has_client() {
        client::drop_client();
    }

    let steam_client = app_id
        .map(|app_id| {
            println!("Rust: Attempting to initialize with AppId: {}", app_id);
            Client::init_app(AppId(app_id))
        })
        .unwrap_or_else(|| {
            println!("Rust: Attempting to initialize without AppId");
            Client::init()
        })
        .map_err(|e| {
            let error_msg = match &e {
                SteamAPIInitError::FailedGeneric(msg) => format!("FailedGeneric: {}", msg),
                SteamAPIInitError::NoSteamClient(msg) => format!("NoSteamClient: {}", msg),
                SteamAPIInitError::VersionMismatch(msg) => format!("VersionMismatch: {}", msg),
            };
            println!("Rust: SteamAPI_Init failed with error: {}", error_msg);
            Error::from_reason(error_msg)
        })?;

    steam_client.user_stats().request_current_stats();
    // Log overlay status
    println!("Rust: Overlay enabled? {}", steam_client.utils().is_overlay_enabled());

    client::set_client(steam_client);
    println!("Rust: SteamAPI_Init succeeded");
    Ok(())
}

#[napi]
pub fn restart_app_if_necessary(app_id: u32) -> bool {
    steamworks::restart_app_if_necessary(AppId(app_id))
}

#[napi]
pub fn run_callbacks() {
    client::get_client().run_callbacks();
}

pub mod api;