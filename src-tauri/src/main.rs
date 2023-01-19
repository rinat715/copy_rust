#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use jira_app::Config;

const FILENAME: &str = "jira.toml";

#[derive(Serialize)]
struct App {
    config: Config
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 
#[tauri::command]
fn get_config(state: tauri::State<App>) -> &Config {
    &state.inner().config
}

fn main() {

    let app = App{
        config: Config::from_file(FILENAME)
    };


    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![greet, get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
