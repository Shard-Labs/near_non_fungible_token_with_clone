use crate::non_fungible_token_clone::NonFungibleTokenClone;
use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
use near_contract_standards::non_fungible_token::Token;
use near_sdk::json_types::U128;
use near_sdk::{env, require, AccountId};

type TokenId = String;

impl NonFungibleTokenClone {
    /// Helper function used by a enumerations methods
    /// Note: this method is not exposed publicly to end users
    fn enum_get_token(&self, owner_id: AccountId, token_id: TokenId) -> Token {
        let clone_id = self.nft_clone_from_id.get(&token_id).unwrap();

        let metadata = self
            .nft
            .token_metadata_by_id
            .as_ref()
            .and_then(|m| m.get(&clone_id));

        let approved_account_ids = self.nft.approvals_by_id.as_ref().map(|approvals_by_id| {
            approvals_by_id
                .get(&token_id.to_string())
                .unwrap_or_default()
        });

        Token {
            token_id,
            owner_id,
            metadata,
            approved_account_ids,
        }
    }
}

impl NonFungibleTokenEnumeration for NonFungibleTokenClone {
    fn nft_total_supply(&self) -> U128 {
      self.nft.nft_total_supply()
    }

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
        // Get starting index, whether or not it was explicitly given.
        // Defaults to 0 based on the spec:
        // https://nomicon.io/Standards/NonFungibleToken/Enumeration.html#interface
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        require!(
            (self.nft.owner_by_id.len() as u128) >= start_index,
            "Out of bounds, please use a smaller from_index."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        require!(limit != 0, "Cannot provide limit of 0.");
        self.nft.owner_by_id
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|(token_id, owner_id)| self.enum_get_token(owner_id, token_id))
            .collect()
    }

    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        self.nft.nft_supply_for_owner(account_id)
    }

    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token> {
        let tokens_per_owner = self.nft.tokens_per_owner.as_ref().unwrap_or_else(|| {
            env::panic_str(
                "Could not find tokens_per_owner when calling a method on the \
                enumeration standard.",
            )
        });
        let token_set = if let Some(token_set) = tokens_per_owner.get(&account_id) {
            token_set
        } else {
            return vec![];
        };
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        require!(limit != 0, "Cannot provide limit of 0.");
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        require!(
            token_set.len() as u128 > start_index,
            "Out of bounds, please use a smaller from_index."
        );
        token_set
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|token_id| self.enum_get_token(account_id.clone(), token_id))
            .collect()
    }
}