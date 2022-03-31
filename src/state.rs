use solana_program::{
    program_pack::{IsInitialized,Pack, Sealed},
    pubkey::Pubkey,
    program_error::ProgramError, 
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Counter {
    pub is_initialised : bool,
    pub count : u64, // used to store count
    pub owner : Pubkey,
}

impl Sealed for Counter {} // Need help explaining how this is used

impl IsInitialized for Counter {
    fn is_initialized(&self) -> bool {
        self.is_initialised // z vs s lol
    }
}

impl Pack for Counter {
    const LEN : usize = 1 + 8 + 32;
    fn unpack_from_slice(src : &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Counter::LEN]; // converts into a [u8;LEN] from &[u8]
        let (
            is_initialised,
            count,
            owner_pubkey,
        ) = array_refs![src,1,8,32]; // pass in our [u8;LEN] and the lengths of each variable
        

        // convert is_initialised into a bool to create struct
        let is_initialised = match is_initialised {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Counter {
            is_initialised,
            count : u64::from_le_bytes(*count),
            owner : Pubkey::new_from_array(*owner_pubkey),
        })
    }

    // turns counter struct into a byte array!
    fn pack_into_slice(&self, dst : &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Counter::LEN]; // converts into a mut [u8;LEN]
        let (
            is_initialised_dst,
            count_dst,
            owner_pubkey_dst,
        ) = mut_array_refs![dst,1,8,32]; // seperates our [u8;LEN] into individual
        // seperated our dst into variables for us to edit easier

        let Counter {
            is_initialised,
            count,
            owner,
        } = self; // gets values from self
        // turns our values from types into byte arrays
        is_initialised_dst[0] = *is_initialised as u8;
        *count_dst = count.to_le_bytes();
        owner_pubkey_dst.copy_from_slice(owner.as_ref());
    }
}