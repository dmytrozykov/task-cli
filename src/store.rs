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
    #[error("Task title must not be empty")]
    EmptyTitle,
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

impl TaskStore {
    pub fn load(path: impl Into<PathBuf>) -> Result<Self, TaskStoreError> {
        let path = path.into();

        match fs::read_to_string(&path) {
            Ok(json) if json.trim().is_empty() => Ok(Self {
                path,
                tasks: Vec::new(),
            }),

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
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    pub fn add(&mut self, title: String) -> Result<u32, TaskStoreError> {
        let title = Self::validate_title(title)?;

        let id = self.get_next_id();
        self.tasks.push(Task {
            id,
            title,
            status: TaskStatus::Todo,
        });
        Ok(id)
    }

    pub fn rename(&mut self, id: u32, title: String) -> Result<(), TaskStoreError> {
        let title = Self::validate_title(title)?;

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

    fn validate_title(title: String) -> Result<String, TaskStoreError> {
        let title = title.trim().to_string();
        if title.is_empty() {
            Err(TaskStoreError::EmptyTitle)
        } else {
            Ok(title)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;

    use tempfile::NamedTempFile;

    fn temp_store() -> TaskStore {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();

        drop(file); // remove the empty file so load() takes the NotFound branch

        TaskStore::load(path).unwrap()
    }

    #[test]
    fn load_missing_file_starts_empty() {
        let store = temp_store();
        assert!(store.get_all().is_empty());
    }

    #[test]
    fn add_creates_a_todo_task() {
        let mut store = temp_store();
        let id = store.add("Buy groceries".to_string()).unwrap();

        let tasks = store.get_all();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, id);
        assert_eq!(tasks[0].title, "Buy groceries".to_string());
        assert_eq!(tasks[0].status, TaskStatus::Todo);
    }

    #[test]
    fn add_task_with_empty_title() {
        let mut store = temp_store();
        assert!(matches!(
            store.add("".to_string()),
            Err(TaskStoreError::EmptyTitle)
        ));
        assert!(matches!(
            store.add(" ".to_string()),
            Err(TaskStoreError::EmptyTitle)
        ));
        assert!(matches!(
            store.add(" \n".to_string()),
            Err(TaskStoreError::EmptyTitle)
        ));
    }

    #[test]
    fn rename_updates_existing_task() {
        let mut store = temp_store();
        let id = store.add("Buy groceries".to_string()).unwrap();

        store.rename(id, "Buy tomatoes".to_string()).unwrap();

        assert_eq!(store.get_all()[0].title, "Buy tomatoes".to_string());
    }

    #[test]
    fn rename_non_existing_task_returns_error() {
        let mut store = temp_store();

        let res = store.rename(1, "Buy tomatoes".to_string());

        assert!(matches!(res, Err(TaskStoreError::TaskNotFound(1))));
    }

    #[test]
    fn delete_removes_task() {
        let mut store = temp_store();
        let id = store.add("Buy groceries".to_string()).unwrap();

        store.delete(id).unwrap();

        assert!(store.get_all().is_empty());
    }

    #[test]
    fn delete_non_existing_task_returns_error() {
        let mut store = temp_store();

        let res = store.delete(1);

        assert!(matches!(res, Err(TaskStoreError::TaskNotFound(1))));
    }

    #[test]
    fn set_status_updates_task() {
        let mut store = temp_store();
        let id = store.add("Buy groceries".to_string()).unwrap();

        store.set_status(id, TaskStatus::Done).unwrap();

        assert_eq!(store.get_all()[0].status, TaskStatus::Done);
    }

    #[test]
    fn set_status_non_existing_task_returns_error() {
        let mut store = temp_store();

        let res = store.set_status(1, TaskStatus::Done);

        assert!(matches!(res, Err(TaskStoreError::TaskNotFound(1))));
    }

    #[test]
    fn save_and_load_roundtrip() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();

        let mut store = TaskStore::load(&path).unwrap();
        store.add("Task 1".to_string()).unwrap();
        store.add("Task 2".to_string()).unwrap();
        store.save().unwrap();

        let reloaded = TaskStore::load(&path).unwrap();
        assert_eq!(reloaded.get_all().len(), 2);
        assert_eq!(reloaded.get_all()[0].title, "Task 1".to_string());
        assert_eq!(reloaded.get_all()[1].title, "Task 2".to_string());
    }

    #[test]
    fn ids_remain_unique() {
        let mut store = temp_store();
        let id1 = store.add("First".to_string()).unwrap();
        let id2 = store.add("Second".to_string()).unwrap();
        store.delete(id1).unwrap();
        let id3 = store.add("Third".to_string()).unwrap();

        let ids: Vec<u32> = store.get_all().iter().map(|t| t.id).collect();
        let mut unique_ids = ids.clone();
        unique_ids.sort();
        unique_ids.dedup();

        assert_eq!(
            ids.len(),
            unique_ids.len(),
            "task ids must be unique, got {ids:?} (id2={id2}, id3={id3})"
        );
    }

    #[test]
    fn save_creates_parent_directory() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("tasks.json");

        let mut store = TaskStore::load(&path).unwrap();
        store.add("Task".to_string()).unwrap();
        store.save().unwrap();

        assert!(path.exists());
    }

    #[test]
    fn load_malformed_json_returns_parse_error() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "not json").unwrap();
        let path = file.path().to_path_buf();

        let res = TaskStore::load(&path);

        assert!(matches!(res, Err(TaskStoreError::Parse(_))));
    }
}
