//! Program entrypoint

#![cfg(all(
    any(target_os = "solana", target_arch = "bpf"),
    not(feature = "no-entrypoint")
))]

pinocchio::program_entrypoint!(crate::processor::process_instruction);
pinocchio::no_allocator!();
pinocchio::nostd_panic_handler!();
