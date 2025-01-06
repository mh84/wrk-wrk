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
    Support = 5,
    Consulting = 6,
    Other = 7,
}
