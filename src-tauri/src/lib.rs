mod commands;
mod database;

use tauri::Manager;

use commands::*;
use database::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default().plugin(tauri_plugin_opener::init());

    let app = builder.setup(|app| {
        tauri::async_runtime::block_on(async move {
            let handle = app.handle();
            let database = Database::new(handle)
                .await
                .expect("failed to initialize database");

            app.manage(PoolProvider(database.pool));
        });

        Ok(())
    });

    app.invoke_handler(tauri::generate_handler![get_tasks, add_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
