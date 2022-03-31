use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::CounterError::InvalidInstruction;

pub enum CounterInstruction {
    // Instruction sent when initialising counter program
    // 0. [signer] User creating their pda counter account
    // 1. [writable] Counter account
    // 2. `[]` The rent sysvar
    InitCounter,

    // Instruction to increment a counter program
    // 0. [signer] User incrementing their own pda counter program
    // 1. [writable] Counter account
    Increment {
        amount : u64,
    },

    // Instruction to close a counter program and reclaim lamports
    // 0. [signer] User closing their pda counter account
    // 1. [writable] Counter account
    Close,
}

impl CounterInstruction {
    // Pass in the input byte array and we return a CounterInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // ? returns InvalidInstruction if our result from .split_first() is a None
        let (tag,rest) = input.split_first().ok_or(InvalidInstruction)?;
        
        Ok(
            match tag {
                0 => Self::InitCounter,
                1 => Self::Increment {
                    amount : Self::unpack_amount(rest)?
                },
                2 => Self::Close,
                
                _ => return Err(InvalidInstruction.into()),
            }
        )
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8) // gets the first 8 bytes
            .and_then(|slice| slice.try_into().ok()) // calls if .get is Some(x) -> turns into a result
            .map(u64::from_le_bytes) // converts [u8;8] into u64
            .ok_or(InvalidInstruction)?; // propogates Ok() or returns InvalidInstruction
        Ok(amount)
    }
}