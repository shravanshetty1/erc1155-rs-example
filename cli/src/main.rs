use borsh::{BorshDeserialize, BorshSerialize};
use clap::{Arg, ArgMatches, Command};
use sled::IVec;
use std::collections::HashSet;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

use std::error::Error;

use contract::{ERC1155Implementation, ERC1155};
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const IS_INITIALISED: &str = "is-initialised";
const CLI_STATE: &str = "state";

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct State {
    accounts: HashSet<Pubkey>,
    current: Pubkey,
}

fn main() -> Result<()> {
    let store = sled::open("~/.erc1155-rs/store")?;
    if (store.get(IS_INITIALISED)?).is_none() {
        initialize_store(&store)?;
    }

    let matches = Command::new("erc115-rs")
        .subcommand(
            Command::new("account")
                .subcommand(Command::new("list"))
                .subcommand(Command::new("current"))
                .subcommand(Command::new("set").arg(Arg::new("account").required(true)))
                .subcommand(Command::new("state").arg(Arg::new("account").required(true))),
        )
        .subcommand(Command::new("transfer").args(vec![
            Arg::new("from").long("from").takes_value(true).required(true),
            Arg::new("to").long("to").takes_value(true).required(true),
            Arg::new("token-id").long("token-id").takes_value(true).required(true),
            Arg::new("amount").long("amount").takes_value(true).required(true),
        ]))
        .subcommand(Command::new("balances").args(vec![
            Arg::new("owner").long("owner").takes_value(true).required(true),
            Arg::new("token-id").long("token-id").takes_value(true).required(true),
        ]))
        .subcommand(
            Command::new("approval")
                .subcommand(
                    Command::new("add").arg(Arg::new("address").takes_value(true).required(true)),
                )
                .subcommand(
                    Command::new("remove")
                        .arg(Arg::new("address").takes_value(true).required(true)),
                ),
        )
        .get_matches();

    let mut state: State =
        State::try_from_slice(store.get(CLI_STATE)?.unwrap_or_default().as_ref())?;

    match matches.subcommand() {
        Some(("account", matches)) => match matches.subcommand() {
            Some(("list", _matches)) => list_accounts(store)?,
            Some(("current", _matches)) => current_account(store)?,
            Some(("state", matches)) => {
                let contract = ERC1155Implementation::new(store, state.current.clone());
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
                set_account(store, account)?
            }
            _ => {}
        },
        Some(("transfer", matches)) => {
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

            let contract = ERC1155Implementation::new(store, state.current.clone());
            contract.safe_batch_transfer_from(from, to, vec![token_id], vec![amount], vec![])?;
        }
        Some(("balances", matches)) => {
            let contract = ERC1155Implementation::new(store, state.current.clone());
            let owner = Pubkey::from_str(matches.value_of("owner").ok_or("owner is mandatory")?)?;
            let token_id = matches
                .value_of("token-id")
                .ok_or("token-id is mandatory")?
                .parse::<u128>()?;
            let balance = contract.balance_of_batch(vec![owner], vec![token_id])?;
            println!("{}", balance.first().unwrap_or(&0));
        }
        Some(("approval", matches)) => match matches.subcommand() {
            Some(("add", matches)) => {
                let address =
                    Pubkey::from_str(matches.value_of("address").ok_or("address is mandatory")?)?;

                let contract = ERC1155Implementation::new(store, state.current.clone());
                contract.set_approval_for_all(address, true)?;
            }
            Some(("remove", matches)) => {
                let address =
                    Pubkey::from_str(matches.value_of("address").ok_or("address is mandatory")?)?;

                let contract = ERC1155Implementation::new(store, state.current.clone());
                contract.set_approval_for_all(address, false)?;
            }
            _ => {}
        },
        _ => {}
    }

    Ok(())
}

pub fn set_account(store: sled::Db, account: Pubkey) -> crate::Result<()> {
    let mut state: State =
        State::try_from_slice(store.get(CLI_STATE)?.unwrap_or_default().as_ref())?;
    state.current = account;
    state.accounts.insert(account);
    store.insert(CLI_STATE, IVec::from(borsh::to_vec(&state)?))?;

    Ok(())
}

pub fn current_account(store: sled::Db) -> crate::Result<()> {
    let state: State = State::try_from_slice(store.get(CLI_STATE)?.unwrap_or_default().as_ref())?;
    println!("{:#?}", state.current);

    Ok(())
}

pub fn list_accounts(store: sled::Db) -> crate::Result<()> {
    let state: State = State::try_from_slice(store.get(CLI_STATE)?.unwrap_or_default().as_ref())?;
    println!("{:#?}", state.accounts);

    Ok(())
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
    store.insert(CLI_STATE, IVec::from(borsh::to_vec(&cli_state)?))?;

    let contract = contract::ERC1155Implementation::new(store.clone(), alice.pubkey());
    contract.create_token(100000)?;

    store.insert(IS_INITIALISED, IVec::from("true"))?;
    Ok(())
}
