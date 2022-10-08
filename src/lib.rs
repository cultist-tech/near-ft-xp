use near_sdk::{AccountId, Balance, env, log, near_bindgen, PanicOnDefault, PromiseOrValue};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;

use mfight_sdk::ft::{FungibleToken, FungibleTokenMetadata, FT_METADATA_SPEC, FungibleTokenMetadataProvider, FtMint};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  token: FungibleToken,
  metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg width='24' height='24' viewBox='0 0 24 24' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M6.17512 18.0234L5.93884 17.7027V16.2629L6.31818 15.9379L5.93884 16.0618V11.0351L5.10793 11.8686C5.02618 11.9505 5.02618 12.0835 5.10793 12.1655L5.67767 12.7369V14.2216L4.96902 13.5108L5.09091 13.0884L4.62552 13.1662L4.36786 12.9078C4.20596 12.7454 4.09751 12.5497 4.04249 12.3427L4.04545 12.3362L4.03902 12.3294C3.93313 11.9127 4.04275 11.4523 4.36786 11.1263L5.04795 10.4441H5.34091V10.1503L8.49431 6.98735H6.98546V8.13528L5.93884 9.18506V7.59159L6.31818 7.38946L5.93884 7.12088V5.93757H7.78204L8.06818 6.20407L8.56638 5.93757H9.54093L9.54343 5.93506L11.0236 5.93506L11.0211 5.93757H12.9789L12.148 5.10415C12.0663 5.02216 11.9337 5.02215 11.852 5.10415L11.2853 5.67252H9.80518L11.1119 4.36184C11.3231 4.15005 11.5907 4.02943 11.8662 4L12.8184 4.29694C12.8422 4.31753 12.8655 4.33916 12.8881 4.36184L13.1591 4.63366V4.72234L13.5455 5.42901L14.0334 5.51058L16.9416 8.42764V6.98735H15.8751L14.8285 5.93757H17.9883V6.34526L17.8636 6.47762L17.9883 6.76931V13.072L18.8921 12.1655C18.9738 12.0835 18.9738 11.9505 18.8921 11.8686L18.2495 11.224V9.73941L18.5904 10.0813L18.6364 10.1933L18.7823 10.2739L19.6321 11.1263C20.1226 11.6182 20.1226 12.4158 19.6321 12.9078L19.5896 12.9504L19.0909 13.0884V13.4507L15.5786 16.9736H16.9416V15.9694L17.9883 14.9196V18.0234H15.2599L15.1136 17.8983L14.7605 18.0234H10.9482L11.852 18.9299C11.9337 19.0119 12.0663 19.0119 12.148 18.9299L12.7915 18.2844H14.2045V18.3518L13.3157 19.2433H13.1591L12.8664 19.6934C12.835 19.7234 12.8025 19.7515 12.7689 19.7777L11.6808 20C11.4724 19.9453 11.2752 19.836 11.1119 19.6722L10.9939 19.5538L10.4545 18.8558V19.0128L6.98546 15.5333V16.9736H8.05476L9.10138 18.0234H7.23847L6.90909 17.8983L6.67764 18.0234H6.17512ZM16.9416 9.91225L14.0255 6.98735H9.97445L6.98546 9.98536V14.0487L9.90156 16.9736H14.0984L16.9416 14.1218V9.91225Z' fill='%23F9A266'/%3E%3Cpath d='M9.12788 14.3601L8.95988 14.8001L9.44788 14.9601L9.73588 15.0081L9.86388 14.7441L10.0959 13.9441V13.9521L10.2479 13.4641L10.2399 13.4721L10.4639 12.6801L10.9039 14.0001L10.8959 14.0721L11.0879 14.5841V14.7041L11.2719 15.0481L11.9759 14.8321L11.9199 14.4481L11.8719 14.3921L11.7119 14.3841L11.8159 14.3041L11.5439 13.6321L11.4639 13.5281L11.2479 12.8561L11.0319 12.2481L10.9039 11.8401L10.9999 11.5841V11.6001L11.3519 10.8001V10.8081L11.6399 10.1201L11.5759 9.98411L11.7039 9.89611L12.0239 9.24011L11.5039 9.00811H11.4239L11.1919 9.02411L11.0479 9.65611L11.0959 9.69611L11.0239 9.73611L10.8159 10.2881L10.7359 10.5521L10.5359 11.0641L10.5119 11.1601L10.3919 10.8161L10.2959 10.5681L10.2639 10.4881L10.1999 10.1601L10.0239 9.76011L10.0319 9.79211L9.79988 8.99211L9.36788 9.06411L9.00788 9.25611L9.39188 10.1761L9.56788 10.1841L9.44788 10.3201L9.79988 11.1041V11.0961L9.88788 11.3361L9.92788 11.4001L9.98388 11.5281L10.1039 11.7761L9.83188 12.5121L9.84788 12.5761L9.64788 13.0321L9.50388 13.3921L9.36788 13.7441L9.39188 13.9281L9.27988 13.9681L9.12788 14.3601Z' fill='%23F9A266'/%3E%3Cpath d='M14.5202 11.8721L14.7762 11.6241L15.0242 11.3201L15.0402 10.7681L14.2082 9.88811L14.0402 9.83211L14.0082 9.69611L13.2882 8.95211L13.0322 9.04811L12.6482 9.21611L12.6642 9.75211L12.7042 10.0801V11.3921L12.6882 11.4321L12.6802 13.4321L12.7762 13.5361L12.7042 13.7841L12.6162 14.9281L13.0802 14.9761L13.4802 14.9521L13.4322 13.9601L13.3042 13.8481L13.4322 13.6561L13.4162 13.1041L13.4882 13.0241L14.0242 12.4161L14.5202 11.8721ZM13.4242 11.3281L13.3362 11.1681L13.4162 10.9201L13.4082 10.5361L13.4162 9.96811L13.8802 10.4881L13.9522 10.5361L14.0082 10.6481L14.3522 11.0321L14.2242 11.2161L14.1442 11.4161L14.0562 11.3921L13.6562 11.8641L13.4242 12.1121V11.3281Z' fill='%23F9A266'/%3E%3C/svg%3E%0A";

