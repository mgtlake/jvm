use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Instruction {
    Nop = 0,
    AConstNull = 1,
    ILoad0 = 26,
    ILoad1 = 27,
    ALoad0 = 42,
    IAdd = 96,
    IReturn = 172,
    Return = 177,
    InvokeSpecial = 183,
}

// TODO use Result
pub fn parse_code(block: Vec<u8>) -> Option<Vec<Instruction>> {
    Some(
        block
            .iter()
            // TODO handle unwrapping better here
            .map(|&bytecode| Instruction::try_from(bytecode).unwrap())
            .collect(),
    )
}
