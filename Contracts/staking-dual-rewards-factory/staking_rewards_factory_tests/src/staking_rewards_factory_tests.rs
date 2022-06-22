use crate::staking_rewards_factory_instance::STAKINGREWARDSFACTORYInstance;
use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, U256
};
use test_env::{TestContract, TestEnv};
//Const
pub const TEN_E_NINE:u128 = 1000000000;

//For Reward Token A
fn deploy_reward_token_a(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "Reward Token A",
        owner,
        runtime_args! {
            "name" => "Token A",
            "symbol" => "ERC20",
            "decimals" => 18 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 100)
        },
    )
}
// For Reward Token B
fn deploy_reward_token_b(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "Reward Token B",
        owner,
        runtime_args! {
            "name" => "Token B",
            "symbol" => "ER",
            "decimals" => 18 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 100)
        },
    )
}
// For Staking Token
fn deploy_staking_token(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "Staking Reward",
        owner,
        runtime_args! {
            "name" => "Staking Reward",
            "symbol" => "ST",
            "decimals" => 9 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 100)
        },
    )
}
fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let staking_reward_factory_instance = STAKINGREWARDSFACTORYInstance::new(
        &env,
        "STAKINGREWARDSFACTORY",
        owner,
        100.into(),
    );
    (env, owner, staking_reward_factory_instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn test_deploy_1() {
    let (env, owner, staking_reward_factory) = deploy();
    
    let staking_reward_factory_instance = STAKINGREWARDSFACTORYInstance::contract_instance(staking_reward_factory.clone());
    let staking_token = deploy_staking_token(&env, owner);
    let reward_token_a = deploy_reward_token_a(&env, owner);
    let reward_token_b = deploy_reward_token_b(&env, owner);
    let staking_token_key = Key::Hash(staking_token.package_hash());
    let reward_token_a_key = Key::Hash(reward_token_a.package_hash());
    let reward_token_b_key = Key::Hash(reward_token_b.package_hash());
    staking_reward_factory_instance.deploy(owner, Key::Account(owner), staking_token_key, reward_token_a_key,reward_token_b_key,U256::from(TEN_E_NINE * 10),U256::from(TEN_E_NINE * 10),U256::from(TEN_E_NINE * 2));
    //let result:Bytes = staking_reward_factory.query_dictionary("staking_rewards_info", staking_token_key.to_string()).unwrap_or_default();
    // let result1:String = staking_token_key.to_string();
    // let v:Bytes = staking_reward_factory_instance.result();
    // let info: StakingRewardsInfo =
    //     StakingRewardsInfo::from_bytes(&v)
    //                 .unwrap()
    //                 .0;
    //println!("{:?}",result1);
    //println!("{:?}",info);
}
#[test]
fn test_update() {
    let (env, owner, staking_reward_factory_instance) = deploy();
    let staking_reward_factory_instance = STAKINGREWARDSFACTORYInstance::contract_instance(staking_reward_factory_instance);
    let staking_token = deploy_staking_token(&env, owner);
    let reward_token_a = deploy_reward_token_a(&env, owner);
    let reward_token_b = deploy_reward_token_b(&env, owner);
    let staking_token = Key::Hash(staking_token.package_hash());
    let reward_token_a = Key::Hash(reward_token_a.package_hash());
    let reward_token_b = Key::Hash(reward_token_b.package_hash());
    staking_reward_factory_instance.deploy(owner, Key::Account(owner), staking_token, reward_token_a,reward_token_b,U256::from(TEN_E_NINE * 10),U256::from(TEN_E_NINE * 10),U256::from(TEN_E_NINE * 2));
    staking_reward_factory_instance.update(owner, staking_token, U256::from(TEN_E_NINE * 10),U256::from(TEN_E_NINE * 10),U256::from(TEN_E_NINE * 2));
}
#[test]
fn test_notify_reward_amounts() {
    let (env, owner, staking_reward_factory_instance) = deploy();
    let staking_reward_factory_package_hash: ContractPackageHash = staking_reward_factory_instance.query_named_key("self_package_hash".to_string());
    let staking_reward_factory_instance = STAKINGREWARDSFACTORYInstance::contract_instance(staking_reward_factory_instance);
    let staking_token = deploy_staking_token(&env, owner);
    let reward_token_a = deploy_reward_token_a(&env, owner);
    let reward_token_b = deploy_reward_token_b(&env, owner);
    let to: Key = Key::from(staking_reward_factory_package_hash);
    let amount: U256 = U256::from(TEN_E_NINE * 10);
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
    let staking_token_key = Key::Hash(staking_token.package_hash());
    let reward_token_a_key = Key::Hash(reward_token_a.package_hash());
    let reward_token_b_key = Key::Hash(reward_token_b.package_hash());

    staking_reward_factory_instance.deploy(owner, Key::Account(owner), staking_token_key, reward_token_a_key,reward_token_b_key,U256::from(TEN_E_NINE * 10),U256::from(TEN_E_NINE * 10),U256::from(TEN_E_NINE * 2));
    staking_reward_factory_instance.notify_reward_amounts(owner);
}
#[test]
fn test_notify_reward_amount() {
    let (env, owner, staking_reward_factory_instance) = deploy();
    let staking_reward_factory_package_hash: ContractPackageHash = staking_reward_factory_instance.query_named_key("self_package_hash".to_string());
    let staking_reward_factory_instance = STAKINGREWARDSFACTORYInstance::contract_instance(staking_reward_factory_instance);
    let staking_token = deploy_staking_token(&env, owner);
    let reward_token_a = deploy_reward_token_a(&env, owner);
    let reward_token_b = deploy_reward_token_b(&env, owner);
    let to: Key = Key::from(staking_reward_factory_package_hash);
    let amount: U256 = U256::from(TEN_E_NINE * 10);
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
    let staking_token = Key::Hash(staking_token.package_hash());
    let reward_token_a = Key::Hash(reward_token_a.package_hash());
    let reward_token_b = Key::Hash(reward_token_b.package_hash());

    staking_reward_factory_instance.deploy(owner, Key::Account(owner), staking_token, reward_token_a,reward_token_b,U256::from(TEN_E_NINE * 10),U256::from(TEN_E_NINE * 10),U256::from(TEN_E_NINE * 2));
    staking_reward_factory_instance.notify_reward_amount(owner, staking_token);
}
#[test]
fn test_pull_extra_tokens() {
    let (env, owner, staking_reward_factory_instance) = deploy();
    let staking_reward_factory_package_hash: ContractPackageHash = staking_reward_factory_instance.query_named_key("self_package_hash".to_string());
    let staking_reward_factory_instance = STAKINGREWARDSFACTORYInstance::contract_instance(staking_reward_factory_instance);
    let staking_token = deploy_staking_token(&env, owner);
    let to: Key = Key::from(staking_reward_factory_package_hash);
    let amount: U256 = U256::from(TEN_E_NINE * 10);
    staking_token.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        0
    );
    let staking_token = Key::Hash(staking_token.package_hash());

    staking_reward_factory_instance.pull_extra_tokens(owner, staking_token, U256::from(TEN_E_NINE * 10));
}

