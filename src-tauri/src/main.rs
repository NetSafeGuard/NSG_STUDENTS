// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::sync::{Arc, Mutex};
use std::thread;
mod packet_capture;

#[tauri::command]
async fn start_capture(
    window: tauri::Window
) -> Result<(), ()> {
    let captured_domains: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let captured_domains_clone = captured_domains.clone();

    std::thread::sleep(std::time::Duration::from_secs(1));
    thread::spawn(move || {
        packet_capture::run_packet_capture(captured_domains_clone, window);
    });

    println!("Capture started");

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_capture])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
