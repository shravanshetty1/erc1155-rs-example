use crate::State;
use clap::{Arg, Command};
use contract::{ERC1155Implementation, ERC1155};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn command() -> Command<'static> {
    Command::new("approval")
        .subcommand(Command::new("add").about("Add an approval to the current account")
            .arg(Arg::new("address").takes_value(true).help("Public key of the account to be approved").required(true)))
        .subcommand(
            Command::new("remove").about("Remove an approval from the current account").arg(Arg::new("address").help("Public key of the account whose approval is to be revoked").takes_value(true).required(true)),
        ).about("Commands related to setting approvals for an account, approved accounts can control the assets of the account their approved for")
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
