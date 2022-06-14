use borsh::BorshDeserialize;
use sled::IVec;
use solana_sdk::pubkey::Pubkey;
use std::collections::{HashMap, HashSet};

const CONTRACT_PREFIX: &str = "contract";
#[derive(borsh::BorshSerialize, borsh::BorshDeserialize, Default)]
pub struct Contract {
    pub(crate) token_count: u128,
}

impl Contract {
    pub(crate) fn get_from_db(store: &sled::Db) -> crate::Result<Contract> {
        Ok(
            Contract::try_from_slice(store.get(CONTRACT_PREFIX)?.unwrap_or_default().as_ref())
                .unwrap_or_default(),
        )
    }

    pub(crate) fn set_to_db(store: &sled::Db, state: Contract) -> crate::Result<()> {
        store.insert(CONTRACT_PREFIX, IVec::from(borsh::to_vec(&state)?))?;
        Ok(())
    }
}

const ACCOUNT_PREFIX: &str = "account";

#[derive(borsh::BorshSerialize, borsh::BorshDeserialize, Default, Debug)]
pub struct Account {
    pub(crate) approvals: HashSet<Pubkey>,
    pub(crate) balances: HashMap<u128, u128>,
}

impl Account {
    pub(crate) fn get_from_db(store: &sled::Db, address: Pubkey) -> crate::Result<Account> {
        Ok(Account::try_from_slice(
            store
                .get(format!("{}-{}", ACCOUNT_PREFIX, address))?
                .unwrap_or_default()
                .as_ref(),
        )
        .unwrap_or_default())
    }

    pub(crate) fn set_to_db(
        store: &sled::Db,
        address: Pubkey,
        state: Account,
    ) -> crate::Result<()> {
        store.insert(
            format!("{}-{}", ACCOUNT_PREFIX, address),
            IVec::from(borsh::to_vec(&state)?),
        )?;
        Ok(())
    }
}
