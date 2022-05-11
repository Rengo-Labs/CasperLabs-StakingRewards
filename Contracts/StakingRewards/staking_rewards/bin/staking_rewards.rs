#![no_main]
#![no_std]

extern crate alloc;
use alloc::{ collections::BTreeSet, format, string::String, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args,CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage, set_key};
use reentrancy_guard_crate::REENTRANCYGUARD;
use rewards_distribution_recipient_crate::REWARDSDISTRIBUTIONRECIPIENT;
use staking_rewards_crate::STAKINGREWARDS;
#[derive(Default)]
struct StakingRewards(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for StakingRewards {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl REENTRANCYGUARD<OnChainContractStorage> for StakingRewards {}
impl REWARDSDISTRIBUTIONRECIPIENT<OnChainContractStorage> for StakingRewards {}
impl STAKINGREWARDS<OnChainContractStorage> for StakingRewards {}

impl StakingRewards {
    fn constructor(
        &self,
        rewards_distribution: Key,
        rewards_token: Key,
        staking_token: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        STAKINGREWARDS::init(
            self,
            rewards_distribution,
            rewards_token,
            staking_token,
            Key::from(contract_hash),
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let rewards_distribution: Key = runtime::get_named_arg("rewards_distribution");
    let rewards_token: Key = runtime::get_named_arg("rewards_token");
    let staking_token: Key = runtime::get_named_arg("staking_token");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    StakingRewards::default().constructor(
        rewards_distribution,
        rewards_token,
        staking_token,
        contract_hash,
        package_hash,
    );
}
#[no_mangle]
fn total_supply() {
    let ret: U256 = StakingRewards::default().total_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn total_supply_js_client() {
    let ret: U256 = StakingRewards::default().total_supply();
    set_key("total_supply", ret);
}
#[no_mangle]
fn balance_of() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = StakingRewards::default().balance_of(account);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn last_time_reward_applicable() {
    let ret: U256 = StakingRewards::default().last_time_reward_applicable();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn reward_per_token() {
    let ret: U256 = StakingRewards::default().reward_per_token();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn earned() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = StakingRewards::default().earned(account);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn stake_with_permit() {
    let amount: U256 = runtime::get_named_arg("amount");
    let deadline: U256 = runtime::get_named_arg("deadline");
    let public_key: String = runtime::get_named_arg("public_key");
    let signature: String = runtime::get_named_arg("signature");
    StakingRewards::default().stake_with_permit(amount, deadline, public_key, signature);
}
#[no_mangle]
fn stake() {
    let amount: U256 = runtime::get_named_arg("amount");
    StakingRewards::default().stake(amount);
}
#[no_mangle]
fn withdraw() {
    let amount: U256 = runtime::get_named_arg("amount");
    StakingRewards::default().withdraw(amount);
}
#[no_mangle]
fn get_reward() {
    StakingRewards::default().get_reward();
}
#[no_mangle]
fn exit() {
    StakingRewards::default().exit();
}
#[no_mangle]
fn notify_reward_amount() {
    let reward: U256 = runtime::get_named_arg("reward");
    let rewards_duration: U256 = runtime::get_named_arg("rewards_duration");
    StakingRewards::default().notify_reward_amount(reward, rewards_duration);
}
//Entry Points
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
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
        "notify_reward_amount",
        vec![
            Parameter::new("reward", U256::cl_type()),
            Parameter::new("rewards_duration", U256::cl_type()),
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
        let rewards_distribution: Key = runtime::get_named_arg("rewards_distribution");
        let rewards_token: Key = runtime::get_named_arg("rewards_token");
        let staking_token: Key = runtime::get_named_arg("staking_token");
        // Prepare constructor args
        let constructor_args = runtime_args! {
            "rewards_distribution" => rewards_distribution,
            "rewards_token" => rewards_token,
            "staking_token" => staking_token,
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
