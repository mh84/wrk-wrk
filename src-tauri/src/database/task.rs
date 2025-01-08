use sqlx::FromRow;

use shared::{Task as SharedTask, TaskKind};

#[derive(FromRow)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub task_kind: i64,
    pub finished: i64,
}

impl From<Task> for SharedTask {
    fn from(value: Task) -> Self {
        Self {
            id: value.id as u32,
            name: value.name,
            task_kind: match value.task_kind {
                0 => TaskKind::Pause,
                1 => TaskKind::Development,
                2 => TaskKind::CodeReview,
                3 => TaskKind::Test,
                4 => TaskKind::Meeting,
                5 => TaskKind::DevOps,
                6 => TaskKind::Support,
                7 => TaskKind::Consulting,
                _ => TaskKind::Other,
            },
            finished: matches!(value.finished, 1),
        }
    }
}
