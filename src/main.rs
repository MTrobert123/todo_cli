mod cli;
mod database;
mod tasks;
fn main() {
    database::create_tables().unwrap_or_else(|err| {
        eprintln!("there was an error creating database: {}", err);
        std::process::exit(1);
    });
    cli::parse_args();
}
