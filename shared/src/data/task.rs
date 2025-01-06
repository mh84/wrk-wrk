use serde::{Deserialize, Serialize};

use super::TaskKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub task_kind: TaskKind,
    pub finished: bool,
}
