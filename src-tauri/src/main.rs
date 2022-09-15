#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn funct(input: String) -> String {
    print!("{} ", input);
    "Function executed".into()
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![funct])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
