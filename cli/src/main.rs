mod account;
mod approval;
mod balances;
mod state;
mod transfer;

use clap::Command;
use sled::IVec;
use std::collections::HashSet;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

use std::error::Error;

use crate::state::State;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const IS_INITIALISED: &str = "is-initialised";
fn main() -> Result<()> {
    let mut path = dirs::home_dir().ok_or("could not find home directory")?;
    path.push(".erc1155-rs");
    path.push("store.txt");
    let store = sled::open(path)?;
    if (store.get(IS_INITIALISED)?).is_none() {
        initialize_store(&store)?;
    }

    let matches = Command::new("erc115-rs")
        .subcommand(account::command())
        .subcommand(transfer::command())
        .subcommand(balances::command())
        .subcommand(approval::command())
        .about("Application to interact with rust implementation of erc1155, contract state is stored locally instead of on the blockchain")
        .get_matches();

    return match matches.subcommand() {
        Some(("account", matches)) => account::handle(store, matches),
        Some(("transfer", matches)) => transfer::handle(store, matches),
        Some(("balances", matches)) => balances::handle(store, matches),
        Some(("approval", matches)) => approval::handle(store, matches),
        _ => Ok(()),
    };
}

pub fn initialize_store(store: &sled::Db) -> crate::Result<()> {
    let alice = Keypair::new();
    let bob = Keypair::new();

    let mut accounts: HashSet<Pubkey> = HashSet::new();
    accounts.insert(alice.pubkey());
    accounts.insert(bob.pubkey());
    let cli_state: State = State {
        accounts,
        current: alice.pubkey(),
    };
    State::set_to_db(store, cli_state)?;

    let contract = contract::ERC1155Implementation::new(store.clone(), alice.pubkey());
    contract.create_token(100000)?;

    store.insert(IS_INITIALISED, IVec::from("true"))?;
    Ok(())
}
