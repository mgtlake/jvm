use std::convert::TryFrom;

use bitflags::_core::convert::Infallible;
use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Instruction {
    Nop = 0,
    AConstNull = 1,
    IConstM1 = 2,
    IConst0 = 3,
    IConst1 = 4,
    IConst2 = 5,
    IConst3 = 6,
    IConst4 = 7,
    IConst5 = 8,
    LConst0 = 9,
    LConst1 = 10,
    FConst0 = 11,
    FConst1 = 12,
    FConst2 = 13,
    LDC = 18,
    ILoad0 = 26,
    ILoad1 = 27,
    ILoad2 = 28,
    ILoad3 = 29,
    LLoad1 = 31,
    ALoad0 = 42,
    ALoad1 = 43,
    Dup = 89,
    IAdd = 96,
    LAdd = 97,
    IfACmpNeq = 166,
    GOTO = 167,
    IReturn = 172,
    LReturn = 173,
    FReturn = 174,
    DReturn = 175,
    AReturn = 176,
    Return = 177,
    InvokeSpecial = 183,
}

impl Instruction {
    pub fn get_width(&self) -> usize {
        match self {
            Instruction::LDC => 2,           // Unsigned byte arg
            Instruction::IfACmpNeq => 3,     // 2 reference args
            Instruction::InvokeSpecial => 3, // 2 byte args
            _ => 1,
        }
    }
}

// TODO use Result
pub fn parse_code(block: Vec<u8>) -> Option<Vec<Instruction>> {
    let mut any_not_implemented = false;
    for &op in block.iter() {
        match Instruction::try_from(op) {
            Err(_) => {
                println!("op {}", op);
                any_not_implemented = true;
            }
            Ok(_) => {}
        }
    }
    if any_not_implemented {
        return None;
    }
    Some(
        block
            .iter()
            // TODO handle unwrapping better here
            .map(|&bytecode| Instruction::try_from(bytecode).unwrap())
            .collect(),
    )
}
