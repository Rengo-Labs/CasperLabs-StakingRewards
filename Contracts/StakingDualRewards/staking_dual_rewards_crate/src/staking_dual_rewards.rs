use crate::alloc::string::ToString;
use crate::data::{
    self, Balances, RewardsA, RewardsB, UserRewardPerTokenAPaid, UserRewardPerTokenBPaid,
};
use alloc::collections::BTreeMap;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256};
use contract_utils::{ContractContext, ContractStorage};
use dual_rewards_distribution_recipient_crate::{
    self, data as dual, DUALREWARDSDISTRIBUTIONRECIPIENT,
};
use owned_crate::{self, data as owned, OWNED};
use pausable_crate::PAUSABLE;
use reentrancy_guard_crate::REENTRANCYGUARD;
//Errors
#[repr(u16)]
pub enum Error {
    /// rewards tokens should be different
    RewardsTokensSame = 20301,
    /// Cannot stake 0
    CannotStake = 20302,
    /// Cannot Withdraw 0
    CannotWithdraw = 20303,
    /// Cannot reduce existing period
    CannotReduce = 20304,
    /// Provided reward-A too high
    RewardATooHigh =20305,
    /// Provided reward-B too high
    RewardBTooHigh = 20306,
    /// Cannot withdraw the staking token
    CannotWithdrawStakingToken = 20307,
    /// Arithmatic Error 1
    ArithmaticError1 = 20308,
    /// Arithmatic Error 2
    ArithmaticError2 = 20309,
    /// Arithmatic Error 3
    ArithmaticError3 = 20310,
    /// Arithmatic Error 4
    ArithmaticError4 = 20311,
    /// Arithmatic Error 5
    ArithmaticError5 = 20312,
    /// Arithmatic Error 6
    ArithmaticError6 = 20313,
    /// Arithmatic Error 7
    ArithmaticError7 = 20314,
    /// Arithmatic Error 8
    ArithmaticError8 = 20315,
    /// Arithmatic Error 9
    ArithmaticError9 = 20316,
    /// Arithmatic Error 10
    ArithmaticError10 = 20317,
    /// Arithmatic Error 11
    ArithmaticError11 = 20318,
    /// Arithmatic Error 12
    ArithmaticError12 = 20319,
    /// Arithmatic Error 13
    ArithmaticError13 = 20320,
    /// Arithmatic Error 14
    ArithmaticError14 = 20321,
    /// Arithmatic Error 15
    ArithmaticError15 = 20322,
    /// Arithmatic Error 16
    ArithmaticError16 = 20323,
    /// Arithmatic Error 17
    ArithmaticError17 = 20324,
    /// Arithmatic Error 18
    ArithmaticError18 = 20325,
    /// Arithmatic Error 19
    ArithmaticError19 = 20326,
    /// Arithmatic Error 20
    ArithmaticError20 = 20327,
    /// Arithmatic Error 21
    ArithmaticError21 = 20328,

}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
//Events
pub enum StakingDualRewardsEvent {
    Staked {
        user: Key,
        amount: U256,
    },
    Withdraw {
        user: Key,
        amount: U256,
    },
    Recovered {
        user: Key,
        amount: U256,
    },
    RewardPaid {
        user: Key,
        reward_token: Key,
        reward: U256,
    },
    RewardAdded {
        reward_a: U256,
        reward_b: U256,
        period_finish: U256,
    },
}

