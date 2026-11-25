#![no_std]

//! Private transaction pool program.
//!
//! This crate is a placeholder: it has an entrypoint and rejects every
//! instruction. The implementation will be migrated here.

// The host build has no panic handler of its own.
#[cfg(not(any(target_os = "solana", target_arch = "bpf")))]
extern crate std;

mod entrypoint;
pub mod processor;

use pinocchio::Address;

/// This program. A placeholder: generate your own before deploying.
pub const ID: Address = Address::from_str_const("3sDoL7Tan84VmueUVRp1NRkhfPRF2z54ccf4J4LMHXdT");
