use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct WhitelistInfo {
    pub token_mint: Pubkey,
    pub merkle_tree: Pubkey,
}
