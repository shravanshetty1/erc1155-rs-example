use sled::IVec;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashSet;

use borsh::{BorshDeserialize, BorshSerialize};

const CLI_STATE: &str = "state";

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct State {
    pub(crate) accounts: HashSet<Pubkey>,
    pub(crate) current: Pubkey,
}

impl State {
    pub(crate) fn get_from_db(store: &sled::Db) -> crate::Result<State> {
        Ok(
            State::try_from_slice(store.get(CLI_STATE)?.unwrap_or_default().as_ref())
                .unwrap_or_default(),
        )
    }

    pub(crate) fn set_to_db(store: &sled::Db, state: State) -> crate::Result<()> {
        store.insert(CLI_STATE, IVec::from(borsh::to_vec(&state)?))?;
        Ok(())
    }
}
