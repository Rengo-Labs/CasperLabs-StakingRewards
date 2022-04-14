use core::str::Bytes;

//use crate::data::{self, Allowances, Balances, Nonces};
use crate::alloc::string::ToString;
use crate::data::{self, Balances, Rewards, UserRewardPerTokenPaid};
use alloc::collections::BTreeMap;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256};
use contract_utils::{set_key, ContractContext, ContractStorage};

use reentrancy_guard_crate::{self, data as reentrancy_guard, REENTRANCYGUARD};
use rewards_distribution_recipient_crate::{
    self, data as rewards_distribution, REWARDSDISTRIBUTIONRECIPIENT,
};

//Errors
#[repr(u16)]
pub enum Error {
    //Cannot stake 0
    CannotStake = 0,
    //Cannot Withdraw 0
    CannotWithdraw = 1,
    //Cannot reduce existing period
    CannotReduce = 2,
    // Provided reward too high
    RewardTooHigh = 3,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
//Events
pub enum StakingRewardsEvent {
    Staked { user: Key, amount: U256 },
    Withdraw { user: Key, amount: U256 },
    RewardPaid { user: Key, reward: U256 },
    RewardAdded { reward: U256, period_finish: U256 },
}

impl StakingRewardsEvent {
    pub fn type_name(&self) -> String {
        match self {
            StakingRewardsEvent::Staked { user: _, amount: _ } => "Staked",
            StakingRewardsEvent::Withdraw { user: _, amount: _ } => "Withdraw",
            StakingRewardsEvent::RewardPaid { user: _, reward: _ } => "Reward Paid",
            StakingRewardsEvent::RewardAdded {
                reward: _,
                period_finish: _,
            } => "Reward Added",
        }
        .to_string()
    }
}
pub trait STAKINGREWARDS<Storage: ContractStorage>:
    ContractContext<Storage> + REENTRANCYGUARD<Storage> + REWARDSDISTRIBUTIONRECIPIENT<Storage>
{
    fn init(
        &self,
        rewards_distribution: Key,
        rewards_token: Key,
        staking_token: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        REENTRANCYGUARD::init(self);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        rewards_distribution::set_rewards_distribution(rewards_distribution);
        data::set_rewards_token(rewards_token);
        data::set_staking_token(staking_token);
        data::Balances::init();
        data::UserRewardPerTokenPaid::init();
        data::Rewards::init();
    }
    fn total_supply(&self) -> U256 {
        return data::get_total_supply();
    }
    fn balance_of(&self, account: Key) -> U256 {
        return Balances::instance().get(&account);
    }
    fn last_time_reward_applicable(&self) -> U256 {
        let blocktime: u64 = runtime::get_blocktime().into();
        return U256::min(U256::from(blocktime), data::get_period_finish());
    }
    fn reward_per_token(&self) -> U256 {
        if data::get_total_supply() == 0.into() {
            return data::get_reward_per_token_stored();
        }
        return data::get_reward_per_token_stored()
            .checked_add(
                self.last_time_reward_applicable()
                    .checked_sub(data::get_last_update_time())
                    .unwrap_or_revert()
                    .checked_mul(data::get_reward_rate())
                    .unwrap_or_revert()
                    .checked_mul(U256::from_dec_str("1000000000000000000").unwrap())
                    .unwrap_or_revert()
                    .checked_div(data::get_total_supply())
                    .unwrap_or_revert(),
            )
            .unwrap_or_revert();
    }
    fn earned(&self, account: Key) -> U256 {
        return Balances::instance()
            .get(&account)
            .checked_mul(
                self.reward_per_token()
                    .checked_sub(UserRewardPerTokenPaid::instance().get(&account))
                    .unwrap_or_revert()
                    .checked_div(U256::from_dec_str("1000000000000000000").unwrap())
                    .unwrap_or_revert()
                    .checked_add(Rewards::instance().get(&account))
                    .unwrap_or_revert(),
            )
            .unwrap_or_revert();
    }
    fn stake_with_permit(&mut self, amount: U256, deadline: U256,public_key: String, signature: String) {
       REENTRANCYGUARD::non_reentrant(self);
        self.update_reward(self.get_caller());
        if !(amount > 0.into()) {
            runtime::revert(ApiError::from(Error::CannotStake));
        }
        data::set_total_supply(
            data::get_total_supply()
                .checked_add(amount)
                .unwrap_or_revert(),
        );
        Balances::instance().set(
            &self.get_caller(),
            Balances::instance()
                .get(&self.get_caller())
                .checked_add(amount)
                .unwrap_or_revert(),
        );
        //let deadline_:u64 = u64::from(deadline);
        // let () = runtime::call_versioned_contract(
        //     data::get_staking_token()
        //         .into_hash()
        //         .unwrap_or_revert()
        //         .into(),
        //       None,
        //     "permit",
        //     runtime_args! {
        //         "public_key" => public_key,
        //         "signature" => signature,
        //         "owner" => self.get_caller(),
        //         "spender" => Key::from(data::get_package_hash()),
        //         "value" => amount,
        //         "deadline" => deadline
        //     },
        // );
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            data::get_staking_token()
                .into_hash()
                .unwrap_or_revert()
                .into(),
            None,
            "transfer_from",
            runtime_args! {
                "owner" => self.get_caller(),
                "recipient" => Key::from(data::get_package_hash()),
                "amount" => amount
            },
        );
        match ret {
            Ok(()) => {}
            Err(e) => runtime::revert(ApiError::User(e as u16)),
        }
        self.staking_rewards_emit(&StakingRewardsEvent::Staked {
            user: self.get_caller(),
            amount: amount,
        });
    }
    fn stake(&mut self, amount: U256) {
        REENTRANCYGUARD::non_reentrant(self);
        self.update_reward(self.get_caller());
        if !(amount > 0.into()) {
            runtime::revert(ApiError::from(Error::CannotStake));
        }
        data::set_total_supply(
            data::get_total_supply()
                .checked_add(amount)
                .unwrap_or_revert(),
        );
        Balances::instance().set(
            &self.get_caller(),
            Balances::instance()
                .get(&self.get_caller())
                .checked_add(amount)
                .unwrap_or_revert(),
        );
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            data::get_staking_token()
                .into_hash()
                .unwrap_or_revert()
                .into(),
            None,
            "transfer_from",
            runtime_args! {
                "owner" => self.get_caller(),
                "recipient" => Key::from(data::get_package_hash()),
                "amount" => amount
            },
        );
        match ret {
            Ok(()) => {}
            Err(e) => runtime::revert(ApiError::User(e as u16)),
        }
        self.staking_rewards_emit(&StakingRewardsEvent::Staked {
            user: self.get_caller(),
            amount: amount,
        });
    }
    fn withdraw(&mut self, amount: U256) {
        REENTRANCYGUARD::non_reentrant(self);
        self.update_reward(self.get_caller());
        if !(amount > 0.into()) {
            runtime::revert(ApiError::from(Error::CannotWithdraw));
        }
        data::set_total_supply(
            data::get_total_supply()
                .checked_sub(amount)
                .unwrap_or_revert(),
        );
        Balances::instance().set(
            &self.get_caller(),
            Balances::instance()
                .get(&self.get_caller())
                .checked_sub(amount)
                .unwrap_or_revert(),
        );
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            data::get_staking_token()
                .into_hash()
                .unwrap_or_revert()
                .into(),
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
        self.staking_rewards_emit(&StakingRewardsEvent::Withdraw {
            user: self.get_caller(),
            amount: amount,
        });
    }
    fn get_reward(&mut self) {
        REENTRANCYGUARD::non_reentrant(self);
        self.update_reward(self.get_caller());
        let reward: U256 = Rewards::instance().get(&self.get_caller());
        if reward > 0.into() {
            Rewards::instance().set(&self.get_caller(), 0.into());
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                data::get_rewards_token()
                    .into_hash()
                    .unwrap_or_revert()
                    .into(),
                None,
                "transfer",
                runtime_args! {
                    "recipient" => self.get_caller(),
                    "amount" => reward
                },
            );
            match ret {
                Ok(()) => {}
                Err(e) => runtime::revert(ApiError::User(e as u16)),
            }
            self.staking_rewards_emit(&StakingRewardsEvent::RewardPaid {
                user: self.get_caller(),
                reward: reward,
            });
        }
    }
    fn exit(&mut self) {
        self.withdraw(Balances::instance().get(&self.get_caller()));
        self.get_reward();
    }
    fn notify_reward_amount(&mut self, reward: U256, rewards_duration: U256) {
        REWARDSDISTRIBUTIONRECIPIENT::only_rewards_distribution(self);
        self.update_reward(data::ZERO_ADDRESS());
        let blocktime: u64 = runtime::get_blocktime().into();

        if !(U256::from(blocktime)
            .checked_add(rewards_duration)
            .unwrap_or_revert()
            >= data::get_period_finish())
        {
            runtime::revert(ApiError::from(Error::CannotReduce));
        }
        if U256::from(blocktime) >= data::get_period_finish() {
            data::set_reward_rate(reward.checked_div(rewards_duration).unwrap_or_revert());
        } else {
            let remaining: U256 = data::get_period_finish()
                .checked_sub(U256::from(blocktime))
                .unwrap_or_revert();
            let left_over: U256 = remaining
                .checked_mul(data::get_reward_rate())
                .unwrap_or_revert();
            data::set_reward_rate(
                data::get_reward_rate()
                    .checked_add(left_over)
                    .unwrap_or_revert()
                    .checked_div(rewards_duration)
                    .unwrap_or_revert(),
            );
        }

        // Ensure the provided reward amount is not more than the balance in the contract.
        // This keeps the reward rate in the right range, preventing overflows due to
        // very high values of rewardRate in the earned and rewardsPerToken functions;
        // Reward + leftover must be less than 2^256 / 10^18 to avoid overflow.
        let balance: U256 = runtime::call_versioned_contract(
            data::get_rewards_token()
                .into_hash()
                .unwrap_or_revert()
                .into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Key::from(data::get_package_hash())
            },
        );
        if !(data::get_reward_rate() <= balance.checked_div(rewards_duration).unwrap_or_revert()) {
            runtime::revert(ApiError::from(Error::RewardTooHigh));
        }
        data::set_last_update_time(U256::from(blocktime));
        data::set_period_finish(
            U256::from(blocktime)
                .checked_add(rewards_duration)
                .unwrap_or_revert(),
        );
        //set_key("result",data::get_period_finish());
        self.staking_rewards_emit(&StakingRewardsEvent::RewardAdded {
            reward: reward,
            period_finish: data::get_period_finish(),
        });
    }

    fn update_reward(&self, account: Key) {
        data::set_reward_per_token_stored(self.reward_per_token());
        data::set_last_update_time(self.last_time_reward_applicable());
        if account != data::ZERO_ADDRESS() {
            Rewards::instance().set(&account, self.earned(account));
            UserRewardPerTokenPaid::instance().set(&account, data::get_reward_per_token_stored());
        }
    }

    fn staking_rewards_emit(&mut self, staking_rewards_event: &StakingRewardsEvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match staking_rewards_event {
            StakingRewardsEvent::Staked { user, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", staking_rewards_event.type_name());
                event.insert("user", user.to_string());
                event.insert("amount", amount.to_string());
                events.push(event);
            }
            StakingRewardsEvent::Withdraw { user, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", staking_rewards_event.type_name());
                event.insert("user", user.to_string());
                event.insert("amount", amount.to_string());
                events.push(event);
            }
            StakingRewardsEvent::RewardPaid { user, reward } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", staking_rewards_event.type_name());
                event.insert("user", user.to_string());
                event.insert("reward", reward.to_string());
                events.push(event);
            }
            StakingRewardsEvent::RewardAdded {
                reward,
                period_finish,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", staking_rewards_event.type_name());
                event.insert("reward", reward.to_string());
                event.insert("period_finish", period_finish.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
