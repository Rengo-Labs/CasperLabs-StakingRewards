use std::collections::BTreeMap;

use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct STAKINGREWARDSFACTORYInstance(TestContract);

impl STAKINGREWARDSFACTORYInstance {
    pub fn contract_instance(contract: TestContract) -> STAKINGREWARDSFACTORYInstance {
        STAKINGREWARDSFACTORYInstance(contract)
    }
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        rewards_token: Key,
        staking_rewards_genesis: U256,
    ) -> TestContract {
        TestContract::new(
            env,
            "staking_rewards_factory.wasm",
            contract_name,
            sender,
            runtime_args! {
                "rewards_token" => rewards_token,
                "staking_rewards_genesis" => staking_rewards_genesis,
            },
        )
    }
    pub fn deploy(
        &self,
        sender: AccountHash,
        staking_token: Key,
        reward_amount: U256,
        rewards_duration: U256,
    ) {
        self.0.call_contract(
            sender,
            "deploy",
            runtime_args! {
                "staking_token" => staking_token,
                "reward_amount" => reward_amount,
                "rewards_duration" => rewards_duration,
            },
            0,
        );
    }
    pub fn update(
        &self,
        sender: AccountHash,
        staking_token: Key,
        reward_amount: U256,
        rewards_duration: U256,
    ) {
        self.0.call_contract(
            sender,
            "update",
            runtime_args! {
                "staking_token" => staking_token,
                "reward_amount" => reward_amount,
                "rewards_duration" => rewards_duration,
            },
            0,
        );
    }
    pub fn notify_reward_amounts(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "notify_reward_amounts", runtime_args! {}, 400);
    }
    pub fn notify_reward_amount(&self, sender: AccountHash, staking_token: Key) {
        self.0.call_contract(
            sender,
            "notify_reward_amount",
            runtime_args! {
                "staking_token" => staking_token
            },
            400,
        );
    }
    pub fn pull_extra_tokens(&self, sender: AccountHash, token: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "pull_extra_tokens",
            runtime_args! {
                "token" => token,
                "amount" => amount
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
