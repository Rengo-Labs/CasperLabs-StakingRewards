use core::convert::TryInto;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use alloc::{string::ToString, vec::Vec};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use casper_types::{bytesrepr::ToBytes, CLTyped, ContractPackageHash, Key, U256};
use contract_utils::{get_key, set_key, Dict};


pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const STAKING_REWARDS_GENESIS: &str = "staking_rewards_genesis";
pub const COUNTER: &str = "counter";
pub const STAKING_REWARDS_INFO_BY_STAKING_TOKEN_DICT: &str = "staking_rewards_info";
pub const RESULT: &str = "result";

//Struct

#[derive(Clone, CLTyped, ToBytes, FromBytes,Debug)]
pub struct StakingRewardsInfo {
    pub staking_rewards: Key,
    pub rewards_token_a: Key,
    pub rewards_token_b: Key,
    pub reward_amount_a:U256,
    pub reward_amount_b:U256,
    pub duration: U256,
}
impl StakingRewardsInfo {
    pub fn new() -> StakingRewardsInfo {
        StakingRewardsInfo {
            staking_rewards: zero_address(),
            rewards_token_a: zero_address(),
            rewards_token_b: zero_address(),
            reward_amount_a: 0.into(),
            reward_amount_b: 0.into(),
            duration: 0.into(),
        }
    }
}
pub struct StakingRewardsInfoByStakingTokenDict{
    dict: Dict,
}
impl StakingRewardsInfoByStakingTokenDict {
    pub fn instance() -> StakingRewardsInfoByStakingTokenDict {
        StakingRewardsInfoByStakingTokenDict {
            dict: Dict::instance(STAKING_REWARDS_INFO_BY_STAKING_TOKEN_DICT),
        }
    }
    pub fn init() {
        Dict::init(STAKING_REWARDS_INFO_BY_STAKING_TOKEN_DICT)
    }
    pub fn get(&self, key: &Key) -> Vec<u8> {
        let mut result: Vec<u8> = self.dict.get_by_key(key).unwrap_or_default();
        if result.is_empty() {
            result = StakingRewardsInfo::new().into_bytes().unwrap();
        }
        result
    }

    pub fn set(&self, key: &Key, value: Vec<u8>) {
        self.dict.set_by_key(key, value);
    }
}


//Zero Address
pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}
//Dict

pub const STAKING_TOKENS_DICT: &str = "staking_tokens";
pub struct StakingTokens {
    dict: Dict,
}

impl StakingTokens {
    pub fn instance() -> StakingTokens {
        StakingTokens {
            dict: Dict::instance(STAKING_TOKENS_DICT),
        }
    }

    pub fn init() {
        Dict::init(STAKING_TOKENS_DICT)
    }

    pub fn get(&self, counter: &U256) -> Key {
        self.dict.get(counter.to_string().as_str()).unwrap_or_revert()
    }

    pub fn set(&self, counter: &U256, value: Key) {
        self.dict.set(counter.to_string().as_str(), value);
    }
}

pub fn set_result<T: ToBytes + CLTyped>(value: T) {
    match runtime::get_key(RESULT) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(RESULT, key);
        }
    }
}

// Period Finish
pub fn set_counter(counter: U256) {
    set_key(COUNTER, counter);
}
pub fn get_counter() -> U256 {
    get_key(COUNTER).unwrap_or_default()
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}
pub fn set_staking_rewards_genesis(staking_rewards_genesis: U256) {
    set_key(STAKING_REWARDS_GENESIS, staking_rewards_genesis);
}
pub fn get_staking_rewards_genesis() -> U256 {
    get_key(STAKING_REWARDS_GENESIS).unwrap_or_revert()
}
pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(SELF_PACKAGE_HASH).unwrap_or_revert()
}
pub fn js_ret<T: CLTyped + ToBytes>(ret: T) {
    set_key(RESULT, ret);
}