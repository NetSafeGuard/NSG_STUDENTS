// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate log;
extern crate clap;
extern crate pcap_parser;
extern crate pnet;

mod common;
mod interface;
mod pcap;
mod tls;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn start_capture(
    window: tauri::Window
) -> Result<(), ()> {

    std::thread::sleep(std::time::Duration::from_secs(1));
    window.emit("test","Capturing...").unwrap();
    interface::process_all_interfaces(window);

    Ok(())  
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_capture])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
