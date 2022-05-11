use crate::staking_dual_rewards_instance::{STAKINGDUALREWARDSInstance};
use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};
//Const
pub const TEN_E_NINE:u128 = 1000000000;
//For Reward Token A
fn deploy_reward_token_a(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "reward_token_a",
        owner,
        runtime_args! {
            "name" => "reward_token_a",
            "symbol" => "ERA",
            "decimals" => 18 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 10000000000000) 
        },
    )
}
//For Reward Token B
fn deploy_reward_token_b(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "reward_token_b",
        owner,
        runtime_args! {
            "name" => "reward_token_b",
            "symbol" => "ERB",
            "decimals" => 18 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 10000000000000)
        },
    )
}
//For Staking Token
fn deploy_staking_token(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "staking_token",
        owner,
        runtime_args! {
            "name" => "ERC20",
            "symbol" => "St",
            "decimals" => 18 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 10000000000000)
        },
    )
}
fn deploy() -> (TestEnv, AccountHash, TestContract, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let reward_token_a = deploy_reward_token_a(&env, owner);
    let reward_token_b = deploy_reward_token_b(&env, owner);
    let staking_rewards = deploy_staking_token(&env, owner);
    let staking_dual_rewards_instance = STAKINGDUALREWARDSInstance::new(
        &env,
        "STAKINGDUALREWARDS",
        owner,
        Key::Account(owner),
        Key::Account(owner),
        Key::Hash(reward_token_a.package_hash()),
        Key::Hash(reward_token_b.package_hash()),
        Key::Hash(staking_rewards.package_hash())
    );
    // Test Contract For Returning Value
    let staking_dual_rewards_package_hash = Key::Hash(staking_dual_rewards_instance.package_hash());
    let proxy = STAKINGDUALREWARDSInstance::proxy(&env, "Proxy", owner, staking_dual_rewards_package_hash);
    // For Minting Purpose
    let to: Key = Key::from(staking_dual_rewards_package_hash);
    let amount: U256 = U256::from(TEN_E_NINE * 1000000000000);
    reward_token_a.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        0
    );
    reward_token_b.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        0
    );
    staking_rewards.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        0
    );
    staking_rewards.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => to , "amount" => amount},
        0
    );
    (env, owner, staking_dual_rewards_instance, proxy)
}

