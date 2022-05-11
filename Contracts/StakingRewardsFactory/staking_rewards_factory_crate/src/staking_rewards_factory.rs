use crate::alloc::string::ToString;
use crate::data::{self,StakingTokens};
use alloc::collections::{BTreeSet};
use alloc::format;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::bytesrepr::{FromBytes, ToBytes};
use casper_types::{
    runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, ContractStorage};
use ownable_crate::{self, OWNABLE};
use staking_rewards_crate::{self,entry_point, STAKINGREWARDS};
//Errors
//#[derive(Default)]
#[repr(u16)]
pub enum Error {
    //StakingRewardsFactory::constructor: genesis too soon
    GenesisTooSoon = 0,
    //StakingRewardsFactory::deploy: already deployed
    StakingRewardsFactoryAlreadyDeploy = 1,
    //StakingRewardsFactory::update: not deployed
    StakingRewardsFactoryUpdateNotDeploy = 2,
    // StakingRewardsFactory::notifyRewardAmounts: called before any deploys
    CalledBeforeAnyDeploys = 3,
    // StakingRewardsFactory::notifyRewardAmount: not ready
    NotifyRewardAmountNotReady = 4,
    //StakingRewardsFactory::notifyRewardAmount: not deployed
    NotifyRewardAmountNotDeploy = 5,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
pub trait STAKINGREWARDSFACTORY<Storage: ContractStorage>:
    ContractContext<Storage> + OWNABLE<Storage> + STAKINGREWARDS<Storage>
{
    fn init(
        &mut self,
        rewards_token: Key,
        staking_rewards_genesis: U256,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        OWNABLE::init(self, contract_hash, package_hash);
        let blocktime: u64 = runtime::get_blocktime().into();
        if !(staking_rewards_genesis >= U256::from(blocktime)) {
            runtime::revert(ApiError::from(Error::GenesisTooSoon));
        }
        data::StakingTokens::init();
        data::StakingRewardsInfoByStakingTokenDict::init();
        data::set_rewards_token(rewards_token);
        data::set_staking_rewards_genesis(staking_rewards_genesis);
    }
    fn deploy(&self, staking_token: Key, reward_amount: U256, rewards_duration: U256) {
        let staking_rewards_info = data::StakingRewardsInfoByStakingTokenDict::instance();
        let info: Vec<u8> = staking_rewards_info.get(&staking_token);
        let mut info: data::StakingRewardsInfo =
            data::StakingRewardsInfo::from_bytes(&info).unwrap().0;
        if !(info.staking_rewards == data::ZERO_ADDRESS()) {
            runtime::revert(ApiError::from(Error::StakingRewardsFactoryAlreadyDeploy));
        }
        let v: String = data::get_counter().to_string();
        let name: String = "StakingRewards".to_string() + &v;
        let (package_hash, _) = storage::create_contract_package_at_hash();
        let (contract_hash, _) = storage::add_contract_version(
            package_hash,
            entry_point::get_entry_points(),
            Default::default(),
        );
        runtime::put_key(&format!("{}_contract", name), contract_hash.into());
        info.staking_rewards = Key::from(package_hash);
        // Access
        let constructor_access: URef =
            storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
                .unwrap_or_revert()
                .pop()
                .unwrap_or_revert();

        // Call the constructor entry point
        let _: () = runtime::call_versioned_contract(
            package_hash,
            None,
            "constructor_sr",
            runtime_args! {
                "rewards_distribution" => Key::from(data::get_package_hash()),
                "rewards_token" => data::get_rewards_token(),
                "staking_token" => staking_token,
                "contract_hash" => contract_hash,
                "package_hash"=> package_hash
            },
        );

        // Remove all URefs from the constructor group, so no one can call it for the second time.
        let mut urefs = BTreeSet::new();
        urefs.insert(constructor_access);
        storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
            .unwrap_or_revert();

        info.reward_amount = reward_amount;
        info.duration = rewards_duration;
        StakingTokens::instance().set(&data::get_counter(), staking_token);
        data::set_counter(data::get_counter().checked_add(1.into()).unwrap_or_revert());
        staking_rewards_info.set(&staking_token, info.clone().into_bytes().unwrap());
    }
    fn update(&self, staking_token: Key, reward_amount: U256, rewards_duration: U256) {
        OWNABLE::only_owner(self);
        let staking_rewards_info = data::StakingRewardsInfoByStakingTokenDict::instance();
        let info: Vec<u8> = staking_rewards_info.get(&staking_token);
        let mut info: data::StakingRewardsInfo =
            data::StakingRewardsInfo::from_bytes(&info).unwrap().0;
        if !(info.staking_rewards != data::ZERO_ADDRESS()) {
            runtime::revert(ApiError::from(Error::StakingRewardsFactoryUpdateNotDeploy));
        }
        info.reward_amount = reward_amount;
        info.duration = rewards_duration;
        staking_rewards_info.set(&staking_token, info.clone().into_bytes().unwrap());
    }
    fn notify_reward_amounts(&self) {
        if !(data::get_counter() > 0.into()) {
            runtime::revert(ApiError::from(Error::CalledBeforeAnyDeploys));
        }
        let range: U256 = data::get_counter();
        for i in 0..range.as_u128() {
            self.notify_reward_amount(data::StakingTokens::instance().get(&U256::from(i)));
        }
    }
    fn notify_reward_amount(&self, staking_token: Key) {
        let blocktime: u64 = runtime::get_blocktime().into();
        if !(U256::from(blocktime) >= data::get_staking_rewards_genesis()) {
            runtime::revert(ApiError::from(Error::NotifyRewardAmountNotReady));
        }
        let staking_rewards_info = data::StakingRewardsInfoByStakingTokenDict::instance();
        let info: Vec<u8> = staking_rewards_info.get(&staking_token);
        let mut info: data::StakingRewardsInfo =
            data::StakingRewardsInfo::from_bytes(&info).unwrap().0;
        if !(info.staking_rewards != data::ZERO_ADDRESS()) {
            runtime::revert(ApiError::from(Error::NotifyRewardAmountNotDeploy));
        }
        if info.reward_amount > 0.into() && info.duration > 0.into() {
            let reward_amount: U256 = info.reward_amount;
            let duration: U256 = info.duration;
            info.reward_amount = 0.into();
            info.duration = 0.into();
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                data::get_rewards_token()
                    .into_hash()
                    .unwrap_or_revert()
                    .into(),
                None,
                "transfer",
                runtime_args! {
                    "recipient" => info.staking_rewards,
                    "amount" => reward_amount
                },
            );
            match ret {
                Ok(()) => {}
                Err(e) => runtime::revert(ApiError::User(e as u16)),
            }
            let () = runtime::call_versioned_contract(
                info.staking_rewards.into_hash().unwrap_or_revert().into(),
                None,
                "notify_reward_amount_sr",
                runtime_args! {
                    "reward" => info.reward_amount,
                    "rewards_duration" => duration
                },
            );
        }
    }
    fn pull_extra_tokens(&self, token: Key, amount: U256) {
        OWNABLE::only_owner(self);
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => self.get_caller(),
                "amount" => amount
            },
        );
        match ret {
            Ok(()) => {}
            Err(e) => runtime::revert(ApiError::User(e as u16)),
        }
    }
}