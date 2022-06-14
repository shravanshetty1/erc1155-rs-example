use crate::State;
use clap::{Arg, Command};
use contract::{ERC1155Implementation, ERC1155};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn command() -> Command<'static> {
    Command::new("approval")
        .subcommand(Command::new("add").arg(Arg::new("address").takes_value(true).required(true)))
        .subcommand(
            Command::new("remove").arg(Arg::new("address").takes_value(true).required(true)),
        )
}

pub fn handle(store: sled::Db, matches: &clap::ArgMatches) -> crate::Result<()> {
    let state = State::get_from_db(&store)?;
    let contract = ERC1155Implementation::new(store, state.current);

    match matches.subcommand() {
        Some(("add", matches)) => {
            let address =
                Pubkey::from_str(matches.value_of("address").ok_or("address is mandatory")?)?;

            contract.set_approval_for_all(address, true)?;
        }
        Some(("remove", matches)) => {
            let address =
                Pubkey::from_str(matches.value_of("address").ok_or("address is mandatory")?)?;

            contract.set_approval_for_all(address, false)?;
        }
        _ => {}
    }

    Ok(())
}
