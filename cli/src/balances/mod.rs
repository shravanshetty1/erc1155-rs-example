use crate::State;
use clap::{Arg, Command};
use contract::{ERC1155Implementation, ERC1155};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn command() -> Command<'static> {
    Command::new("balances").args(vec![
        Arg::new("owner")
            .long("owner")
            .takes_value(true)
            .required(true),
        Arg::new("token-id")
            .long("token-id")
            .takes_value(true)
            .required(true),
    ])
}

pub fn handle(store: sled::Db, matches: &clap::ArgMatches) -> crate::Result<()> {
    let state = State::get_from_db(&store)?;
    let contract = ERC1155Implementation::new(store, state.current);
    let owner = Pubkey::from_str(matches.value_of("owner").ok_or("owner is mandatory")?)?;
    let token_id = matches
        .value_of("token-id")
        .ok_or("token-id is mandatory")?
        .parse::<u128>()?;
    let balance = contract.balance_of_batch(vec![owner], vec![token_id])?;
    println!("{}", balance.first().unwrap_or(&0));

    Ok(())
}
