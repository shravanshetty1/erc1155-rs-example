use crate::state::State;
use clap::{Arg, Command};
use contract::ERC1155Implementation;

use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn command() -> Command<'static> {
    Command::new("account")
        .subcommand(Command::new("list").about("Lists all known accounts"))
        .subcommand(Command::new("current").about("Public key of the current account in use"))
        .subcommand(Command::new("set").about("Allows setting the current account").arg(Arg::new("account")
            .required(true).help("Public key of the account to set")))
        .subcommand(Command::new("state").about("Prints the contract state of the current account - shows the list of approved accounts and balances of all tokens")
            .arg(Arg::new("account").required(true))).about("Commands related to accounts in use")
}

pub fn handle(store: sled::Db, matches: &clap::ArgMatches) -> crate::Result<()> {
    let mut state: State = State::get_from_db(&store)?;
    let contract = ERC1155Implementation::new(store.clone(), state.current);

    match matches.subcommand() {
        Some(("list", _matches)) => println!("{:#?}", state.accounts),
        Some(("current", _matches)) => println!("{:#?}", state.current),
        Some(("state", matches)) => {
            let account =
                Pubkey::from_str(matches.value_of("account").ok_or("account is mandatory")?)?;

            let account_state = contract.account_state(account)?;
            println!("{:#?}", account_state)
        }
        Some(("set", matches)) => {
            let account = matches
                .value_of("account")
                .ok_or("account arg is required")?;
            let account = Pubkey::from_str(account)?;

            state.current = account;
            state.accounts.insert(account);
            State::set_to_db(&store, state)?;
        }
        _ => {}
    }

    Ok(())
}
