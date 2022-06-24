#![no_std]

extern crate alloc;

pub mod data;
mod staking_dual_rewards;
pub mod entry_points;

pub use staking_dual_rewards::STAKINGDUALREWARDS;
