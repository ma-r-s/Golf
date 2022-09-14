use std::process::Command;
#[tauri::command]
fn execute_command(input: String) {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;
    let output = Command::new(command).args(args).spawn().unwrap();
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
