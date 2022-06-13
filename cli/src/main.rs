use clap::{ArgMatches, Command};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let matches = Command::new("erc115-rs")
        .subcommand(
            Command::new("account")
                .subcommand(Command::new("list"))
                .subcommand(Command::new("add"))
                .subcommand(Command::new("current"))
                .subcommand(Command::new("set")),
        )
        .subcommand(Command::new("transfer"))
        .subcommand(Command::new("balances"))
        .subcommand(Command::new("approve"))
        .get_matches();

    match matches.subcommand() {
        Some(("account", matches)) => match matches.subcommand() {
            Some(("list", matches)) => {
                println!("hello")
            }
            _ => {}
        },
        _ => {}
    }

    Ok(())
}
