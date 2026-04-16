mod commands;
mod models;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::fs;
use std::sync::Mutex;
use tauri::Manager;

pub struct AppState {
    pub db: Mutex<SqlitePool>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            fs::create_dir_all(&app_data_dir)?;
            let db_path = app_data_dir.join("markitdown.db");

            tauri::async_runtime::block_on(async {
                let options = SqliteConnectOptions::new()
                    .filename(&db_path)
                    .create_if_missing(true);

                let pool = SqlitePoolOptions::new()
                    .connect_with(options)
                    .await
                    .expect("Failed to connect to database");

                sqlx::query(include_str!("../migrations/001_init.sql"))
                    .execute(&pool)
                    .await
                    .expect("Failed to run migrations");

                app.manage(AppState {
                    db: Mutex::new(pool),
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::convert_file,
            commands::batch_convert,
            commands::get_history,
            commands::delete_history,
            commands::open_file_dialog,
            commands::open_folder_dialog,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
