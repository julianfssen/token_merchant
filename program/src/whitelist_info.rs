use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WhitelistInfo {
    pub default_token_price: u64,
    pub default_token_purchase_limit: u64,
    pub max_transfer_amount_delegated: u64,
    pub merkle_tree: Pubkey,
    pub token_mint: Pubkey,
}
