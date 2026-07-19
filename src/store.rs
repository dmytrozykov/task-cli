use crate::task::*;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug)]
pub struct TaskStore {
    path: PathBuf,
    tasks: Vec<Task>,
}

#[derive(Error, Debug)]
pub enum TaskStoreError {
    #[error("Task with id {id} does not exist")]
    TaskDoesNotExist { id: u32 },
}

impl TaskStore {
    pub fn load(path: PathBuf) -> Self {
        Self {
            path,
            tasks: Vec::new(),
        }
    }

    pub fn save(&mut self) -> Result<(), TaskStoreError> {
        todo!()
    }

    pub fn add(&mut self, title: String) -> u32 {
        todo!()
    }

    pub fn rename(&mut self, id: u32, new_title: String) -> Result<(), TaskStoreError> {
        todo!()
    }

    pub fn delete(&mut self, id: i32) -> Result<(), TaskStoreError> {
        todo!()
    }

    pub fn set_status(&mut self, id: u32, status: TaskStatus) -> Result<(), TaskStoreError> {
        todo!()
    }

    pub fn get_all(&self) -> &[&Task] {
        todo!()
    }
}
