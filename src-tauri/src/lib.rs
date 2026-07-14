mod commands;
mod db;
mod error;
mod models;

use commands::DbState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let data_dir = app_handle
                    .path()
                    .app_data_dir()
                    .expect("failed to resolve app data dir");
                let db_path = data_dir.join("kanban.db");
                let pool = db::init_pool(db_path)
                    .await
                    .expect("failed to initialize database");
                app_handle.manage(DbState(pool));
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_boards,
            commands::create_board,
            commands::get_board,
            commands::rename_board,
            commands::delete_board,
            commands::create_list,
            commands::rename_list,
            commands::delete_list,
            commands::reorder_list,
            commands::create_card,
            commands::update_card,
            commands::delete_card,
            commands::move_card,
            commands::list_labels,
            commands::create_label,
            commands::update_label,
            commands::delete_label,
            commands::set_card_labels,
            commands::create_checklist_item,
            commands::update_checklist_item,
            commands::delete_checklist_item,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
