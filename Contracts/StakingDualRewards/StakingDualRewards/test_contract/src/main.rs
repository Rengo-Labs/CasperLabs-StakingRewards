#![no_main]
#![no_std]

extern crate alloc;
use alloc::{ collections::BTreeSet, format, vec};

use casper_contract::{
    contract_api::{ runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args,
    CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group,
    Key, Parameter, RuntimeArgs, URef, U256,
};
pub mod mappings;

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let staking_dual_rewards: Key = runtime::get_named_arg("staking_dual_rewards");

    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::staking_dual_rewards_key(),
        ContractPackageHash::from(staking_dual_rewards.into_hash().unwrap_or_default()),
    );
}

#[no_mangle]
fn total_supply() {
    let staking_rewards_address: ContractPackageHash = mappings::get_key(&mappings::staking_dual_rewards_key());
    let ret: U256 = runtime::call_versioned_contract(
        staking_rewards_address,
        None,
        "total_supply",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn balance_of() {
    let staking_rewards_address: ContractPackageHash = mappings::get_key(&mappings::staking_dual_rewards_key());
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = runtime::call_versioned_contract(
        staking_rewards_address,
        None,
        "balance_of",
        runtime_args! {
            "account" => account
        },
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn last_time_reward_applicable() {
    let staking_rewards_address: ContractPackageHash = mappings::get_key(&mappings::staking_dual_rewards_key());
    let ret: U256 = runtime::call_versioned_contract(
        staking_rewards_address,
        None,
        "last_time_reward_applicable",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn reward_per_token_a() {
    let staking_rewards_address: ContractPackageHash = mappings::get_key(&mappings::staking_dual_rewards_key());
    let ret: U256 = runtime::call_versioned_contract(
        staking_rewards_address,
        None,
        "reward_per_token_a",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn reward_per_token_b() {
    let staking_rewards_address: ContractPackageHash = mappings::get_key(&mappings::staking_dual_rewards_key());
    let ret: U256 = runtime::call_versioned_contract(
        staking_rewards_address,
        None,
        "reward_per_token_b",
        runtime_args! {},
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn earned_a() {
    let staking_rewards_address: ContractPackageHash = mappings::get_key(&mappings::staking_dual_rewards_key());
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = runtime::call_versioned_contract(
        staking_rewards_address,
        None,
        "earned_a",
        runtime_args! {
            "account" => account
        },
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn earned_b() {
    let staking_rewards_address: ContractPackageHash = mappings::get_key(&mappings::staking_dual_rewards_key());
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = runtime::call_versioned_contract(
        staking_rewards_address,
        None,
        "earned_b",
        runtime_args! {
            "account" => account
        },
    );
    mappings::set_key(&mappings::result_key(), ret);
}
#[no_mangle]
fn set_staking_dual_rewards() {
    let token: Key = runtime::get_named_arg("token");
    mappings::set_key(
        &mappings::staking_dual_rewards_key(),
        ContractHash::from(token.into_hash().unwrap_or_revert()),
    );
}
//Entry Points
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("staking_dual_rewards", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_staking_dual_rewards",
        vec![Parameter::new("token", CLType::Key)],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("account", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "last_time_reward_applicable",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_per_token_a",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_per_token_b",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "earned_a",
        vec![Parameter::new("account", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "earned_b",
        vec![Parameter::new("account", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    let staking_dual_rewards: Key = runtime::get_named_arg("staking_dual_rewards");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "staking_dual_rewards" => staking_dual_rewards
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
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
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
}
