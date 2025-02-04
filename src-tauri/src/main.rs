#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
mod onnx_runner;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![onnx_runner::run_inference])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
