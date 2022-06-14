use crate::State;
use clap::{Arg, Command};
use contract::{ERC1155Implementation, ERC1155};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn command() -> Command<'static> {
    Command::new("transfer")
        .args(vec![
            Arg::new("from")
                .long("from")
                .takes_value(true)
                .required(true)
                .help("Public key of the account to transfer the tokens from"),
            Arg::new("to")
                .long("to")
                .takes_value(true)
                .required(true)
                .help("Public key of the account to transfer the tokens to"),
            Arg::new("token-id")
                .long("token-id")
                .takes_value(true)
                .required(true)
                .help("The id of the token to be transferred"),
            Arg::new("amount")
                .long("amount")
                .takes_value(true)
                .required(true)
                .help("The amount of tokens to be transferred"),
        ])
        .about("Command that allows transferring tokens from one account to another")
}

pub fn handle(store: sled::Db, matches: &clap::ArgMatches) -> crate::Result<()> {
    let state = State::get_from_db(&store)?;

    let from = Pubkey::from_str(
        matches
            .value_of("from")
            .ok_or("from account is mandatory")?,
    )?;
    let to = Pubkey::from_str(matches.value_of("to").ok_or("to account is mandatory")?)?;
    let token_id = matches
        .value_of("token-id")
        .ok_or("token-id is mandatory")?
        .parse::<u128>()?;
    let amount = matches
        .value_of("amount")
        .ok_or("amount is mandatory")?
        .parse::<u128>()?;

    let contract = ERC1155Implementation::new(store, state.current);
    contract.safe_batch_transfer_from(from, to, vec![token_id], vec![amount], vec![])?;

    Ok(())
}
