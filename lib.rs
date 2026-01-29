use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    pub counter: u32,
}

/// Instructions:
/// 0 => Increment(u32)
/// 1 => Decrement(u32)
/// 2 => Update(u32)
/// 3 => Reset
pub enum CounterInstructions {
    Increment(IncrementArgs),
    Decrement(DecrementArgs),
    Update(UpdateArgs),
    Reset,
}

pub struct IncrementArgs {
    pub value: u32,
}

pub struct DecrementArgs {
    pub value: u32,
}

pub struct UpdateArgs {
    pub value: u32,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            // Increment(u32)
            0 => {
                if rest.len() < 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let value = u32::from_le_bytes(rest[0..4].try_into().unwrap());
                CounterInstructions::Increment(IncrementArgs { value })
            }

            // Decrement(u32)
            1 => {
                if rest.len() < 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let value = u32::from_le_bytes(rest[0..4].try_into().unwrap());
                CounterInstructions::Decrement(DecrementArgs { value })
            }

            // Update(u32)
            2 => {
                if rest.len() < 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let value = u32::from_le_bytes(rest[0..4].try_into().unwrap());
                CounterInstructions::Update(UpdateArgs { value })
            }

            // Reset
            3 => CounterInstructions::Reset,

            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entry point");

    let instruction = CounterInstructions::unpack(instruction_data)?;

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstructions::Increment(args) => {
            msg!("Increment by {}", args.value);
            counter_account.counter = counter_account.counter.saturating_add(args.value);
        }

        CounterInstructions::Decrement(args) => {
            msg!("Decrement by {}", args.value);

            // If decrement is bigger than current counter -> set to 0
            if args.value > counter_account.counter {
                counter_account.counter = 0;
            } else {
                counter_account.counter -= args.value;
            }
        }

        CounterInstructions::Reset => {
            msg!("Reset");
            counter_account.counter = 0;
        }

        CounterInstructions::Update(args) => {
            msg!("Update to {}", args.value);
            counter_account.counter = args.value;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{clock::Epoch, pubkey::Pubkey};
    use std::mem;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;

        // Allocate enough space for CounterAccount
        let mut data = vec![0; mem::size_of::<CounterAccount>()];

        // Initialize account data with counter = 0
        CounterAccount { counter: 0 }
            .serialize(&mut &mut data[..])
            .unwrap();

        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        // Increment by 10
        let inc_value = 10u32;
        let mut increment_instruction_data: Vec<u8> = vec![0];
        increment_instruction_data.extend_from_slice(&inc_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            10
        );

        // Decrement by 3
        let dec_value = 3u32;
        let mut decrement_instruction_data: Vec<u8> = vec![1];
        decrement_instruction_data.extend_from_slice(&dec_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            7
        );

        // Decrement by 100 (should go to 0)
        let dec_value2 = 100u32;
        let mut decrement_instruction_data2: Vec<u8> = vec![1];
        decrement_instruction_data2.extend_from_slice(&dec_value2.to_le_bytes());

        process_instruction(&program_id, &accounts, &decrement_instruction_data2).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        // Update to 33
        let update_value = 33u32;
        let mut update_instruction_data: Vec<u8> = vec![2];
        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &update_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            33
        );

        // Reset
        let reset_instruction_data: Vec<u8> = vec![3];
        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
