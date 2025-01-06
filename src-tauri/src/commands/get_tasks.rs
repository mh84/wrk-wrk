use anyhow::Result;
use tauri::State;

use crate::database::{PoolProvider, Task};
use shared::Task as SharedTask;

#[tauri::command]
pub async fn get_tasks(
    pattern: String,
    finished: bool,
    state: State<'_, PoolProvider>,
) -> Result<Vec<SharedTask>, String> {
    let no_pattern = pattern.is_empty();
    let lower = pattern.to_lowercase();

    let pool = &state.0;

    let tasks = sqlx::query_as::<_, Task>(
        r#"
            SELECT *
            FROM tasks
            WHERE task_kind >= 0 AND finished = ?
        "#,
    )
    .bind(finished)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?
    .into_iter()
    .filter(|t| no_pattern || t.name.to_lowercase().contains(&lower))
    .map(|t| t.into())
    .collect();

    Ok(tasks)
}