#[near_bindgen]
impl Contract {
  /// Initializes the contract with the given total supply owned by the given `owner_id` with
  /// default metadata (for example purposes only).
  #[init]
  pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
    Self::new(
      owner_id,
      total_supply,
      FungibleTokenMetadata {
        spec: FT_METADATA_SPEC.to_string(),
        name: "Multiverse Fighters XP".to_string(),
        symbol: "MFXP".to_string(),
        icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
        reference: None,
        reference_hash: None,
        decimals: 24,
      },
    )
  }

  /// Initializes the contract with the given total supply owned by the given `owner_id` with
  /// the given fungible token metadata.
  #[init]
  pub fn new(
    owner_id: AccountId,
    total_supply: U128,
    metadata: FungibleTokenMetadata,
  ) -> Self {
    assert!(!env::state_exists(), "Already initialized");
    metadata.assert_valid();
    let mut this = Self {
      token: FungibleToken::new(b"a".to_vec()),
      metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
    };
    this.token.internal_register_account(&owner_id);
    this.token.internal_deposit(&owner_id, total_supply.into());

    FtMint {
      owner_id: &owner_id,
      amount: &total_supply,
      memo: None
    }.emit();

    this
  }

  fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
    log!("Closed @{} with {}", account_id, balance);
  }

  fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
    log!("Account @{} burned {}", account_id, amount);
  }

  fn assert_owner(&self) {
    assert_eq!(env::current_account_id(), env::predecessor_account_id(), "Unauthorized");
  }

  pub fn claim_tokens(&mut self, receiver_id: AccountId, amount: U128) {
    self.assert_owner();

    self.token.internal_deposit(&receiver_id, amount.0);

    FtMint {
      owner_id: &(receiver_id),
      amount: &amount,
      memo: None
    }.emit();
  }

  #[init(ignore_state)]
  #[private]
  pub fn migrate() -> Self {
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct OldFungibleToken {

    }
    #[derive(BorshDeserialize)]
    struct Old {
      token: FungibleToken,
      metadata: LazyOption<FungibleTokenMetadata>,
    }

    let old: Old = env::state_read().expect("Error");

    let token = FungibleToken {
      accounts: old.token.accounts,
      total_supply: old.token.total_supply,
      account_storage_usage: old.token.account_storage_usage,
    };

    Self {
      token,
      metadata: LazyOption::new(b"m".to_vec(), Some(&FungibleTokenMetadata {
        spec: FT_METADATA_SPEC.to_string(),
        name: "Multiverse Fighters XP".to_string(),
        symbol: "MFXP".to_string(),
        icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
        reference: None,
        reference_hash: None,
        decimals: 24,
      })),
    }
  }
}

// macroses

mfight_sdk::impl_fungible_token_core!(Contract, token);
mfight_sdk::impl_fungible_token_storage!(Contract, token);

// impl

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
  fn ft_metadata(&self) -> FungibleTokenMetadata {
    self.metadata.get().unwrap()
  }
}

// tests

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
  use near_sdk::{Balance, testing_env};
  use near_sdk::MockedBlockchain;
  use near_sdk::test_utils::{accounts, VMContextBuilder};

  use super::*;

  const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

  fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
      .current_account_id(accounts(0))
      .signer_account_id(predecessor_account_id.clone())
      .predecessor_account_id(predecessor_account_id);
    builder
  }

  #[test]
  fn test_new() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
    testing_env!(context.is_view(true).build());
    assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
    assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
  }

  #[test]
  #[should_panic(expected = "The contract is not initialized")]
  fn test_default() {
    let context = get_context(accounts(1));
    testing_env!(context.build());
    let _contract = Contract::default();
  }

  #[test]
  fn test_transfer() {
    let mut context = get_context(accounts(2));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
    testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
    // Paying for account registration, aka storage deposit
    contract.storage_deposit(None, None);

    testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
    let transfer_amount = TOTAL_SUPPLY / 3;
    contract.ft_transfer(accounts(1), transfer_amount.into(), None);

    testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
    assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
    assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
  }
}
