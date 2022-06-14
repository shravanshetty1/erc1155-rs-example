use crate::state::State;
use clap::{Arg, Command};
use contract::ERC1155Implementation;

use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn command() -> Command<'static> {
    Command::new("account")
        .subcommand(Command::new("list"))
        .subcommand(Command::new("current"))
        .subcommand(Command::new("set").arg(Arg::new("account").required(true)))
        .subcommand(Command::new("state").arg(Arg::new("account").required(true)))
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
