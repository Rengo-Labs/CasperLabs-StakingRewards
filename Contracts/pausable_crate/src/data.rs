use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, CLTyped, ContractPackageHash, Key, U256};
use contract_utils::{get_key, set_key};
use core::convert::TryInto;

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const LAST_PAUSE_TIME: &str = "last_pause_time";
pub const PAUSED: &str = "paused";
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
pub fn set_last_pause_time(last_pause_time: U256) {
    set_key(LAST_PAUSE_TIME, last_pause_time);
}
pub fn get_last_pause_time() -> U256 {
    get_key(LAST_PAUSE_TIME).unwrap_or_revert()
}
pub fn set_paused(paused: bool) {
    set_key(PAUSED, paused);
}
pub fn get_paused() -> bool {
    get_key(PAUSED).unwrap_or_default()
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
