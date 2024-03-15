// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::commands::list_drives::list_drives;

pub mod commands;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![list_drives])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
