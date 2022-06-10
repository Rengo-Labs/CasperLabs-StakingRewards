#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use dual_rewards_distribution_recipient_crate::DUALREWARDSDISTRIBUTIONRECIPIENT;
use owned_crate::{self, OWNED};
use pausable_crate::{self, PAUSABLE};
use reentrancy_guard_crate::REENTRANCYGUARD;
use staking_dual_rewards_crate::{self,data, STAKINGDUALREWARDS};
#[derive(Default)]
struct StakingDualRewards(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for StakingDualRewards {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}
impl REENTRANCYGUARD<OnChainContractStorage> for StakingDualRewards {}
impl DUALREWARDSDISTRIBUTIONRECIPIENT<OnChainContractStorage> for StakingDualRewards {}
impl STAKINGDUALREWARDS<OnChainContractStorage> for StakingDualRewards {}
impl OWNED<OnChainContractStorage> for StakingDualRewards {}
impl PAUSABLE<OnChainContractStorage> for StakingDualRewards {}

impl StakingDualRewards {
    fn constructor(
        &mut self,
        owner: Key,
        dual_rewards_distribution: Key,
        rewards_token_a: Key,
        rewards_token_b: Key,
        staking_token: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        STAKINGDUALREWARDS::init(
            self,
            owner,
            dual_rewards_distribution,
            rewards_token_a,
            rewards_token_b,
            staking_token,
            Key::from(contract_hash),
            package_hash,
        )
    }
}

#[no_mangle]
fn constructor() {
    let owner: Key = runtime::get_named_arg("owner");
    let dual_rewards_distribution: Key = runtime::get_named_arg("dual_rewards_distribution");
    let rewards_token_a: Key = runtime::get_named_arg("rewards_token_a");
    let rewards_token_b: Key = runtime::get_named_arg("rewards_token_b");
    let staking_token: Key = runtime::get_named_arg("staking_token");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    StakingDualRewards::default().constructor(
        owner,
        dual_rewards_distribution,
        rewards_token_a,
        rewards_token_b,
        staking_token,
        contract_hash,
        package_hash,
    );
}
//data
#[no_mangle]
fn rewards_token_a() {
    runtime::ret(CLValue::from_t(data::get_rewards_token_a()).unwrap_or_revert());
}
#[no_mangle]
fn rewards_token_b() {
    runtime::ret(CLValue::from_t(data::get_rewards_token_b()).unwrap_or_revert());
}
#[no_mangle]
fn staking_token() {
    runtime::ret(CLValue::from_t(data::get_staking_token()).unwrap_or_revert());
}
#[no_mangle]
fn period_finish() {
    runtime::ret(CLValue::from_t(data::get_period_finish()).unwrap_or_revert());
}
#[no_mangle]
fn reward_rate_a() {
    runtime::ret(CLValue::from_t(data::get_reward_rate_a()).unwrap_or_revert());
}
#[no_mangle]
fn reward_rate_b() {
    runtime::ret(CLValue::from_t(data::get_reward_rate_b()).unwrap_or_revert());
}
#[no_mangle]
fn last_update_time() {
    runtime::ret(CLValue::from_t(data::get_last_update_time()).unwrap_or_revert());
}
#[no_mangle]
fn reward_per_token_a_stored() {
    runtime::ret(CLValue::from_t(data::get_reward_per_token_a_stored()).unwrap_or_revert());
}
#[no_mangle]
fn reward_per_token_b_stored() {
    runtime::ret(CLValue::from_t(data::get_reward_per_token_b_stored()).unwrap_or_revert());
}
#[no_mangle]
fn user_reward_per_token_a_paid() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(data::UserRewardPerTokenAPaid::instance().get(&owner)).unwrap_or_revert());
}
#[no_mangle]
fn user_reward_per_token_b_paid() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(data::UserRewardPerTokenBPaid::instance().get(&owner)).unwrap_or_revert());
}
#[no_mangle]
fn rewards_a() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(data::RewardsA::instance().get(&owner)).unwrap_or_revert());
}
#[no_mangle]
fn rewards_b() {
    let owner: Key = runtime::get_named_arg("owner");
    runtime::ret(CLValue::from_t(data::RewardsB::instance().get(&owner)).unwrap_or_revert());
}
//functions
#[no_mangle]
fn set_paused() {
    let paused: bool = runtime::get_named_arg("paused");
    PAUSABLE::set_paused(&mut StakingDualRewards::default(), paused);
}
#[no_mangle]
fn nominate_new_owner() {
    let owner: Key = runtime::get_named_arg("owner");
    OWNED::nominate_new_owner(&mut StakingDualRewards::default(), owner);
}
#[no_mangle]
fn accept_ownership() {
    OWNED::accept_ownership(&mut StakingDualRewards::default());
}
#[no_mangle]
fn total_supply() {
    let ret: U256 = StakingDualRewards::default().total_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn balance_of() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = StakingDualRewards::default().balance_of(account);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn last_time_reward_applicable() {
    let ret: U256 = StakingDualRewards::default().last_time_reward_applicable();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn reward_per_token_a() {
    let ret: U256 = StakingDualRewards::default().reward_per_token_a();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn reward_per_token_b() {
    let ret: U256 = StakingDualRewards::default().reward_per_token_b();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn earned_a() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = StakingDualRewards::default().earned_a(account);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn earned_b() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = StakingDualRewards::default().earned_b(account);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn total_supply_js_client() {
    let ret: U256 = StakingDualRewards::default().total_supply();
    data::js_ret(ret)
}
#[no_mangle]
fn balance_of_js_client() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = StakingDualRewards::default().balance_of(account);
    data::js_ret(ret)
}
#[no_mangle]
fn last_time_reward_applicable_js_client() {
    let ret: U256 = StakingDualRewards::default().last_time_reward_applicable();
    data::js_ret(ret)
}
#[no_mangle]
fn reward_per_token_a_js_client() {
    let ret: U256 = StakingDualRewards::default().reward_per_token_a();
    data::js_ret(ret)
}
#[no_mangle]
fn reward_per_token_b_js_client() {
    let ret: U256 = StakingDualRewards::default().reward_per_token_b();
    data::js_ret(ret)
}
#[no_mangle]
fn earned_a_js_client() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = StakingDualRewards::default().earned_a(account);
    data::js_ret(ret)
}
#[no_mangle]
fn earned_b_js_client() {
    let account: Key = runtime::get_named_arg("account");
    let ret: U256 = StakingDualRewards::default().earned_b(account);
    data::js_ret(ret)
}
#[no_mangle]
fn stake() {
    let amount: U256 = runtime::get_named_arg("amount");
    StakingDualRewards::default().stake(amount);
}
#[no_mangle]
fn withdraw() {
    let amount: U256 = runtime::get_named_arg("amount");
    StakingDualRewards::default().withdraw(amount);
}
#[no_mangle]
fn get_reward() {
    StakingDualRewards::default().get_reward();
}
#[no_mangle]
fn exit() {
    StakingDualRewards::default().exit();
}
#[no_mangle]
fn notify_reward_amount() {
    let reward_a: U256 = runtime::get_named_arg("reward_a");
    let reward_b: U256 = runtime::get_named_arg("reward_b");
    let rewards_duration: U256 = runtime::get_named_arg("rewards_duration");
    StakingDualRewards::default().notify_reward_amount(reward_a, reward_b, rewards_duration);
}
#[no_mangle]
fn recover_erc20() {
    let token_address: Key = runtime::get_named_arg("token_address");
    let token_amount: U256 = runtime::get_named_arg("token_amount");
    StakingDualRewards::default().recover_erc20(token_address, token_amount);
}
//Entry Points
fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("dual_rewards_distribution", Key::cl_type()),
            Parameter::new("rewards_token_a", Key::cl_type()),
            Parameter::new("rewards_token_b", Key::cl_type()),
            Parameter::new("staking_token", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "rewards_token_a",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "rewards_token_b",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "staking_token",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "period_finish",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_rate_a",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_rate_b",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "last_update_time",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_per_token_a_stored",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_per_token_b_stored",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "user_reward_per_token_a_paid",
        vec![Parameter::new("owner", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "user_reward_per_token_b_paid",
        vec![Parameter::new("owner", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "rewards_a",
        vec![Parameter::new("owner", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "rewards_b",
        vec![Parameter::new("owner", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "nominate_new_owner",
        vec![Parameter::new("owner", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "accept_ownership",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_paused",
        vec![Parameter::new("paused", bool::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
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
        "reward_per_token_a",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_per_token_b",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "earned_a",
        vec![Parameter::new("account", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "earned_b",
        vec![Parameter::new("account", Key::cl_type())],
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
        "balance_of_js_client",
        vec![Parameter::new("account", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "last_time_reward_applicable_js_client",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_per_token_a_js_client",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_per_token_b_js_client",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "earned_a_js_client",
        vec![Parameter::new("account", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "earned_b_js_client",
        vec![Parameter::new("account", Key::cl_type())],
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
            Parameter::new("reward_a", U256::cl_type()),
            Parameter::new("reward_b", U256::cl_type()),
            Parameter::new("rewards_duration", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "recover_erc20",
        vec![
            Parameter::new("token_address", Key::cl_type()),
            Parameter::new("token_amount", U256::cl_type()),
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
        // Prepare constructor args
        let owner: Key = runtime::get_named_arg("owner");
        let dual_rewards_distribution: Key = runtime::get_named_arg("dual_rewards_distribution");
        let rewards_token_a: Key = runtime::get_named_arg("rewards_token_a");
        let rewards_token_b: Key = runtime::get_named_arg("rewards_token_b");
        let staking_token: Key = runtime::get_named_arg("staking_token");
        let constructor_args = runtime_args! {
            "owner" => owner,
            "dual_rewards_distribution" => dual_rewards_distribution,
            "rewards_token_a" => rewards_token_a,
            "rewards_token_b" => rewards_token_b,
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
