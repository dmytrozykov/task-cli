mod cli;
mod store;
mod task;

use anyhow::{Ok, Result};
use clap::Parser;

use cli::{Cli, Command, ListCommand};
use store::TaskStore;
use task::{Task, TaskStatus};

const JSON_FILENAME: &'static str = "tasks.json";

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut store = TaskStore::load(JSON_FILENAME)?;

    handle_command(cli.command, &mut store)?;

    store.save()?;

    Ok(())
}

fn handle_command(command: Command, store: &mut TaskStore) -> Result<()> {
    match command {
        Command::Add { title } => {
            let id = store.add(title);
            println!("Task added successfully (ID: {id})");
        }

        Command::Update { id, title } => {
            store.rename(id, title)?;
            println!("Task updated successfully (ID: {id})");
        }

        Command::Delete { id } => {
            store.delete(id)?;
            println!("Task deleted successfully (ID: {id})");
        }

        Command::MarkInProgress { id } => {
            store.set_status(id, TaskStatus::InProgress)?;
            println!("Task marked as in progress (ID: {id})");
        }

        Command::MarkDone { id } => {
            store.set_status(id, TaskStatus::Done)?;
            println!("Task marked as in done (ID: {id})");
        }

        Command::List { command } => {
            let status = match command {
                None => None,
                Some(ListCommand::Todo) => Some(TaskStatus::Todo),
                Some(ListCommand::InProgress) => Some(TaskStatus::InProgress),
                Some(ListCommand::Done) => Some(TaskStatus::Done),
            };

            list_tasks(store.get_all(), status);
        }
    }

    Ok(())
}

fn list_tasks(tasks: &[Task], status: Option<TaskStatus>) {
    let mut tasks = tasks
        .iter()
        .filter(|task| status.map_or(true, |status| task.status == status));

    if tasks.next().is_none() {
        println!("No tasks");
        return;
    }

    let message = match status {
        None => "All tasks:",
        Some(TaskStatus::Todo) => "Todo tasks:",
        Some(TaskStatus::InProgress) => "In progres tasks:",
        Some(TaskStatus::Done) => "Done tasks:",
    };

    println!("{message}");

    for task in tasks {
        println!("{task}")
    }
}
