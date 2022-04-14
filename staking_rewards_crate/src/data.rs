use core::convert::TryInto;

use alloc::string::String;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, CLTyped, ContractPackageHash, Key, U256};
use contract_utils::{get_key, set_key, Dict};

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const REWARDS_TOKEN: &str = "rewards_token";
pub const STAKING_TOKEN: &str = "staking_token";
pub const PERIOD_FINISH: &str = "period_finish";
pub const REWARD_RATE: &str = "reward_rate";
pub const LAST_UPDATE_TIME: &str = "last_update_time";
pub const REWARD_PER_TOKEN_STORED: &str = "reward_per_token_stored";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const RESULT: &str = "result";
//Zero Address
pub fn ZERO_ADDRESS() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}
//Dict

pub const USER_REWARD_PER_TOKEN_PAID_DICT: &str = "user_reward_per_token_paid";
pub struct UserRewardPerTokenPaid {
    dict: Dict,
}

impl UserRewardPerTokenPaid {
    pub fn instance() -> UserRewardPerTokenPaid {
        UserRewardPerTokenPaid {
            dict: Dict::instance(USER_REWARD_PER_TOKEN_PAID_DICT),
        }
    }

    pub fn init() {
        Dict::init(USER_REWARD_PER_TOKEN_PAID_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub const REWARDS_DICT: &str = "rewards";
pub struct Rewards {
    dict: Dict,
}

impl Rewards {
    pub fn instance() -> Rewards {
        Rewards {
            dict: Dict::instance(REWARDS_DICT),
        }
    }

    pub fn init() {
        Dict::init(REWARDS_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub const BALACNES: &str = "balances";
pub struct Balances {
    dict: Dict,
}

impl Balances {
    pub fn instance() -> Balances {
        Balances {
            dict: Dict::instance(BALACNES),
        }
    }

    pub fn init() {
        Dict::init(BALACNES)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value)
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
pub fn set_reward_rate(reward_rate: U256) {
    set_key(REWARD_RATE, reward_rate);
}
pub fn get_reward_rate() -> U256 {
    get_key(REWARD_RATE).unwrap_or_default()
}
// Period Finish
pub fn set_period_finish(period_finish: U256) {
    set_key(PERIOD_FINISH, period_finish);
}
pub fn get_period_finish() -> U256 {
    get_key(PERIOD_FINISH).unwrap_or_default()
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn set_last_update_time(last_update_time: U256) {
    set_key(LAST_UPDATE_TIME, last_update_time);
}
pub fn get_last_update_time() -> U256 {
    get_key(LAST_UPDATE_TIME).unwrap_or_revert()
}

pub fn set_rewards_token(rewards_token: Key) {
    set_key(REWARDS_TOKEN, rewards_token);
}
pub fn get_rewards_token() -> Key {
    get_key(REWARDS_TOKEN).unwrap_or_revert()
}

pub fn set_staking_token(staking_token: Key) {
    set_key(STAKING_TOKEN, staking_token);
}
pub fn get_staking_token() -> Key {
    get_key(STAKING_TOKEN).unwrap_or_revert()
}

pub fn set_total_supply(total_supply: U256) {
    set_key(TOTAL_SUPPLY, total_supply);
}
pub fn get_total_supply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_default()
}

pub fn set_reward_per_token_stored(reward_per_token_stored: U256) {
    set_key(REWARD_PER_TOKEN_STORED, reward_per_token_stored);
}
pub fn get_reward_per_token_stored() -> U256 {
    get_key(REWARD_PER_TOKEN_STORED).unwrap_or_default()
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
