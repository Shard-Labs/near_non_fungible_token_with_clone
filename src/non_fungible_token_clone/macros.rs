#[macro_export]
macro_rules! impl_non_fungible_token_clone {
    ($contract: ident, $token: ident) => {
        #[macro_use]
        extern crate near_non_fungible_token_with_clone;
        //use crate::near_non_fungible_token_with_clone::core::NonFungibleTokenClone;

        #[near_bindgen]
        impl NonFungibleTokenClone for $contract {
            #[payable]
            fn nft_clone(
                &mut self,
                genesis_token: TokenId,
                next_token_id: TokenId,
                owner_id: AccountId,
            ) {
                assert!(
                    self.$token.token_metadata_by_id.is_some(),
                    "Contract does not implement non fungible token core"
                );

                if let Some(token_map) = &mut self.$token.token_metadata_by_id {
                    let token_metadata = token_map.get(&genesis_token);
                    self.$token
                        .internal_mint(next_token_id, owner_id, token_metadata);
                }
            }
        }
    };
}
