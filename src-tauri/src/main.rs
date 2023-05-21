// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// js 要調用的指令
#[tauri::command]
fn greet(name: &str) -> String {
    format!("hello {}!", name)
}

fn main() {
  
  println!("Message from Rust: ");


  
    tauri::Builder::default()
        // 註冊指令，註冊完前端即可調用
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
