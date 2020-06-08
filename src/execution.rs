use crate::class::*;
use crate::instructions::*;
use std::borrow::Borrow;

#[derive(Copy, Clone, Debug)]
pub enum DataType {
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Char(char),
    ReturnAddress, // TODO work out how to represent this
    Reference,     // TODO work out how these work
    Void, // Used as return value of void methods
    Placeholder,   // Used for double-width types Long, Double
}

pub struct Frame<'a> {
    // TODO account for that stupid double-wide scheme
    local_variables: Vec<DataType>,
    operand_stack: Vec<DataType>,
    // TODO dynamic linking - reference to constant pool
    class: &'a Class,
    ip: usize,
    code: &'a Vec<Instruction>,
}

impl Frame<'_> {
    pub fn exec(&mut self) -> DataType {
        while self.ip < self.code.len() {
            let op = self.code.get(self.ip).unwrap();

            match op {
                Instruction::Nop => {}
                // TODO implement null references
                Instruction::AConstNull => self.operand_stack.push(DataType::Reference),
                Instruction::ILoad0 => self.operand_stack.push(self.local_variables[0]),
                Instruction::ILoad1 => self.operand_stack.push(self.local_variables[1]),
                Instruction::ALoad0 => self.operand_stack.push(self.local_variables[0]),
                Instruction::IAdd => {
                    let a = self.operand_stack.pop().unwrap();
                    let b = self.operand_stack.pop().unwrap();
                    let sum = match (a, b) {
                        (DataType::Integer(a_val), DataType::Integer(b_val)) => Some(a_val + b_val),
                        _ => None,
                    };
                    self.operand_stack.push(DataType::Integer(sum.unwrap()));
                }
                Instruction::IReturn => {
                    return self.operand_stack.pop().unwrap();
                }
                Instruction::Return => {
                    return DataType::Void;
                }
                Instruction::InvokeSpecial => {
                    // TODO handle this complexity later
                }
            };
            println!("OP {:?} STACK {:?}", op, self.operand_stack);
            self.ip += 1;
        }
        return DataType::Void;
    }
}

pub fn load_frame<'a>(method: String, class: &'a Class, args: Vec<DataType>) -> Frame<'a> {
    // Find first method in class of that name that contains code
    // TODO do better than unwrap
    let code = class.get_code(method).unwrap();

    let locals = Vec::from(args);
    // TODO first item should be current object for instance invocation

    Frame {
        local_variables: locals,
        operand_stack: vec![],
        class,
        ip: 0,
        code,
    }
}
