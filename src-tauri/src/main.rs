// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::{
    fs::OpenOptions,
    env,
};

use serde_json::{json, Value};
use std::fs;
use tempfile::NamedTempFile;
use std::io::Read;
use std::io::Write;
fn main() {
    println!("Message from Rust: ");

    tauri::Builder::default()
        // 註冊指令，註冊完前端即可調用
        .invoke_handler(tauri::generate_handler![
            EventStoreToFile,
            response_event_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 回傳原始數據資料
#[tauri::command]
fn response_event_data() -> tauri::Result<String> {
    // 讀取文件
    let data = std::fs::read_to_string("./data.json")?;
    // 返回讀取到的 JSON 字串
    Ok(data)
}

// 持久化事件到本地文件
#[tauri::command]
fn EventStoreToFile(dataJson: &str) -> tauri::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("data.json")?;

    // 讀取文件
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut v = if contents.is_empty() {
        // 不存在 即建立json陣列
        json!([])
    } else {
        // 存在即讀近來
        read_json(&contents)
    };

    // 關閉
    drop(file);

    //印出
    println!("{}", dataJson);

    //  讀取傳入的json字符串 並轉換成 json
    let new_value = read_json(dataJson);
    v.as_array_mut().unwrap().push(new_value);
    let json_string = v.to_string();

    println!("{}", json_string);

    // 二次備份，兩份文件只會少一個事件
    fs::copy("data.json", "data_backup_file.json")?;

    // 創建一個臨時文件
    let mut temp_file = NamedTempFile::new()?;
    // 寫入數據
    writeln!(temp_file, "{}", json_string)?;
    // 獲取臨時文件的路徑
    let temp_path = temp_file.path().to_owned();
    // 關閉臨時文件
    // drop(temp_file);

    // 獲取絕對路徑
    let mut path = PathBuf::new();
    match env::current_dir() {
        Ok(p) => {
            println!("The current directory is {}", p.display());
            path = p;
        }
        Err(e) => {
            println!("Failed to get current directory: {}", e);
            return Err(e.into());
        }
    }
    // 存放路徑
    path.push("data.json");

    println!("The current directory is {}", path.display());
    // 將臨時文件重命名為目標文件
    std::fs::rename(&temp_path, path)?;
    Ok(())
}

// 解析json字符串
fn read_json(raw_json: &str) -> Value {
    println!("asd{}", raw_json);
    let parsed: Value = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
