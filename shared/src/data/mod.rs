mod task;

pub use task::Task;

use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
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

#[derive(Clone)]
pub struct TaskKindValue {
    pub name: String,
    pub color: String,
    pub db_value: i64,
}

impl From<TaskKind> for TaskKindValue {
    fn from(value: TaskKind) -> Self {
        match value {
            TaskKind::Pause => Self {
                name: "Pause".into(),
                color: "bg-gray-400".into(),
                db_value: 0,
            },
            TaskKind::Development => Self {
                name: "Development".into(),
                color: "bg-green-500".into(),
                db_value: 1,
            },
            TaskKind::CodeReview => Self {
                name: "CodeReview".into(),
                color: "bg-green-400".into(),
                db_value: 2,
            },
            TaskKind::Test => Self {
                name: "Test".into(),
                color: "bg-green-300".into(),
                db_value: 3,
            },
            TaskKind::Meeting => Self {
                name: "Meeting".into(),
                color: "bg-yellow-300".into(),
                db_value: 4,
            },
            TaskKind::DevOps => Self {
                name: "DevOps".into(),
                color: "bg-yellow-500".into(),
                db_value: 5,
            },
            TaskKind::Support => Self {
                name: "Support".into(),
                color: "bg-red-300".into(),
                db_value: 6,
            },
            TaskKind::Consulting => Self {
                name: "Consulting".into(),
                color: "bg-red-400".into(),
                db_value: 7,
            },
            TaskKind::Other => Self {
                name: "Other".into(),
                color: "bg-blue-500".into(),
                db_value: 8,
            },
        }
    }
}

impl From<&str> for TaskKind {
    fn from(value: &str) -> TaskKind {
        match value {
            "Pause" => TaskKind::Pause,
            "Development" => TaskKind::Development,
            "CodeReview" => TaskKind::CodeReview,
            "Test" => TaskKind::Test,
            "Meeting" => TaskKind::Meeting,
            "DevOps" => TaskKind::DevOps,
            "Support" => TaskKind::Support,
            "Consulting" => TaskKind::Consulting,
            _ => TaskKind::Other,
        }
    }
}
