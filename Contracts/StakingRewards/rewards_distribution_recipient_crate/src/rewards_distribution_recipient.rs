//use crate::data::{self, Allowances, Balances, Nonces};
use crate::data;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ApiError, ContractPackageHash, Key, U256};
use contract_utils::{ContractContext, ContractStorage};

#[repr(u16)]
pub enum Error {
    //Caller is not RewardsDistribution contract
    CallerNotRewardsDistributor = 0,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
pub trait REWARDSDISTRIBUTIONRECIPIENT<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self) {}
    //Modifier
    fn only_rewards_distribution(&self) {
        if !(self.get_caller() == data::get_rewards_distribution()) {
            runtime::revert(ApiError::from(Error::CallerNotRewardsDistributor));
        }
    }
}
