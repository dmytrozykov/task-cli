use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}: {} - {}", self.id, self.title, self.status)
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_symbol())
    }
}

impl TaskStatus {
    pub fn get_symbol(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "[ ]",
            TaskStatus::InProgress => "[-]",
            TaskStatus::Done => "[x]",
        }
    }
}
