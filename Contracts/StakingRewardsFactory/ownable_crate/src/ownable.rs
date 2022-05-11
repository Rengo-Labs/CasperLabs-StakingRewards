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
    //Ownable: caller is not the owner
    NotOwner = 0,
    //Ownable: new owner is the zero address
    NewOwnerAddressZero = 1,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
//Events
pub enum OwnableEvent {
    OwnershipTransferred { previous_owner: Key, new_owner: Key },
}

impl OwnableEvent {
    pub fn type_name(&self) -> String {
        match self {
            OwnableEvent::OwnershipTransferred {
                previous_owner: _,
                new_owner: _,
            } => "OwnershipTransferred",
        }
        .to_string()
    }
}
pub trait OWNABLE<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, contract_hash: Key, package_hash: ContractPackageHash) {
        data::set_owner(self.get_caller());
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        self.ownable_emit(&OwnableEvent::OwnershipTransferred {
            previous_owner: data::ZERO_ADDRESS(),
            new_owner: data::get_owner(),
        });
    }
    fn owner(&self) -> Key {
        return data::get_owner();
    }

    fn only_owner(&self) {
        if !(self.is_owner()) {
            runtime::revert(ApiError::from(Error::NotOwner));
        }
    }

    fn is_owner(&self) -> bool {
        return self.get_caller() == data::get_owner();
    }

    fn renounce_ownership(&mut self) {
        self.only_owner();
        self.ownable_emit(&OwnableEvent::OwnershipTransferred {
            previous_owner: data::get_owner(),
            new_owner: data::ZERO_ADDRESS(),
        });
        data::set_owner(data::ZERO_ADDRESS());
    }
    fn transfer_ownership(&mut self, new_owner: Key) {
        self.only_owner();
        self._transfer_ownership(new_owner);
    }
    fn _transfer_ownership(&mut self, new_owner: Key) {
        if !(new_owner != data::ZERO_ADDRESS()) {
            runtime::revert(ApiError::from(Error::NewOwnerAddressZero));
        }
        self.ownable_emit(&OwnableEvent::OwnershipTransferred {
            previous_owner: data::get_owner(),
            new_owner: new_owner,
        });
        data::set_owner(new_owner);
    }

    fn ownable_emit(&mut self, ownable_event: &OwnableEvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match ownable_event {
            OwnableEvent::OwnershipTransferred {
                previous_owner,
                new_owner,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", ownable_event.type_name());
                event.insert("previous_owner", previous_owner.to_string());
                event.insert("new_owner", new_owner.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
