#![no_std]

extern crate alloc;

pub mod data;
mod reentrancy_guard;

pub use reentrancy_guard::REENTRANCYGUARD;
