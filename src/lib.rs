use napi::bindgen_prelude::Error;
use napi_derive::napi;
use steamworks::AppId;
use steamworks::Client;
use steamworks::SteamAPIInitError;
use std::env;
use steamworks_sys;
use winapi::um::libloaderapi::{GetModuleHandleW, GetModuleFileNameW};
use std::os::windows::ffi::{OsStringExt, OsStrExt};
use std::ffi::{OsString, OsStr};

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

    println!("Rust: Attempting to initialize with AppId: {:?}", app_id);
    let steam_client = app_id
        .map(|app_id| {
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

    // Get the path of the loaded steam_api64.dll
    let dll_name = OsStr::new("steam_api64.dll");
    let mut path_buffer: [u16; 260] = [0; 260]; // MAX_PATH = 260
    unsafe {
        let handle = GetModuleHandleW(dll_name.encode_wide().chain(std::iter::once(0)).collect::<Vec<u16>>().as_ptr());
        if handle.is_null() {
            return Err(Error::from_reason("Failed to get steam_api64.dll handle".to_string()));
        }
        let len = GetModuleFileNameW(handle, path_buffer.as_mut_ptr(), path_buffer.len() as u32);
        if len == 0 {
            return Err(Error::from_reason("Failed to get steam_api64.dll path".to_string()));
        }
        let dll_path = OsString::from_wide(&path_buffer[..len as usize]);
        println!("Rust: Found steam_api64.dll at: {:?}", dll_path);
    }

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