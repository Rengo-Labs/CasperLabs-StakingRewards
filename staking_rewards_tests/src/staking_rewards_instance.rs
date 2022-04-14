use std::collections::BTreeMap;

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, CLTyped, ContractPackageHash, Key, RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct STAKINGREWARDSInstance(TestContract);

impl STAKINGREWARDSInstance {
    pub fn contract_instance(contract: TestContract) -> STAKINGREWARDSInstance {
        STAKINGREWARDSInstance(contract)
    }
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        rewards_distribution: Key,
        rewards_token: Key,
        staking_token: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "staking_rewards.wasm",
            contract_name,
            sender,
            runtime_args! {
                "rewards_distribution" => rewards_distribution,
                "rewards_token" => rewards_token,
                "staking_token" => staking_token,
            },
        )
    }
    pub fn proxy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        staking_rewards: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "staking_rewards_test.wasm",
            contract_name,
            sender,
            runtime_args! {
                "staking_rewards" => staking_rewards
            },
        )
    }
    pub fn total_supply(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "total_supply", runtime_args! {},0);
    }
    pub fn balance_of(&self, sender: AccountHash, account: Key) {
        self.0.call_contract(
            sender,
            "balance_of",
            runtime_args! {
                "account" => account
            },
            0
        );
    }
    pub fn last_time_reward_applicable(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "last_time_reward_applicable", runtime_args! {},200);
    }
    pub fn reward_per_token(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "reward_per_token", runtime_args! {},200);
    }
    pub fn earned(&self, sender: AccountHash, account: Key) {
        self.0.call_contract(
            sender,
            "earned",
            runtime_args! {
                "account" => account
            },
            100
        );
    }
    pub fn stake_with_permit(
        &self,
        sender: AccountHash,
        amount: U256,
        deadline: U256,
        public_key: String,
        signature: String,
    ) {
        self.0.call_contract(
            sender,
            "stake_with_permit",
            runtime_args! {
                "amount" => amount,
                "deadline" => deadline,
                "public_key" => public_key,
                "signature" => signature
            },
            0
        );
    }
    pub fn stake(&self, sender: AccountHash, amount: U256) {
        self.0.call_contract(
            sender,
            "stake",
            runtime_args! {
                "amount" => amount
            },
            0
        );
    }
    pub fn withdraw(&self, sender: AccountHash, amount: U256) {
        self.0.call_contract(
            sender,
            "withdraw",
            runtime_args! {
                "amount" => amount
            },
            0
        );
    }
    pub fn get_reward(&self, sender: AccountHash) {
        self.0.call_contract(sender, "get_reward", runtime_args! {},0);
    }
    pub fn exit(&self, sender: AccountHash) {
        self.0.call_contract(sender, "exit", runtime_args! {},0);
    }
    pub fn notify_reward_amount(&self, sender: AccountHash, reward: U256, rewards_duration: U256) {
        self.0.call_contract(
            sender,
            "notify_reward_amount",
            runtime_args! {
                "reward" => reward,
                "rewards_duration" => rewards_duration,
            },
            50
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
