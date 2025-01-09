use anyhow::Result;
use tauri::State;

use crate::database::PoolProvider;

#[tauri::command]
pub async fn add_task(
    name: String,
    taskkind: i64,
    state: State<'_, PoolProvider>,
) -> Result<String, String> {
    let pool = &state.0;

    let _ = sqlx::query(
        r#"
            INSERT INTO tasks (name, task_kind, finished)
            VALUES (?, ?, ?)
        "#,
    )
    .bind(name)
    .bind(taskkind)
    .bind(0)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(String::new())
}
