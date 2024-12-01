// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod database_helper;

#[tauri::command]
fn greet(name: &str) -> String
{
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run()
{
    let migrations = database_helper::get_migrations();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::new()
            .add_migrations("sqlite:database.db", migrations)
            .build()
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
