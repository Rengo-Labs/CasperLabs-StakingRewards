#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, string::String, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{set_key, ContractContext, OnChainContractStorage};
use ownable_crate::OWNABLE;
use reentrancy_guard_crate::REENTRANCYGUARD;
use rewards_distribution_recipient_crate::REWARDSDISTRIBUTIONRECIPIENT;
use staking_rewards_crate::STAKINGREWARDS;
use staking_rewards_factory_crate::STAKINGREWARDSFACTORY;
#[derive(Default)]
struct StakingRewardsFactory(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for StakingRewardsFactory {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl REENTRANCYGUARD<OnChainContractStorage> for StakingRewardsFactory {}
impl REWARDSDISTRIBUTIONRECIPIENT<OnChainContractStorage> for StakingRewardsFactory {}
impl STAKINGREWARDS<OnChainContractStorage> for StakingRewardsFactory {}
impl STAKINGREWARDSFACTORY<OnChainContractStorage> for StakingRewardsFactory {}
impl StakingRewardsFactory {
    fn constructor(
        &mut self,
        rewards_token: Key,
        staking_rewards_genesis: U256,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        STAKINGREWARDSFACTORY::init(
            self,
            rewards_token,
            staking_rewards_genesis,
            Key::from(contract_hash),
            package_hash,
        )
    }
}
impl OWNABLE<OnChainContractStorage> for StakingRewardsFactory {}
#[no_mangle]
fn constructor() {
    let rewards_token: Key = runtime::get_named_arg("rewards_token");
    let staking_rewards_genesis: U256 = runtime::get_named_arg("staking_rewards_genesis");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    StakingRewardsFactory::default().constructor(
        rewards_token,
        staking_rewards_genesis,
        contract_hash,
        package_hash,
    );
}
#[no_mangle]
fn balance_of() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = STAKINGREWARDS::balance_of(&StakingRewardsFactory::default(), account);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn constructor_sr() {
    let rewards_distribution: Key = runtime::get_named_arg("rewards_distribution");
    let rewards_token: Key = runtime::get_named_arg("rewards_token");
    let staking_token: Key = runtime::get_named_arg("staking_token");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    STAKINGREWARDS::init(
        &StakingRewardsFactory::default(),
        rewards_distribution,
        rewards_token,
        staking_token,
        Key::from(contract_hash),
        package_hash,
    );
}
#[no_mangle]
fn total_supply() {
    let ret: U256 = STAKINGREWARDS::total_supply(&StakingRewardsFactory::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn total_supply_js_client() {
    let ret: U256 = STAKINGREWARDS::total_supply(&StakingRewardsFactory::default());
    set_key("total_supply", ret);
}
// #[no_mangle]
// fn balance_of() {
//     let account: Key = runtime::get_named_arg("account");
//     let ret: U256 = StakingRewards::default().balance_of(account);
//     runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
// }
#[no_mangle]
fn last_time_reward_applicable() {
    let ret: U256 = STAKINGREWARDS::last_time_reward_applicable(&StakingRewardsFactory::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn reward_per_token() {
    let ret: U256 = STAKINGREWARDS::reward_per_token(&StakingRewardsFactory::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn earned() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = STAKINGREWARDS::earned(&StakingRewardsFactory::default(), account);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn stake_with_permit() {
    let amount: U256 = runtime::get_named_arg("amount");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let public_key: String = runtime::get_named_arg("public_key");
    let signature: String = runtime::get_named_arg("signature");
    STAKINGREWARDS::stake_with_permit(
        &mut StakingRewardsFactory::default(),
        amount,
        deadline,
        public_key,
        signature,
    );
}
#[no_mangle]
fn stake() {
    let amount: U256 = runtime::get_named_arg("amount");
    STAKINGREWARDS::stake(&mut StakingRewardsFactory::default(), amount);
}
#[no_mangle]
fn withdraw() {
    let amount: U256 = runtime::get_named_arg("amount");
    STAKINGREWARDS::withdraw(&mut StakingRewardsFactory::default(), amount);
}
#[no_mangle]
fn get_reward() {
    STAKINGREWARDS::get_reward(&mut StakingRewardsFactory::default());
}
#[no_mangle]
fn exit() {
    STAKINGREWARDS::exit(&mut StakingRewardsFactory::default());
}
#[no_mangle]
fn notify_reward_amount_sr() {
    let reward: U256 = runtime::get_named_arg("reward");
    let rewards_duration: U256 = runtime::get_named_arg("rewards_duration");
    STAKINGREWARDS::notify_reward_amount(
        &mut StakingRewardsFactory::default(),
        reward,
        rewards_duration,
    );
}

#[no_mangle]
fn owner() {
    let ret: Key = OWNABLE::owner(&StakingRewardsFactory::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn is_owner() {
    let ret: bool = OWNABLE::is_owner(&StakingRewardsFactory::default());
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn renounce_ownership() {
    OWNABLE::renounce_ownership(&mut StakingRewardsFactory::default());
}
#[no_mangle]
fn transfer_ownership() {
    let new_owner: Key = runtime::get_named_arg("new_owner");
    OWNABLE::transfer_ownership(&mut StakingRewardsFactory::default(), new_owner);
}
#[no_mangle]
fn deploy() {
    let staking_token: Key = runtime::get_named_arg("staking_token");
    let reward_amount: U256 = runtime::get_named_arg("reward_amount");
    let rewards_duration: U256 = runtime::get_named_arg("rewards_duration");
    StakingRewardsFactory::default().deploy(staking_token, reward_amount, rewards_duration)
}
#[no_mangle]
fn update() {
    let staking_token: Key = runtime::get_named_arg("staking_token");
    let reward_amount: U256 = runtime::get_named_arg("reward_amount");
    let rewards_duration: U256 = runtime::get_named_arg("rewards_duration");

    StakingRewardsFactory::default().update(staking_token, reward_amount, rewards_duration)
}
#[no_mangle]
fn notify_reward_amounts() {
    StakingRewardsFactory::default().notify_reward_amounts()
}
#[no_mangle]
fn notify_reward_amount() {
    let staking_token: Key = runtime::get_named_arg("staking_token");
    StakingRewardsFactory::default().notify_reward_amount(staking_token)
}
#[no_mangle]
fn pull_extra_tokens() {
    let token: Key = runtime::get_named_arg("token");
    let amount: U256 = runtime::get_named_arg("amount");
    StakingRewardsFactory::default().pull_extra_tokens(token, amount)
}
//Entry Points
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("rewards_token", Key::cl_type()),
            Parameter::new("staking_rewards_genesis", U256::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "constructor_sr",
        vec![
            Parameter::new("rewards_distribution", Key::cl_type()),
            Parameter::new("rewards_token", Key::cl_type()),
            Parameter::new("staking_token", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply_js_client",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("account", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "last_time_reward_applicable",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_per_token",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "earned",
        vec![Parameter::new("account", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "stake_with_permit",
        vec![
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("deadline", U256::cl_type()),
            Parameter::new("public_key", String::cl_type()),
            Parameter::new("signature", String::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "stake",
        vec![Parameter::new("amount", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![Parameter::new("amount", U256::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_reward",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "exit",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "notify_reward_amount_sr",
        vec![
            Parameter::new("reward", U256::cl_type()),
            Parameter::new("rewards_duration", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "owner",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "is_owner",
        vec![],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "renounce_ownership",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_ownership",
        vec![Parameter::new("new_owner", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "deploy",
        vec![
            Parameter::new("staking_token", Key::cl_type()),
            Parameter::new("reward_amount", U256::cl_type()),
            Parameter::new("rewards_duration", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "update",
        vec![
            Parameter::new("staking_token", Key::cl_type()),
            Parameter::new("reward_amount", U256::cl_type()),
            Parameter::new("rewards_duration", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "notify_reward_amounts",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "notify_reward_amount",
        vec![Parameter::new("staking_token", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "pull_extra_tokens",
        vec![
            Parameter::new("token", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Contract name must be same for all new versions of the contracts
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());
        let rewards_token: Key = runtime::get_named_arg("rewards_token");
        let staking_rewards_genesis: U256 = runtime::get_named_arg("staking_rewards_genesis");
        // Prepare constructor args
        let constructor_args = runtime_args! {
            "rewards_token" => rewards_token,
            "staking_rewards_genesis" => staking_rewards_genesis,
            "contract_hash" => contract_hash,
            "package_hash"=> package_hash
        };

        // Add the constructor group to the package hash with a single URef.
        let constructor_access: URef =
            storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
                .unwrap_or_revert()
                .pop()
                .unwrap_or_revert();

        // Call the constructor entry point
        let _: () =
            runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

        // Remove all URefs from the constructor group, so no one can call it for the second time.
        let mut urefs = BTreeSet::new();
        urefs.insert(constructor_access);
        storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
            .unwrap_or_revert();

        // Store contract in the account's named keys.
        runtime::put_key(
            &format!("{}_package_hash", contract_name),
            package_hash.into(),
        );
        runtime::put_key(
            &format!("{}_package_hash_wrapped", contract_name),
            storage::new_uref(package_hash).into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
        runtime::put_key(
            &format!("{}_package_access_token", contract_name),
            access_token.into(),
        );
    } else {
        // this is a contract upgrade
        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();

        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}
