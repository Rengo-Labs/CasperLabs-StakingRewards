use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, CLTyped, ContractPackageHash, Key, U256};
use contract_utils::{get_key, set_key, Dict};
use core::convert::TryInto;

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const REWARDS_TOKEN_A: &str = "rewards_token_a";
pub const REWARDS_TOKEN_B: &str = "rewards_token_b";
pub const STAKING_TOKEN: &str = "staking_token";
pub const PERIOD_FINISH: &str = "period_finish";
pub const REWARD_RATE_A: &str = "reward_rate_a";
pub const REWARD_RATE_B: &str = "reward_rate_b";
pub const LAST_UPDATE_TIME: &str = "last_update_time";
pub const REWARD_PER_TOKEN_A_STORED: &str = "reward_per_token_a_stored";
pub const REWARD_PER_TOKEN_B_STORED: &str = "reward_per_token_b_stored";
pub const TOTAL_SUPPLY: &str = "total_supply";
pub const TEN_E_NINE:u128 = 1000000000;
pub const RESULT: &str = "result";
//Zero Address
pub fn zero_address() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
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
//Dictionaries
pub const USER_REWARD_PER_TOKEN_A_PAID_DICT: &str = "user_reward_per_token_a_paid";
pub struct UserRewardPerTokenAPaid {
    dict: Dict,
}

impl UserRewardPerTokenAPaid {
    pub fn instance() -> UserRewardPerTokenAPaid {
        UserRewardPerTokenAPaid {
            dict: Dict::instance(USER_REWARD_PER_TOKEN_A_PAID_DICT),
        }
    }

    pub fn init() {
        Dict::init(USER_REWARD_PER_TOKEN_A_PAID_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}
pub const USER_REWARD_PER_TOKEN_B_PAID_DICT: &str = "user_reward_per_token_b_paid";
pub struct UserRewardPerTokenBPaid {
    dict: Dict,
}

impl UserRewardPerTokenBPaid {
    pub fn instance() -> UserRewardPerTokenBPaid {
        UserRewardPerTokenBPaid {
            dict: Dict::instance(USER_REWARD_PER_TOKEN_B_PAID_DICT),
        }
    }

    pub fn init() {
        Dict::init(USER_REWARD_PER_TOKEN_B_PAID_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}
pub const REWARDS_A_DICT: &str = "rewards_a";
pub struct RewardsA {
    dict: Dict,
}

impl RewardsA {
    pub fn instance() -> RewardsA {
        RewardsA {
            dict: Dict::instance(REWARDS_A_DICT),
        }
    }

    pub fn init() {
        Dict::init(REWARDS_A_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}
pub const REWARDS_B_DICT: &str = "rewards_b";
pub struct RewardsB {
    dict: Dict,
}

impl RewardsB {
    pub fn instance() -> RewardsB {
        RewardsB {
            dict: Dict::instance(REWARDS_B_DICT),
        }
    }

    pub fn init() {
        Dict::init(REWARDS_B_DICT)
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
//Setter and Getter
pub fn set_reward_rate_a(reward_rate: U256) {
    set_key(REWARD_RATE_A, reward_rate);
}
pub fn get_reward_rate_a() -> U256 {
    get_key(REWARD_RATE_A).unwrap_or_default()
}
pub fn set_reward_rate_b(reward_rate: U256) {
    set_key(REWARD_RATE_B, reward_rate);
}
pub fn get_reward_rate_b() -> U256 {
    get_key(REWARD_RATE_B).unwrap_or_default()
}
// Period Finish
pub fn set_period_finish(period_finish: U256) {
    set_key(PERIOD_FINISH, period_finish);
}
pub fn get_period_finish() -> U256 {
    get_key(PERIOD_FINISH).unwrap_or_default()
}
pub fn set_last_update_time(last_update_time: U256) {
    set_key(LAST_UPDATE_TIME, last_update_time);
}
pub fn get_last_update_time() -> U256 {
    get_key(LAST_UPDATE_TIME).unwrap_or_revert()
}

pub fn set_rewards_token_a(rewards_token: Key) {
    set_key(REWARDS_TOKEN_A, rewards_token);
}
pub fn get_rewards_token_a() -> Key {
    get_key(REWARDS_TOKEN_A).unwrap_or_revert()
}
pub fn set_rewards_token_b(rewards_token: Key) {
    set_key(REWARDS_TOKEN_B, rewards_token);
}
pub fn get_rewards_token_b() -> Key {
    get_key(REWARDS_TOKEN_B).unwrap_or_revert()
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

pub fn set_reward_per_token_a_stored(reward_per_token_stored: U256) {
    set_key(REWARD_PER_TOKEN_A_STORED, reward_per_token_stored);
}
pub fn get_reward_per_token_a_stored() -> U256 {
    get_key(REWARD_PER_TOKEN_A_STORED).unwrap_or_default()
}
pub fn set_reward_per_token_b_stored(reward_per_token_stored: U256) {
    set_key(REWARD_PER_TOKEN_B_STORED, reward_per_token_stored);
}
pub fn get_reward_per_token_b_stored() -> U256 {
    get_key(REWARD_PER_TOKEN_B_STORED).unwrap_or_default()
}
pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
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
