use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, CLTyped, U256};
use contract_utils::{get_key, set_key};
use core::convert::TryInto;

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const GUARD_COUNTER: &str = "guard_counter";
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

pub fn set_guard_counter(guard_counter: U256) {
    set_key(GUARD_COUNTER, guard_counter);
}
pub fn get_guard_counter() -> U256 {
    get_key(GUARD_COUNTER).unwrap_or_revert()
}

