//use crate::data::{self, Allowances, Balances, Nonces};
use crate::data;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ApiError, ContractPackageHash, Key, U256};
use contract_utils::{ContractContext, ContractStorage};

#[repr(u16)]
pub enum Error {
    // "ReentrancyGuard: reentrant call"
    ReentrantCall = 0,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
pub trait REENTRANCYGUARD<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self) {
        data::set_guard_counter(1.into());
      
    }
    fn non_reentrant(&self) {
        data::set_guard_counter(
            data::get_guard_counter()
                .checked_add(1.into())
                .unwrap_or_revert(),
        );
        let local_counter: U256 = data::get_guard_counter();
        if !(local_counter == data::get_guard_counter()) {
            runtime::revert(ApiError::from(Error::ReentrantCall));
        }
    }
}
