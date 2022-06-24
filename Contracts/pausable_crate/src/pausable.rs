use crate::alloc::string::ToString;
use crate::data;
use alloc::collections::BTreeMap;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::runtime;
use casper_contract::contract_api::storage;
use casper_types::{ApiError, ContractPackageHash, Key, URef, U256};
use contract_utils::{ContractContext, ContractStorage};
use owned_crate::{self, data as owned, OWNED};
//Errors
#[repr(u16)]
pub enum Error {
    /// Owner must be set
    OwnerMustSet = 20101,
    /// This action cannot be performed while the contract is paused
    ContractPaused = 20102,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
//Events
pub enum PausableEvent {
    PauseChanged { is_paused: bool },
}

impl PausableEvent {
    pub fn type_name(&self) -> String {
        match self {
            PausableEvent::PauseChanged { is_paused: _ } => "PauseChanged",
        }
        .to_string()
    }
}
pub trait PAUSABLE<Storage: ContractStorage>: ContractContext<Storage> + OWNED<Storage> {
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash) {
        OWNED::init(self, self.get_caller(), contract_hash, package_hash);
        if !(owned::get_owner() != data::zero_address()) {
            runtime::revert(ApiError::from(Error::OwnerMustSet));
        }
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
    }
    fn set_paused(&mut self, paused: bool) {
        OWNED::only_owner(self);
        if paused == data::get_paused() {
            return;
        }
        data::set_paused(paused);
        let blocktime: u64 = runtime::get_blocktime().into();
        if paused {
            data::set_last_pause_time(U256::from(blocktime));
        }
        self.pausable_emit(&PausableEvent::PauseChanged { is_paused: paused });
    }
    //Modifier
    fn not_paused(&self) {
        if !(!data::get_paused()) {
            runtime::revert(ApiError::from(Error::ContractPaused));
        }
    }
    fn pausable_emit(&mut self, pausable_event: &PausableEvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match pausable_event {
            PausableEvent::PauseChanged { is_paused } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", pausable_event.type_name());
                event.insert("is_paused", is_paused.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