#[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}
#[test]
fn total_supply() {
    let (_, owner,instance,proxy) = deploy();
    let proxy = STAKINGDUALREWARDSInstance::contract_instance(proxy);
    let staking_dual_rewards_instance = STAKINGDUALREWARDSInstance::contract_instance(instance);
    let amount:U256 = U256::from(TEN_E_NINE * 2);
    staking_dual_rewards_instance.stake(owner,amount);
    proxy.total_supply(owner);
    let v:U256 = proxy.result();
    println!("{:?}",v);
}
#[test]
fn balance_of() {
    let (_, owner, instance,proxy) = deploy();
    let proxy = STAKINGDUALREWARDSInstance::contract_instance(proxy);
    let staking_dual_rewards_instance = STAKINGDUALREWARDSInstance::contract_instance(instance);
    let amount:U256 = U256::from(TEN_E_NINE * 2);
    staking_dual_rewards_instance.stake(owner,amount);
    proxy.balance_of(owner,Key::Account(owner));
    let v:U256 = proxy.result();
    println!("{:?}",v);
}
#[test]
fn last_time_reward_applicable() {
    let (_, owner,_,proxy) = deploy();
    let proxy = STAKINGDUALREWARDSInstance::contract_instance(proxy);
    proxy.last_time_reward_applicable(owner);
}
#[test]
fn reward_per_token_a() {
    let (_, owner,instance,proxy) = deploy();
    let proxy = STAKINGDUALREWARDSInstance::contract_instance(proxy);
    let staking_dual_rewards_instance = STAKINGDUALREWARDSInstance::contract_instance(instance);
    let amount:U256 = U256::from(TEN_E_NINE * 20);
    staking_dual_rewards_instance.stake(owner,amount);
    staking_dual_rewards_instance.notify_reward_amount(owner,U256::from(TEN_E_NINE * 20),U256::from(TEN_E_NINE * 20),100.into());
    proxy.reward_per_token_a(owner);
    let v1:U256 = proxy.result();
    println!("{:?}",v1);
}
#[test]
fn reward_per_token_b() {
    let (_, owner,instance,proxy) = deploy();
    let proxy = STAKINGDUALREWARDSInstance::contract_instance(proxy);
    let staking_dual_rewards_instance = STAKINGDUALREWARDSInstance::contract_instance(instance);
    let amount:U256 = U256::from(TEN_E_NINE * 20);
    staking_dual_rewards_instance.stake(owner,amount);
    staking_dual_rewards_instance.notify_reward_amount(owner,U256::from(TEN_E_NINE * 20),U256::from(TEN_E_NINE * 20),100.into());
    proxy.reward_per_token_b(owner);
    let v1:U256 = proxy.result();
    println!("{:?}",v1);
}
#[test]
fn earned_a() {
    let (_, owner,instance,proxy) = deploy();
    let staking_dual_rewards_instance = STAKINGDUALREWARDSInstance::contract_instance(instance);
    let proxy = STAKINGDUALREWARDSInstance::contract_instance(proxy);
    let amount:U256 = U256::from(TEN_E_NINE * 1000000000000);
    staking_dual_rewards_instance.stake(owner,amount);
    staking_dual_rewards_instance.notify_reward_amount(owner,U256::from(TEN_E_NINE * 1000000000000),U256::from(TEN_E_NINE * 1000000000000),200.into());
    proxy.earned_a(owner,Key::Account(owner));
    let v:U256 = proxy.result();
    println!("{:?}",v);
}
#[test]
fn earned_b() {
    let (_, owner,instance,proxy) = deploy();
    let staking_dual_rewards_instance = STAKINGDUALREWARDSInstance::contract_instance(instance);
    let proxy = STAKINGDUALREWARDSInstance::contract_instance(proxy);
    let amount:U256 = U256::from(TEN_E_NINE * 1000000000000);
    staking_dual_rewards_instance.stake(owner,amount);
    staking_dual_rewards_instance.notify_reward_amount(owner,U256::from(TEN_E_NINE * 1000000000000),U256::from(TEN_E_NINE * 1000000000000),200.into());
    proxy.earned_b(owner,Key::Account(owner));
    let v:U256 = proxy.result();
    println!("{:?}",v);
}
#[test]
fn stake() {
    let (_, owner,instance,_) = deploy();
    let staking_dual_rewards_instance = STAKINGDUALREWARDSInstance::contract_instance(instance);
    let amount:U256 = U256::from(TEN_E_NINE * 20);
    staking_dual_rewards_instance.stake(owner,amount);
}
#[test]
fn withdraw() {
    let (_, owner,instance,_) = deploy();
    let staking_dual_rewards_instance = STAKINGDUALREWARDSInstance::contract_instance(instance);
    let amount:U256 =U256::from(TEN_E_NINE * 20);
    staking_dual_rewards_instance.stake(owner,amount);
    let withdraw_amount:U256=U256::from(TEN_E_NINE * 10);
    staking_dual_rewards_instance.withdraw(owner,withdraw_amount);
}
#[test]
fn get_reward() {
    let (_, owner,instance,_) = deploy();
    let staking_dual_rewards_instance = STAKINGDUALREWARDSInstance::contract_instance(instance);
    staking_dual_rewards_instance.get_reward(owner);
}
#[test]
fn exit() {
    let (_, owner,instance,_) = deploy();
    let staking_dual_rewards_instance = STAKINGDUALREWARDSInstance::contract_instance(instance);
    let amount:U256 = U256::from(TEN_E_NINE * 20);
    staking_dual_rewards_instance.stake(owner,amount);
    staking_dual_rewards_instance.exit(owner);
}
#[test]
fn notify_reward_amount() {
    let (_, owner,instance,_) = deploy();
    let staking_dual_rewards_instance = STAKINGDUALREWARDSInstance::contract_instance(instance);
    let amount:U256 = U256::from(TEN_E_NINE * 1000000000000);
    staking_dual_rewards_instance.stake(owner,amount);
    staking_dual_rewards_instance.notify_reward_amount(owner,U256::from(TEN_E_NINE * 1000000000000),U256::from(TEN_E_NINE * 1000000000000),2.into());
}
#[test]
fn recover_erc20() {
    let (env, owner,instance,_) = deploy();
    let staking_dual_rewards_package_hash: ContractPackageHash = instance.query_named_key("self_package_hash".to_string());
    let to: Key = Key::from(staking_dual_rewards_package_hash);
    let staking_dual_rewards_instance = STAKINGDUALREWARDSInstance::contract_instance(instance);
    let token_address = deploy_reward_token_a(&env, owner);
    let amount:U256 = U256::from(TEN_E_NINE * 20);
    token_address.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        0
    );
    staking_dual_rewards_instance.stake(owner,amount);
    staking_dual_rewards_instance.recover_erc20(owner,Key::Hash(token_address.package_hash()),amount);
}