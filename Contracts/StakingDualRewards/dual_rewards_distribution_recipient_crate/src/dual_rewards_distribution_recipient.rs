//use crate::data::{self, Allowances, Balances, Nonces};
use crate::data;
use casper_contract::contract_api::runtime;
use casper_types::ApiError;
use contract_utils::{ContractContext, ContractStorage};

#[repr(u16)]
pub enum Error {
    // Caller is not DualRewardsDistribution contract
    NotDualRewardsDistribution = 0,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
pub trait DUALREWARDSDISTRIBUTIONRECIPIENT<Storage: ContractStorage>:
    ContractContext<Storage>
{
    fn init(&self) {}
    fn only_dual_rewards_distribution(&self) {
        if !(self.get_caller() == data::get_dual_rewards_distribution()) {
            runtime::revert(ApiError::from(Error::NotDualRewardsDistribution));
        }
    }
}
