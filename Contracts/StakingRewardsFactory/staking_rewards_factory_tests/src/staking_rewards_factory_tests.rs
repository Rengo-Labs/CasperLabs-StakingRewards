use crate::staking_rewards_factory_instance::STAKINGREWARDSFACTORYInstance;
use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};
pub const TEN_E_NINE:u128 = 1000000000;
fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "erc2020",
        owner,
        runtime_args! {
            "name" => "ERC",
            "symbol" => "ERC20",
            "decimals" => 18 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 1000)
        },
    )
}
fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);
    let instance = STAKINGREWARDSFACTORYInstance::new(
        &env,
        "STAKINGREWARDSFACTORY",
        owner,
        Key::Hash(erc20.package_hash()),
        100.into(),
    );
    // For Minting Purpose
    let key: ContractPackageHash = instance.query_named_key("self_package_hash".to_string());
    let to: Key = Key::from(key);
    let amount: U256 = U256::from(TEN_E_NINE * 100);
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        0,
    );
    (env, owner, instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn test_deploy_1() {
    let (env, owner, instance) = deploy();
    let instance = STAKINGREWARDSFACTORYInstance::contract_instance(instance);
    let erc20 = deploy_erc20(&env, owner);
    let staking_token = Key::Hash(erc20.package_hash());

    instance.deploy(owner, staking_token, 30.into(), 100.into());
}
#[test]
fn test_update() {
    let (env, owner, instance) = deploy();
    let instance = STAKINGREWARDSFACTORYInstance::contract_instance(instance);
    let erc20 = deploy_erc20(&env, owner);
    let staking_token = Key::Hash(erc20.package_hash());
    instance.deploy(owner, staking_token, U256::from(TEN_E_NINE * 10), 100.into());
    instance.update(owner, staking_token, U256::from(TEN_E_NINE * 10), 100.into());
}
#[test]
fn test_notify_reward_amounts() {
    let (env, owner, instance) = deploy();
    let instance = STAKINGREWARDSFACTORYInstance::contract_instance(instance);
    let erc20 = deploy_erc20(&env, owner);
    let staking_token = Key::Hash(erc20.package_hash());
    instance.deploy(owner, staking_token, U256::from(TEN_E_NINE * 10), 100.into());
    instance.notify_reward_amounts(owner);
}
#[test]
fn test_notify_reward_amount() {
    let (env, owner, instance) = deploy();
    let instance = STAKINGREWARDSFACTORYInstance::contract_instance(instance);
    let erc20 = deploy_erc20(&env, owner);
    let staking_token = Key::Hash(erc20.package_hash());
    instance.deploy(owner, staking_token, U256::from(TEN_E_NINE * 10), 100.into());
    instance.notify_reward_amount(owner, staking_token);
}
#[test]
fn test_pull_extra_tokens() {
    let (env, owner, instance) = deploy();
    let instance = STAKINGREWARDSFACTORYInstance::contract_instance(instance);
    let erc20 = deploy_erc20(&env, owner);
    let staking_token = Key::Hash(erc20.package_hash());
    instance.pull_extra_tokens(owner, staking_token, U256::from(TEN_E_NINE * 100));
}

