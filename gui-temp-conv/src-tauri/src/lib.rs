// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn convert_to_f(value: f32) -> f32 {
    (value * 1.8f32) + 32f32
}

#[tauri::command]
fn convert_to_c(value: f32) -> f32 {
    (value - 32f32) / 1.8f32
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, convert_to_f, convert_to_c])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
