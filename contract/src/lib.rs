use solana_sdk::pubkey::Pubkey;
use std::error::Error;

// erc1155 interface from https://eips.ethereum.org/EIPS/eip-1155
pub trait ERC1155 {
    fn safe_batch_transfer_from(
        from: Pubkey,
        to: Pubkey,
        ids: Vec<u128>,
        values: Vec<u128>,
        data: Vec<u8>,
    ) -> crate::Result<()>;
    fn balance_of_batch(owners: Vec<Pubkey>, ids: Vec<u128>) -> crate::Result<u128>;
    fn set_approval_for_all(operator: Pubkey, approved: bool) -> crate::Result<()>;
    fn is_approved_for_all(owner: Pubkey, operator: Pubkey) -> crate::Result<bool>;
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;
