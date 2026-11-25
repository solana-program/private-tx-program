//! Program state processor

use pinocchio::{error::ProgramError, AccountView, Address, ProgramResult};

/// Processes an instruction. No instruction is implemented yet, so all of
/// them are rejected.
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &mut [AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    Err(ProgramError::InvalidInstructionData)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_every_instruction() {
        for data in [&[][..], &[0], &[1, 2, 3]] {
            assert_eq!(
                process_instruction(&crate::ID, &mut [], data),
                Err(ProgramError::InvalidInstructionData)
            );
        }
    }
}
