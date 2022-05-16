use std::collections::BTreeMap;

use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct STAKINGDUALREWARDSInstance(TestContract);

impl STAKINGDUALREWARDSInstance {
    pub fn contract_instance(contract: TestContract) -> STAKINGDUALREWARDSInstance {
        STAKINGDUALREWARDSInstance(contract)
    }
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        owner: Key,
        dual_rewards_distribution: Key,
        rewards_token_a: Key,
        rewards_token_b: Key,
        staking_token: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "staking_dual_rewards.wasm",
            contract_name,
            sender,
            runtime_args! {
                "owner" => owner,
                "dual_rewards_distribution" => dual_rewards_distribution,
                "rewards_token_a" => rewards_token_a,
                "rewards_token_b" => rewards_token_b,
                "staking_token" => staking_token,
            },
        )
    }
    pub fn proxy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        staking_dual_rewards: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "staking_dual_rewards_test.wasm",
            contract_name,
            sender,
            runtime_args! {
                "staking_dual_rewards" => staking_dual_rewards
            },
        )
    }
    pub fn total_supply(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "total_supply", runtime_args! {}, 0);
    }
    pub fn balance_of(&self, sender: AccountHash, account: Key) {
        self.0.call_contract(
            sender,
            "balance_of",
            runtime_args! {
                "account" => account
            },
            0,
        );
    }
    pub fn last_time_reward_applicable(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "last_time_reward_applicable", runtime_args! {}, 200);
    }
    pub fn reward_per_token_a(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "reward_per_token_a", runtime_args! {}, 600);
    }
    pub fn reward_per_token_b(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "reward_per_token_b", runtime_args! {}, 600);
    }
    pub fn earned_a(&self, sender: AccountHash, account: Key) {
        self.0.call_contract(
            sender,
            "earned_a",
            runtime_args! {
                "account" => account
            },
            500,
        );
    }
    pub fn earned_b(&self, sender: AccountHash, account: Key) {
        self.0.call_contract(
            sender,
            "earned_b",
            runtime_args! {
                "account" => account
            },
            500,
        );
    }
    pub fn stake(&self, sender: AccountHash, amount: U256) {
        self.0.call_contract(
            sender,
            "stake",
            runtime_args! {
                "amount" => amount
            },
            0,
        );
    }
    pub fn withdraw(&self, sender: AccountHash, amount: U256) {
        self.0.call_contract(
            sender,
            "withdraw",
            runtime_args! {
                "amount" => amount
            },
            0,
        );
    }
    pub fn get_reward(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "get_reward", runtime_args! {}, 0);
    }
    pub fn exit(&self, sender: AccountHash) {
        self.0.call_contract(sender, "exit", runtime_args! {}, 0);
    }
    pub fn notify_reward_amount(
        &self,
        sender: AccountHash,
        reward_a: U256,
        reward_b: U256,
        rewards_duration: U256,
    ) {
        self.0.call_contract(
            sender,
            "notify_reward_amount",
            runtime_args! {
                "reward_a" => reward_a,
                "reward_b" => reward_b,
                "rewards_duration" => rewards_duration,
            },
            50,
        );
    }
    pub fn recover_erc20(&self, sender: AccountHash, token_address: Key, token_amount: U256) {
        self.0.call_contract(
            sender,
            "recover_erc20",
            runtime_args! {
                "token_address" => token_address,
                "token_amount" => token_amount,
            },
            0,
        );
    }
    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }
}
