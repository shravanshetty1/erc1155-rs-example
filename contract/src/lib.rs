use borsh::BorshDeserialize;
use sled::IVec;

use solana_sdk::pubkey::Pubkey;
use std::collections::{HashMap, HashSet};
use std::error::Error;

// erc1155 interface from https://eips.ethereum.org/EIPS/eip-1155
pub trait ERC1155 {
    fn safe_batch_transfer_from(
        &self,
        from: Pubkey,
        to: Pubkey,
        ids: Vec<u128>,
        values: Vec<u128>,
        data: Vec<u8>,
    ) -> crate::Result<()>;
    fn balance_of_batch(&self, owners: Vec<Pubkey>, ids: Vec<u128>) -> crate::Result<Vec<u128>>;
    fn set_approval_for_all(&self, operator: Pubkey, approved: bool) -> crate::Result<()>;
    fn is_approved_for_all(&self, owner: Pubkey, operator: Pubkey) -> crate::Result<bool>;
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct ERC1155Implementation {
    store: sled::Db,
    caller: Pubkey,
}

impl ERC1155Implementation {
    pub fn new(store: sled::Db, caller: Pubkey) -> Self {
        ERC1155Implementation { store, caller }
    }

    pub fn create_token(&self, supply: u128) {}
}

const ACCOUNT_PREFIX: &str = "account";

#[derive(borsh::BorshSerialize, borsh::BorshDeserialize, Default)]
pub struct Account {
    approvals: HashSet<Pubkey>,
    balances: HashMap<u128, u128>,
}

impl Account {
    fn get_from_db(store: &sled::Db, address: Pubkey) -> crate::Result<Account> {
        Ok(Account::try_from_slice(
            store
                .get(format!("{}-{}", ACCOUNT_PREFIX, address))?
                .unwrap_or_default()
                .as_ref(),
        )
        .unwrap_or_default())
    }

    fn set_to_db(store: &sled::Db, address: Pubkey, state: Account) -> crate::Result<()> {
        store.insert(
            format!("{}-{}", ACCOUNT_PREFIX, address),
            IVec::from(borsh::to_vec(&state)?),
        )?;
        Ok(())
    }
}

impl ERC1155 for ERC1155Implementation {
    // TODO check if caller can execute transaction
    fn safe_batch_transfer_from(
        &self,
        from: Pubkey,
        to: Pubkey,
        ids: Vec<u128>,
        values: Vec<u128>,
        _data: Vec<u8>,
    ) -> Result<()> {
        let mut from_state: Account = Account::get_from_db(&self.store, from.clone())?;
        let mut to_state: Account = Account::get_from_db(&self.store, to.clone())?;

        for i in 0..ids.len() {
            let id = ids.get(i).ok_or("index out of bounds")?;
            let val = values.get(i).ok_or("index out of bounds")?;

            // TODO not sure if this works
            if from_state.balances.get(id).unwrap_or(&0) > val {
                *from_state.balances.get_mut(id).unwrap_or(&mut 0) -= val;
                *to_state.balances.get_mut(id).unwrap_or(&mut 0) += val;
            } else {
                return Err(format!("insufficient balance for token num {}", id).into());
            }
        }

        Account::set_to_db(&self.store, from, from_state)?;
        Account::set_to_db(&self.store, to, to_state)?;

        Ok(())
    }

    fn balance_of_batch(&self, owners: Vec<Pubkey>, ids: Vec<u128>) -> Result<Vec<u128>> {
        let mut balances: Vec<u128> = Vec::new();
        for i in 0..owners.len() {
            let owner = owners.get(i).ok_or("index out of bounds")?;
            let id = ids.get(i).ok_or("index out of bounds")?;

            let mut owner_state = Account::get_from_db(&self.store, owner.clone())?;
            balances.push(owner_state.balances.get(id).unwrap_or(&0).clone());
        }

        Ok(balances)
    }

    fn set_approval_for_all(&self, operator: Pubkey, approved: bool) -> Result<()> {
        let mut caller_state = Account::get_from_db(&self.store, self.caller.clone())?;
        if approved {
            caller_state.approvals.insert(operator);
        } else {
            caller_state.approvals.remove(&operator);
        }
        Account::set_to_db(&self.store, self.caller.clone(), caller_state);
        Ok(())
    }

    fn is_approved_for_all(&self, owner: Pubkey, operator: Pubkey) -> Result<bool> {
        let mut owner_state = Account::get_from_db(&self.store, owner.clone())?;
        Ok(owner_state.approvals.get(&operator).is_some())
    }
}
