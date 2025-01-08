mod task;

pub use task::Task;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum TaskKind {
    #[default]
    Pause = 0,
    Development = 1,
    CodeReview = 2,
    Test = 3,
    Meeting = 4,
    DevOps = 5,
    Support = 6,
    Consulting = 7,
    Other = 8,
}

impl From<TaskKind> for i64 {
    fn from(value: TaskKind) -> Self {
        match value {
            TaskKind::Pause => 0,
            TaskKind::Development => 1,
            TaskKind::CodeReview => 2,
            TaskKind::Test => 3,
            TaskKind::Meeting => 4,
            TaskKind::DevOps => 5,
            TaskKind::Support => 6,
            TaskKind::Consulting => 7,
            TaskKind::Other => 8,
        }
    }
}
