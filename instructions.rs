use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize)]
pub struct UpdateArgs {
    pub value: u32,
}

pub enum CounterInstructions {
    Increment,
    Decrement,
    Update(UpdateArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        let instruction = match variant {
            0 => Self::Increment,
            1 => Self::Decrement,
            2 => {
                let args = UpdateArgs::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Self::Update(args)
            }
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        };

        Ok(instruction)
    }
}
