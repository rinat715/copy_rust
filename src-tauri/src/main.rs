#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use jira_app::config::Config;
use jira_app::Client;
use jira_app::worklog::WorklogResponse;


const FILENAME: &str = "jira.toml";

#[derive(Serialize)]
struct App {
    config: Config
}

#[derive(Serialize)]
struct ErrorMessage {
    message: String,

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


#[tauri::command]
// Arguments should be passed as a JSON object with camelCase keys
// js: let res = await invoke("get_worklogs", { startDate: "2023-01-16", endDate: "2023-01-17" })
async fn get_worklogs(state: tauri::State<'_, App>, start_date: &str, end_date: &str) -> Result<WorklogResponse, ErrorMessage> {
    let vendor = state.inner().config.get_vendor();
    let client = match Client::build(vendor){
        Ok(client) => client,
        Err(error) => return Err(ErrorMessage {message: format!("Error {}", error)}),
    };

    match client.get_worklogs(start_date, end_date).await {
        Ok(res) => return Ok(res),
        Err(error) => return Err(ErrorMessage {message: format!("Error {}", error)}),
    };
}


fn main() {

    let app = App{
        config: Config::from_file(FILENAME)
    };


    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![greet, get_config, get_worklogs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
