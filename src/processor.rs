use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    program_pack::{Pack,IsInitialized},
};

use crate::{instruction::CounterInstruction, error::CounterError, state::Counter};

pub struct Processor;

impl Processor {
    // Take byte array and turn it into instructions for our processor
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    ) -> ProgramResult {
        let instruction : CounterInstruction = CounterInstruction::unpack(instruction_data)?;
        match instruction {
            CounterInstruction::InitCounter => {
                msg!("Initiating Counter!");
                Self::process_init(accounts,program_id)
            },
            CounterInstruction::Increment {amount} => {
                msg!("Incrementing Counter by {}",amount);
                Self::increment(accounts,amount,program_id)
            },
            CounterInstruction::Close => {
                msg!("Closing counter...");
                Self::close(accounts,program_id)
            }
        }
    }       
    
    fn process_init(
        accounts: &[AccountInfo],
        _program_id: &Pubkey,
    ) -> ProgramResult {
        // Check that intialiser has signer
        // check that our counter hasn't already been initialised
        // check that our counter is rent exempt
        let account_info_iter = &mut accounts.iter();
        let initialiser = next_account_info(account_info_iter)?;

        if !initialiser.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let counter_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(counter_account.lamports(), counter_account.data_len()) {
            return Err(CounterError::NotRentExempt.into());
        }

        let mut counter_info = Counter::unpack_unchecked(&counter_account.data.borrow())?;

        if counter_info.is_initialized() == true {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        counter_info.is_initialised = true;
        counter_info.count = 0;
        counter_info.owner = *initialiser.key;

        Counter::pack(counter_info, &mut counter_account.try_borrow_mut_data()?)?;

        Ok(())
    }

    fn increment(
        accounts: &[AccountInfo],
        amount : u64,
        _program_id : &Pubkey,
    ) -> ProgramResult {
        // Check that initialiser is signer
        // Check that initialiser is owner of counter account (not sure if will fail if counter is unitialised)
        // Will just fail the owner check if not initialised
        // Check counter has been intialised
        // Check for overflow errors
        let account_info_iter = &mut accounts.iter();
        let initialiser = next_account_info(account_info_iter)?;

        if !initialiser.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        
        let counter_account = next_account_info(account_info_iter)?;
        let mut counter_info = Counter::unpack_unchecked(&counter_account.data.borrow())?;

        // check correct user
        if counter_info.owner != *initialiser.key {
            return Err(CounterError::IncorrectUser.into());
        }
        // check overflow
        counter_info.count = counter_info.count.checked_add(amount).ok_or(CounterError::OverflowError)?;

        Counter::pack(counter_info, &mut counter_account.try_borrow_mut_data()?)?;

        Ok(())
    }

    fn close(
        accounts : &[AccountInfo],
        _program_id: &Pubkey,
    ) -> ProgramResult {
        // Check signer
        // Check owner is signer
        // Zero account data for good practice - malicious instructions
        let account_info_iter = &mut accounts.iter();
        let initialiser = next_account_info(account_info_iter)?;

        if !initialiser.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        
        let counter_account = next_account_info(account_info_iter)?;
        let counter_info = Counter::unpack_unchecked(&counter_account.data.borrow())?;

        // check correct user
        if counter_info.owner != *initialiser.key {
            return Err(CounterError::IncorrectUser.into());
        }

        msg!("Time to close the counter account");
        **initialiser.lamports.borrow_mut() = initialiser.lamports()
            .checked_add(counter_account.lamports())
            .ok_or(CounterError::OverflowError)?;
        
        **counter_account.lamports.borrow_mut() = 0;
        *counter_account.data.borrow_mut() = &mut [];
        
        Ok(())
    }
}