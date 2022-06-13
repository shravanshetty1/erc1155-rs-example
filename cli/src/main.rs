use borsh::{BorshDeserialize, BorshSerialize};
use clap::Command;
use sled::IVec;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const IS_INITIALISED: &str = "is-initialised";
const CLI_STATE: &str = "state";

#[derive(BorshSerialize, BorshDeserialize)]
pub struct State {
    accounts: Vec<Pubkey>,
    current: Pubkey,
}

fn main() -> Result<()> {
    let store = sled::open("~/.erc1155-rs/store")?;
    if (store.get(IS_INITIALISED)?).is_none() {
        initialize_store(store)?;
    }

    let matches = Command::new("erc115-rs")
        .subcommand(
            Command::new("account")
                .subcommand(Command::new("list"))
                .subcommand(Command::new("current"))
                .subcommand(Command::new("set")),
        )
        .subcommand(Command::new("transfer"))
        .subcommand(Command::new("balances"))
        .subcommand(Command::new("approve"))
        .get_matches();

    match matches.subcommand() {
        Some(("account", matches)) => match matches.subcommand() {
            Some(("list", _matches)) => {
                println!("hello")
            }
            _ => {}
        },
        _ => {}
    }

    Ok(())
}

pub fn initialize_store(store: sled::Db) -> crate::Result<()> {
    let alice = Keypair::new();
    let bob = Keypair::new();

    let cli_state: State = State {
        accounts: vec![alice.pubkey(), bob.pubkey()],
        current: alice.pubkey(),
    };
    store.insert(CLI_STATE, IVec::from(borsh::to_vec(&cli_state)?))?;

    store.insert(IS_INITIALISED, IVec::from("true"))?;
    Ok(())
}
