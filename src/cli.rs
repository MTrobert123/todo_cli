use crate::tasks;
use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add new task
    Add {
        /// Task name for the new task
        name: Option<String>,
        #[clap(short = 'd')]
        /// Due date for the new task (format: "%Y-%m-%d %H:%M:%S")
        date: Option<String>,
    },
    /// List all available tasks
    Ls {
        /// Disabling emojis for older or unsupported terminal.
        #[clap(short = 'd', long = "disable-pretty")]
        disable_pretty: bool,
    },
    /// Delete a task
    Del { id: Option<i32> },
    /// Mark a task as done
    Check { id: Option<i32> },
    /// Mark a task as undone
    Uncheck { id: Option<i32> },
    /// Rename a task
    Rename {
        id: Option<i32>,
        name: Option<String>,
    },
}

fn check_id(id: &Option<i32>) -> i32 {
    match id {
        Some(value) => *value,
        None => {
            eprintln!("error: please enter the task id.");
            exit(1);
        }
    }
}

pub fn parse_args() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add { name, date } => {
            let date_str = date.as_deref().unwrap_or_else(|| "");
            let name_str = match name.as_deref() {
                Some(value) => value,
                None => {
                    eprintln!("error: no name was provided.");
                    exit(1);
                }
            };
            match tasks::new_task(name_str.to_string(), date_str.to_string()) {
                Ok(value) => println!("Task '{}' was created.", value),
                Err(err) => eprintln!("There was an error creating new task: {}", err),
            }
        }
        Commands::Ls { disable_pretty } => {
            if *disable_pretty == true {
                std::env::set_var("DISABLE_TODO_PRETTY", "1");
            }
            tasks::get_all_tasks().unwrap_or_else(|err| {
                eprintln!("There was a problem getting tasks: {}", err);
            });
        }
        Commands::Del { id } => match id {
            Some(value) => {
                tasks::delete_task(value).unwrap_or_else(|err| {
                    eprintln!("There was an error deleting task: {}", err);
                });
            }
            None => {
                eprintln!("error: please enter the task id.");
            }
        },
        Commands::Check { id } => {
            let id = check_id(&id);
            tasks::check_task(id, true).unwrap_or_else(|err| {
                eprintln!("There was an error checking task: {}", err);
            });
        }
        Commands::Uncheck { id } => {
            let id = check_id(&id);
            tasks::check_task(id, false).unwrap_or_else(|err| {
                eprintln!("There was an error unchecking task: {}", err);
            });
        }
        Commands::Rename { id, name } => {
            let id = check_id(&id);
            match name.as_deref() {
                Some(value) => {
                    tasks::rename_task(id, value.to_string()).unwrap_or_else(|err| {
                        eprintln!("There was an error renaming task: {}", err);
                    });
                }
                None => {
                    eprintln!("error: no name provided.");
                }
            }
        }
    }
}
