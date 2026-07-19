mod cli;
mod store;
mod task;

use clap::Parser;
use cli::*;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Add { title: _ } => {
            println!("Task added successfully (ID: 1)");
        }

        Command::Update { id, title } => {
            println!("Task updated successfully: \"{title}\" (ID: {id})");
        }

        Command::Delete { id } => {
            println!("Task deleted successfully (ID: {id})");
        }

        Command::MarkInProgress { id } => {
            println!("Task marked as in progress (ID: {id})");
        }

        Command::MarkDone { id } => {
            println!("Task marked as in done (ID: {id})");
        }

        Command::List { command } => match command {
            None => {
                println!("Listing all tasks")
            }

            Some(ListCommand::Todo) => {
                println!("Listing todo tasks")
            }

            Some(ListCommand::InProgress) => {
                println!("Listing in progress tasks")
            }

            Some(ListCommand::Done) => {
                println!("Listing completed tasks")
            }
        },
    }
}
