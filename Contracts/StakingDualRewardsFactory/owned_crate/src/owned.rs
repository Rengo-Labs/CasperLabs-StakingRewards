use crate::alloc::string::ToString;
use crate::data::{self};
use alloc::collections::BTreeMap;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime};
use casper_types::{ ApiError, ContractPackageHash, Key, URef};
use contract_utils::{ContractContext, ContractStorage};

//Errors
#[repr(u16)]
pub enum Error {
    //Owner address cannot be 0
    OwnerAddressZero = 0,
    //You must be nominated before you can accept ownership
    NominatedBeforeAccept = 1,
    //Only the contract owner may perform this action
    OnlyContractOwner = 2,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
//Events
pub enum OwnedEvent {
    OwnerNominated { new_owner: Key },
    OwnerChanged { old_owner: Key, new_owner: Key },
}

impl OwnedEvent {
    pub fn type_name(&self) -> String {
        match self {
            OwnedEvent::OwnerNominated { new_owner: _ } => "OwnerNominated",
            OwnedEvent::OwnerChanged {
                old_owner: _,
                new_owner: _,
            } => "OwnerNominated",
        }
        .to_string()
    }
}
pub trait OWNED<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, owner: Key, contract_hash: Key, package_hash: ContractPackageHash) {
        if !(owner != data::ZERO_ADDRESS()) {
            runtime::revert(ApiError::from(Error::OwnerAddressZero));
        }
        data::set_owner(owner);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        self.owned_emit(&OwnedEvent::OwnerChanged {
            old_owner: data::ZERO_ADDRESS(),
            new_owner: data::get_owner(),
        });
    }
    fn nominate_new_owner(&mut self, owner: Key) {
        self.only_owner();
        data::set_nominated_owner(owner);
        self.owned_emit(&OwnedEvent::OwnerNominated { new_owner: owner });
    }
    fn accept_ownership(&mut self) {
        if !(self.get_caller() == data::get_nominated_owner()) {
            runtime::revert(ApiError::from(Error::NominatedBeforeAccept));
        }
        self.owned_emit(&OwnedEvent::OwnerChanged {
            old_owner: data::get_owner(),
            new_owner: data::get_nominated_owner(),
        });
        data::set_owner(data::get_nominated_owner());
        data::set_nominated_owner(data::ZERO_ADDRESS());
    }
    fn only_owner(&self) {
        self._only_owner();
    }
    fn _only_owner(&self) {
        if !(self.get_caller() == data::get_owner()) {
            runtime::revert(ApiError::from(Error::OnlyContractOwner));
        }
    }
    fn owned_emit(&mut self, owned_event: &OwnedEvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match owned_event {
            OwnedEvent::OwnerNominated { new_owner } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", owned_event.type_name());
                event.insert("new_owner", new_owner.to_string());
                events.push(event);
            }
            OwnedEvent::OwnerChanged {
                old_owner,
                new_owner,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", owned_event.type_name());
                event.insert("old_owner", old_owner.to_string());
                event.insert("new_owner", new_owner.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
