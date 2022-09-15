#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::Command;
#[tauri::command]
fn execute_command(input: String) -> String {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;
    if cfg!(target_os = "windows") {
        "Golf-CIMB cannot execute commands in Windows".into()
    } else {
        println!(
            "status: {}",
            Command::new(command)
                .args(args)
                .output()
                .expect("failed to execute process")
                .status
        );
        "Command executed".into()
    }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
