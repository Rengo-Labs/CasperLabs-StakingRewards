use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, CLTyped, ContractPackageHash, Key};
use contract_utils::{get_key, set_key};
use core::convert::TryInto;

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const REWARDS_DISTRIBUTION: &str = "rewards_distribution";
pub const RESULT: &str = "result";

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
pub fn set_rewards_distribution(rewards_distribution: Key) {
    set_key(REWARDS_DISTRIBUTION, rewards_distribution)
}
pub fn get_rewards_distribution() -> Key {
    get_key(REWARDS_DISTRIBUTION).unwrap_or_revert()
}

