use crate::task::*;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug)]
pub struct Store {
    path: PathBuf,
    tasks: Vec<Task>,
}

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Task with id {id} does not exist")]
    TaskDoesNotExist { id: u32 },
}

impl Store {
    pub fn load(path: PathBuf) -> Self {
        Self {
            path,
            tasks: Vec::new(),
        }
    }

    pub fn save(&mut self) -> Result<(), StoreError> {
        todo!()
    }

    pub fn add_task(&mut self, title: String) -> u32 {
        todo!()
    }

    pub fn update_task(&mut self, id: u32, title: String) -> Result<(), StoreError> {
        todo!()
    }

    pub fn delete_task(&mut self, id: i32) -> Result<(), StoreError> {
        todo!()
    }

    pub fn task_set_status(&mut self, id: u32, status: TaskStatus) -> Result<(), StoreError> {
        todo!()
    }

    pub fn get_tasks(&mut self) -> &[Task] {
        todo!()
    }
}
