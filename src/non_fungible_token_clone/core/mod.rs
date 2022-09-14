mod core_impl;
//pub use self::core_impl::*;

use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::AccountId;

pub trait NonFungibleTokenClone {
    fn nft_clone(&mut self, genesis_token: TokenId, next_token_id: TokenId, owner_id: AccountId);
}
