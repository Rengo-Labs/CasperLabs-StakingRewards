use crate::staking_rewards_instance::STAKINGREWARDSInstance;
use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};
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
            "initial_supply" => U256::from(1000000000000000 as u128)
        },
    )
}
fn deploy() -> (TestEnv, AccountHash, TestContract, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);
    let instance = STAKINGREWARDSInstance::new(
        &env,
        "STAKINGREWARDS",
        owner,
        Key::Account(owner),
        Key::Hash(erc20.package_hash()),
        Key::Hash(erc20.package_hash()),
    );
    // Test Contract For Returning Value
    let staking_rewards_package_hash = Key::Hash(instance.package_hash());
    let proxy = STAKINGREWARDSInstance::proxy(&env, "Proxy", owner, staking_rewards_package_hash);
    // For Minting Purpose
    let key: ContractPackageHash = instance.query_named_key("self_package_hash".to_string());
    let to: Key = Key::from(key);
    let amount: U256 = 4000.into();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        0
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => to , "amount" => amount},
        0
    );
    (env, owner, instance, proxy)
}

#[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}
#[test]
fn total_supply() {
    let (_, owner,instance,proxy) = deploy();
    let proxy = STAKINGREWARDSInstance::contract_instance(proxy);
    let instance = STAKINGREWARDSInstance::contract_instance(instance);
    let amount:U256 = 2000.into();
    instance.stake(owner,amount);
    proxy.total_supply(owner);
    let v:U256 = proxy.result();
    println!("{:?}",v);
}
#[test]
fn balance_of() {
    let (_, owner, instance,proxy) = deploy();
    let proxy = STAKINGREWARDSInstance::contract_instance(proxy);
    let instance = STAKINGREWARDSInstance::contract_instance(instance);
    let amount:U256 = 3000.into();
    instance.stake(owner,amount);
    proxy.balance_of(owner,Key::Account(owner));
    let v:U256 = proxy.result();
    println!("{:?}",v);
}
#[test]
fn last_time_reward_applicable() {
    let (_, owner,_,proxy) = deploy();
    let proxy = STAKINGREWARDSInstance::contract_instance(proxy);
    proxy.last_time_reward_applicable(owner);
}
#[test]
fn reward_per_token() {
    let (_, owner,instance,proxy) = deploy();
    let proxy = STAKINGREWARDSInstance::contract_instance(proxy);
    let instance = STAKINGREWARDSInstance::contract_instance(instance);
    let amount:U256 = 2000.into();
    instance.stake(owner,amount);
    instance.notify_reward_amount(owner,30.into(),10.into());
    proxy.reward_per_token(owner);
}
#[test]
fn earned() {
    let (_, owner,instance,proxy) = deploy();
    let instance = STAKINGREWARDSInstance::contract_instance(instance);
    let proxy = STAKINGREWARDSInstance::contract_instance(proxy);
    let amount:U256 = 2000.into();
    instance.stake(owner,amount);
    instance.notify_reward_amount(owner,30.into(),10.into());
    proxy.earned(owner,Key::Account(owner));
}
#[test]
fn stake_with_permit() {
    let (_, owner,instance,_) = deploy();
    let instance = STAKINGREWARDSInstance::contract_instance(instance);
    let amount:U256 = 2000.into();
    instance.stake_with_permit(owner,amount,10.into(),"".to_string(),"".to_string()); 
}
#[test]
fn stake() {
    let (_, owner,instance,_) = deploy();
    let instance = STAKINGREWARDSInstance::contract_instance(instance);
    let amount:U256 = 2000.into();
    instance.stake(owner,amount);
}
#[test]
fn withdraw() {
    let (_, owner,instance,_) = deploy();
    let instance = STAKINGREWARDSInstance::contract_instance(instance);
    let amount:U256 = 2000.into();
    instance.stake(owner,amount);
    let withdraw_amount:U256=20.into();
    instance.withdraw(owner,withdraw_amount);
}
#[test]
fn get_reward() {
    let (_, owner,instance,_) = deploy();
    let instance = STAKINGREWARDSInstance::contract_instance(instance);
    instance.get_reward(owner);
}
#[test]
fn exit() {
    let (_, owner,instance,_) = deploy();
    let instance = STAKINGREWARDSInstance::contract_instance(instance);
    let amount:U256 = 2000.into();
    instance.stake(owner,amount);
    instance.exit(owner);
}
#[test]
fn notify_reward_amount() {
    let (_, owner,instance,_) = deploy();
    let instance = STAKINGREWARDSInstance::contract_instance(instance);
    let amount:U256 = 2000.into();
    instance.stake(owner,amount);
    instance.notify_reward_amount(owner,30.into(),10.into());
}