impl StakingDualRewardsEvent {
    pub fn type_name(&self) -> String {
        match self {
            StakingDualRewardsEvent::Staked { user: _, amount: _ } => "Staked",
            StakingDualRewardsEvent::Withdraw { user: _, amount: _ } => "Withdraw",
            StakingDualRewardsEvent::Recovered { user: _, amount: _ } => "Recovered",
            StakingDualRewardsEvent::RewardPaid {
                user: _,
                reward_token: _,
                reward: _,
            } => "Reward Paid",
            StakingDualRewardsEvent::RewardAdded {
                reward_a: _,
                reward_b: _,
                period_finish: _,
            } => "Reward Added",
        }
        .to_string()
    }
}
pub trait STAKINGDUALREWARDS<Storage: ContractStorage>:
    ContractContext<Storage>
    + OWNED<Storage>
    + REENTRANCYGUARD<Storage>
    + PAUSABLE<Storage>
    + DUALREWARDSDISTRIBUTIONRECIPIENT<Storage>
{
    fn init(
        &mut self,
        owner: Key,
        dual_rewards_distribution: Key,
        rewards_token_a: Key,
        rewards_token_b: Key,
        staking_token: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        OWNED::init(self, owner, contract_hash, package_hash);
        PAUSABLE::init(self, contract_hash, package_hash);
        if !(rewards_token_a != rewards_token_b) {
            runtime::revert(ApiError::from(Error::RewardsTokensSame));
        }
        data::Balances::init();
        data::UserRewardPerTokenAPaid::init();
        data::RewardsA::init();
        data::UserRewardPerTokenBPaid::init();
        data::RewardsB::init();
        dual::set_dual_rewards_distribution(dual_rewards_distribution);
        data::set_rewards_token_a(rewards_token_a);
        data::set_rewards_token_b(rewards_token_b);
        data::set_staking_token(staking_token);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
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
    fn reward_per_token_a(&self) -> U256 {
        if data::get_total_supply() == 0.into() {
            return data::get_reward_per_token_a_stored();
        }
        return data::get_reward_per_token_a_stored()
            .checked_add(
                self.last_time_reward_applicable()
                    .checked_sub(data::get_last_update_time())
                    .unwrap_or_revert_with(Error::ArithmaticError1)
                    .checked_mul(data::get_reward_rate_a())
                    .unwrap_or_revert_with(Error::ArithmaticError1)
                    .checked_mul(U256::from(data::TEN_E_NINE))
                    .unwrap_or_revert_with(Error::ArithmaticError1)
                    .checked_div(data::get_total_supply())
                    .unwrap_or_revert_with(Error::ArithmaticError1),
            )
            .unwrap_or_revert_with(Error::ArithmaticError1);
    }
    fn reward_per_token_b(&self) -> U256 {
        if data::get_total_supply() == 0.into() {
            return data::get_reward_per_token_b_stored();
        }
        return data::get_reward_per_token_b_stored()
            .checked_add(
                self.last_time_reward_applicable()
                    .checked_sub(data::get_last_update_time())
                    .unwrap_or_revert_with(Error::ArithmaticError2)
                    .checked_mul(data::get_reward_rate_b())
                    .unwrap_or_revert_with(Error::ArithmaticError2)
                    .checked_mul(U256::from(data::TEN_E_NINE))
                    .unwrap_or_revert_with(Error::ArithmaticError2)
                    .checked_div(data::get_total_supply())
                    .unwrap_or_revert_with(Error::ArithmaticError2),
            )
            .unwrap_or_revert_with(Error::ArithmaticError2);
    }
    fn earned_a(&self, account: Key) -> U256 {
        return Balances::instance()
            .get(&account)
            .checked_mul(
                self.reward_per_token_a()
                    .checked_sub(UserRewardPerTokenAPaid::instance().get(&account))
                    .unwrap_or_revert_with(Error::ArithmaticError3)
                    .checked_div(U256::from(data::TEN_E_NINE))
                    .unwrap_or_revert_with(Error::ArithmaticError3)
                    .checked_add(RewardsA::instance().get(&account))
                    .unwrap_or_revert_with(Error::ArithmaticError3),
            )
            .unwrap_or_revert_with(Error::ArithmaticError3);
    }
    fn earned_b(&self, account: Key) -> U256 {
        return Balances::instance()
            .get(&account)
            .checked_mul(
                self.reward_per_token_b()
                    .checked_sub(UserRewardPerTokenBPaid::instance().get(&account))
                    .unwrap_or_revert_with(Error::ArithmaticError4)
                    .checked_div(U256::from(data::TEN_E_NINE))
                    .unwrap_or_revert_with(Error::ArithmaticError4)
                    .checked_add(RewardsB::instance().get(&account))
                    .unwrap_or_revert_with(Error::ArithmaticError4),
            )
            .unwrap_or_revert_with(Error::ArithmaticError4);
    }
    fn stake(&mut self, amount: U256) {
        REENTRANCYGUARD::enter(self);
        PAUSABLE::not_paused(self);
        self.update_reward(self.get_caller());
        if !(amount > 0.into()) {
            runtime::revert(ApiError::from(Error::CannotStake));
        }
        data::set_total_supply(
            data::get_total_supply()
                .checked_add(amount)
                .unwrap_or_revert_with(Error::ArithmaticError5),
        );
        Balances::instance().set(
            &self.get_caller(),
            Balances::instance()
                .get(&self.get_caller())
                .checked_add(amount)
                .unwrap_or_revert_with(Error::ArithmaticError6),
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
        self.staking_dual_rewards_emit(&StakingDualRewardsEvent::Staked {
            user: self.get_caller(),
            amount: amount,
        });
        REENTRANCYGUARD::leave(self);
    }
    fn withdraw(&mut self, amount: U256) {
        REENTRANCYGUARD::enter(self);
        self.update_reward(self.get_caller());
        if !(amount > 0.into()) {
            runtime::revert(ApiError::from(Error::CannotWithdraw));
        }
        data::set_total_supply(
            data::get_total_supply()
                .checked_sub(amount)
                .unwrap_or_revert_with(Error::ArithmaticError7),
        );
        Balances::instance().set(
            &self.get_caller(),
            Balances::instance()
                .get(&self.get_caller())
                .checked_sub(amount)
                .unwrap_or_revert_with(Error::ArithmaticError8),
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
        self.staking_dual_rewards_emit(&StakingDualRewardsEvent::Withdraw {
            user: self.get_caller(),
            amount: amount,
        });
        REENTRANCYGUARD::leave(self);
    }
    fn get_reward(&mut self) {
        REENTRANCYGUARD::enter(self);
        self.update_reward(self.get_caller());
        let reward_amount_a: U256 = RewardsA::instance().get(&self.get_caller());
        if reward_amount_a > 0.into() {
            RewardsA::instance().set(&self.get_caller(), 0.into());
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                data::get_rewards_token_a()
                    .into_hash()
                    .unwrap_or_revert()
                    .into(),
                None,
                "transfer",
                runtime_args! {
                    "recipient" => self.get_caller(),
                    "amount" => reward_amount_a
                },
            );
            self.staking_dual_rewards_emit(&StakingDualRewardsEvent::RewardPaid {
                user: self.get_caller(),
                reward_token: data::get_rewards_token_a(),
                reward: reward_amount_a,
            });
        }
        let reward_amount_b: U256 = RewardsB::instance().get(&self.get_caller());
        if reward_amount_b > 0.into() {
            RewardsB::instance().set(&self.get_caller(), 0.into());
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                data::get_rewards_token_b()
                    .into_hash()
                    .unwrap_or_revert()
                    .into(),
                None,
                "transfer",
                runtime_args! {
                    "recipient" => self.get_caller(),
                    "amount" => reward_amount_b
                },
            );
            self.staking_dual_rewards_emit(&StakingDualRewardsEvent::RewardPaid {
                user: self.get_caller(),
                reward_token: data::get_rewards_token_b(),
                reward: reward_amount_b,
            });
        }
        REENTRANCYGUARD::leave(self);
    }
    fn exit(&mut self) {
        self.withdraw(Balances::instance().get(&self.get_caller()));
        self.get_reward();
    }
    fn notify_reward_amount(&mut self, reward_a: U256, reward_b: U256, rewards_duration: U256) {
        DUALREWARDSDISTRIBUTIONRECIPIENT::only_dual_rewards_distribution(self);
        self.update_reward(data::ZERO_ADDRESS());
        let blocktime: u64 = runtime::get_blocktime().into();

        if !(U256::from(blocktime)
            .checked_add(rewards_duration)
            .unwrap_or_revert_with(Error::ArithmaticError9)
            >= data::get_period_finish())
        {
            runtime::revert(ApiError::from(Error::CannotReduce));
        }
        if U256::from(blocktime) >= data::get_period_finish() {
            data::set_reward_rate_a(reward_a.checked_div(rewards_duration).unwrap_or_revert_with(Error::ArithmaticError10));
            data::set_reward_rate_b(reward_b.checked_div(rewards_duration).unwrap_or_revert_with(Error::ArithmaticError11));
        } else {
            let remaining: U256 = data::get_period_finish()
                .checked_sub(U256::from(blocktime))
                .unwrap_or_revert_with(Error::ArithmaticError12);
            let left_over_a: U256 = remaining
                .checked_mul(data::get_reward_rate_a())
                .unwrap_or_revert_with(Error::ArithmaticError13);
            data::set_reward_rate_a(
                reward_a
                    .checked_add(left_over_a)
                    .unwrap_or_revert_with(Error::ArithmaticError14)
                    .checked_div(rewards_duration)
                    .unwrap_or_revert_with(Error::ArithmaticError15),
            );
            let left_over_b: U256 = remaining
                .checked_mul(data::get_reward_rate_b())
                .unwrap_or_revert_with(Error::ArithmaticError16);
            data::set_reward_rate_b(
                reward_b
                    .checked_add(left_over_b)
                    .unwrap_or_revert_with(Error::ArithmaticError17)
                    .checked_div(rewards_duration)
                    .unwrap_or_revert_with(Error::ArithmaticError18),
            );
        }

        // Ensure the provided reward amount is not more than the balance in the contract.
        // This keeps the reward rate in the right range, preventing overflows due to
        // very high values of rewardRate in the earned and rewardsPerToken functions;
        // Reward + leftover must be less than 2^256 / 10^18 to avoid overflow.
        let balance_a: U256 = runtime::call_versioned_contract(
            data::get_rewards_token_a()
                .into_hash()
                .unwrap_or_revert()
                .into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Key::from(data::get_package_hash())
            },
        );
        if !(data::get_reward_rate_a()
            <= balance_a.checked_div(rewards_duration).unwrap_or_revert_with(Error::ArithmaticError19))
        {
            runtime::revert(ApiError::from(Error::RewardATooHigh));
        }
        let balance_b: U256 = runtime::call_versioned_contract(
            data::get_rewards_token_b()
                .into_hash()
                .unwrap_or_revert()
                .into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Key::from(data::get_package_hash())
            },
        );
        if !(data::get_reward_rate_b()
            <= balance_b.checked_div(rewards_duration).unwrap_or_revert_with(Error::ArithmaticError20))
        {
            runtime::revert(ApiError::from(Error::RewardBTooHigh));
        }
        data::set_last_update_time(U256::from(blocktime));
        data::set_period_finish(
            U256::from(blocktime)
                .checked_add(rewards_duration)
                .unwrap_or_revert_with(Error::ArithmaticError21),
        );
        self.staking_dual_rewards_emit(&StakingDualRewardsEvent::RewardAdded {
            reward_a: reward_a,
            reward_b: reward_b,
            period_finish: data::get_period_finish(),
        });
    }
    fn recover_erc20(&mut self, token_address: Key, token_amount: U256) {
        OWNED::only_owner(self);
        if !(token_address != data::get_staking_token()) {
            runtime::revert(ApiError::from(Error::CannotWithdrawStakingToken));
        }
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            token_address.into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => owned::get_owner(),
                "amount" => token_amount
            },
        );
        self.staking_dual_rewards_emit(&StakingDualRewardsEvent::Recovered {
            user: token_address,
            amount: token_amount,
        });
    }
    fn update_reward(&self, account: Key) {
        data::set_reward_per_token_a_stored(self.reward_per_token_a());
        data::set_reward_per_token_b_stored(self.reward_per_token_b());
        data::set_last_update_time(self.last_time_reward_applicable());
        if account != data::ZERO_ADDRESS() {
            RewardsA::instance().set(&account, self.earned_a(account));
            UserRewardPerTokenAPaid::instance()
                .set(&account, data::get_reward_per_token_a_stored());
        }
        if account != data::ZERO_ADDRESS() {
            RewardsB::instance().set(&account, self.earned_b(account));
            UserRewardPerTokenBPaid::instance()
                .set(&account, data::get_reward_per_token_b_stored());
        }
    }
    fn staking_dual_rewards_emit(&mut self, staking_dual_rewards_event: &StakingDualRewardsEvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match staking_dual_rewards_event {
            StakingDualRewardsEvent::Staked { user, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", staking_dual_rewards_event.type_name());
                event.insert("user", user.to_string());
                event.insert("amount", amount.to_string());
                events.push(event);
            }
            StakingDualRewardsEvent::Withdraw { user, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", staking_dual_rewards_event.type_name());
                event.insert("user", user.to_string());
                event.insert("amount", amount.to_string());
                events.push(event);
            }
            StakingDualRewardsEvent::RewardPaid {
                user,
                reward_token,
                reward,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", staking_dual_rewards_event.type_name());
                event.insert("user", user.to_string());
                event.insert("reward_token", reward_token.to_string());
                event.insert("reward", reward.to_string());
                events.push(event);
            }
            StakingDualRewardsEvent::RewardAdded {
                reward_a,
                reward_b,
                period_finish,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", staking_dual_rewards_event.type_name());
                event.insert("reward", reward_a.to_string());
                event.insert("reward", reward_b.to_string());
                event.insert("period_finish", period_finish.to_string());
                events.push(event);
            }
            StakingDualRewardsEvent::Recovered { user, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", staking_dual_rewards_event.type_name());
                event.insert("user", user.to_string());
                event.insert("amount", amount.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
