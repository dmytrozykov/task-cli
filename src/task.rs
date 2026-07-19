use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}
