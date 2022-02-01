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
    Add {
        name: Option<String>,
        #[clap(short = 'd')]
        date: Option<String>,
    },
    Ls,
    Del {
        id: Option<i32>,
    },
    Check {
        id: Option<i32>,
    },
    Uncheck {
        id: Option<i32>,
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
        Commands::Ls => {
            tasks::get_all_tasks().unwrap_or_else(|err| {
                eprintln!("There was a problem getting tasks: {}", err);
                exit(1);
            });
        }
        Commands::Del { id } => match id {
            Some(value) => {
                tasks::delete_task(value).unwrap_or_else(|err| {
                    eprintln!("There was an error deleting task: {}", err);
                    exit(1);
                });
            }
            None => {
                eprintln!("error: please enter the task id.");
                exit(1);
            }
        },
        Commands::Check { id } => {
            let id = check_id(&id);
            tasks::check_task(id, true).unwrap_or_else(|err| {
                eprintln!("There was an error checking task: {}", err);
            });
        }
        &Commands::Uncheck { id } => {
            let id = check_id(&id);
            tasks::check_task(id, false).unwrap_or_else(|err| {
                eprintln!("There was an error unchecking task: {}", err);
            });
        }
    }
}
