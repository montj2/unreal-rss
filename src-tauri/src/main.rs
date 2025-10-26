#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;
mod db;
mod feed;

use tauri::Manager;

#[tokio::main]
async fn main() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            // Initialize database on app start
            let app_data_dir = app
                .path_resolver()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");

            let db_path = app_data_dir.join("unreal-rss.db");
            db::init(&db_path).expect("Failed to initialize database");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            api::add_feed,
            api::get_feeds,
            api::delete_feed,
            api::get_articles,
            api::get_article,
            api::mark_as_read,
            api::mark_as_unread,
            api::star_article,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
