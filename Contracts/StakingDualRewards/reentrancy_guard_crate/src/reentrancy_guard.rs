//use crate::data::{self, Allowances, Balances, Nonces};
use crate::data;
use casper_contract::{contract_api::runtime::{self}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ApiError, U256};
use contract_utils::{ContractContext, ContractStorage, set_key,get_key};

const LOCK: &str = "lock";
#[repr(u16)]
pub enum Error {
    // "ReentrancyGuard: reentrant call"
    ReentrantCall = 20201,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
fn set_lock(lock:bool){
    set_key(LOCK, lock)
}
fn get_lock() -> bool{
    get_key(LOCK).unwrap_or_default()
}
pub trait REENTRANCYGUARD<Storage: ContractStorage>: ContractContext<Storage> {
    fn enter(&self) {
        if get_lock()  {
            runtime::revert(Error::ReentrantCall);
        }
        set_lock(true);
    }
    fn leave(&self){
        set_lock(false);
    }
}
