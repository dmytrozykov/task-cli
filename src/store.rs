use std::{fs, io, path::PathBuf};

use thiserror::Error;

use crate::task::{Task, TaskStatus};

#[derive(Debug)]
pub struct TaskStore {
    path: PathBuf,
    tasks: Vec<Task>,
}

#[derive(Error, Debug)]
pub enum TaskStoreError {
    #[error("Task not found: {0}")]
    TaskNotFound(u32),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

impl TaskStore {
    pub fn load(path: impl Into<PathBuf>) -> Result<Self, TaskStoreError> {
        let path = path.into();

        match fs::read_to_string(&path) {
            Ok(json) => Ok(Self {
                path,
                tasks: serde_json::from_str(&json)?,
            }),

            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(Self {
                path,
                tasks: Vec::new(),
            }),

            Err(err) => Err(err.into()),
        }
    }

    pub fn save(&self) -> Result<(), TaskStoreError> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    pub fn add(&mut self, title: String) -> u32 {
        let id = self.get_next_id();
        self.tasks.push(Task {
            id,
            title,
            status: TaskStatus::Todo,
        });
        id
    }

    pub fn rename(&mut self, id: u32, title: String) -> Result<(), TaskStoreError> {
        let Some(task) = self.get_task_mut(id) else {
            return Err(TaskStoreError::TaskNotFound(id));
        };

        task.title = title;
        Ok(())
    }

    pub fn delete(&mut self, id: u32) -> Result<(), TaskStoreError> {
        let old_len = self.tasks.len();

        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() == old_len {
            return Err(TaskStoreError::TaskNotFound(id));
        }

        Ok(())
    }

    pub fn set_status(&mut self, id: u32, status: TaskStatus) -> Result<(), TaskStoreError> {
        let Some(task) = self.get_task_mut(id) else {
            return Err(TaskStoreError::TaskNotFound(id));
        };

        task.status = status;
        Ok(())
    }

    pub fn get_all(&self) -> &[Task] {
        &self.tasks
    }

    fn get_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    fn get_next_id(&self) -> u32 {
        self.tasks.iter().map(|t| t.id).max().map_or(1, |m| m + 1)
    }
}
