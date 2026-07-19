use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "task-cli")]
#[command(version, about, long_about= None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Create a new task.
    Add {
        /// Title of the new task.
        title: String,
    },

    /// Update the title of an existing task.
    Update {
        /// ID of the task to update.
        id: i32,

        /// New title for the task.
        title: String,
    },

    /// Delete a task.
    Delete {
        /// ID of the task to delete.
        id: i32,
    },

    /// Mark a task as in progress.
    MarkInProgress {
        /// ID of the task to update.
        id: i32,
    },

    /// Mark a task as done.
    MarkDone {
        /// ID of the task to update.
        id: i32,
    },

    /// List tasks.
    List {
        #[command(subcommand)]
        command: Option<ListCommand>,
    },
}

#[derive(Subcommand)]
pub enum ListCommand {
    /// Show tasks that have not been started.
    Todo,

    /// Show tasks that are currently in progress.
    InProgress,

    /// Show completed tasks.
    Done,
}
