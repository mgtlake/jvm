use std::borrow::Borrow;

use crate::class::*;
use crate::instructions::Instruction::*;
use crate::instructions::*;

use DataType::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DataType {
    Byte(u8),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Char(char),
    Bool(bool),
    ReturnAddress, // TODO work out how to represent this
    Reference,     // TODO work out how these work
    Void,          // Used as return value of void methods
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
            let mut jumped = false;

            match op {
                Nop => {}
                // TODO implement null references
                AConstNull => self.operand_stack.push(Reference),
                IConstM1 => self.operand_stack.push(Integer(-1)),
                IConst0 => self.operand_stack.push(Integer(0)),
                IConst1 => self.operand_stack.push(Integer(1)),
                IConst2 => self.operand_stack.push(Integer(2)),
                IConst3 => self.operand_stack.push(Integer(3)),
                IConst4 => self.operand_stack.push(Integer(4)),
                IConst5 => self.operand_stack.push(Integer(5)),
                LConst0 => self.operand_stack.push(Long(0)),
                LConst1 => self.operand_stack.push(Long(1)),
                FConst0 => self.operand_stack.push(Float(0.0)),
                FConst1 => self.operand_stack.push(Float(1.0)),
                FConst2 => self.operand_stack.push(Float(2.0)),
                LDC => {
                    let index = self.code[self.ip + 1] as usize;
                    let constant = self.class.get_constant_value(index).unwrap();
                    self.operand_stack.push(constant);
                }
                ILoad0 => self.operand_stack.push(self.local_variables[0]),
                ILoad1 => self.operand_stack.push(self.local_variables[1]),
                ILoad2 => self.operand_stack.push(self.local_variables[2]),
                ILoad3 => self.operand_stack.push(self.local_variables[3]),
                LLoad1 => self.operand_stack.push(self.local_variables[1]),
                ALoad0 => self.operand_stack.push(self.local_variables[0]),
                ALoad1 => self.operand_stack.push(self.local_variables[1]),
                Dup => self
                    .operand_stack
                    .push(self.operand_stack.last().copied().unwrap()),
                IAdd => {
                    let a = self.operand_stack.pop().unwrap();
                    let b = self.operand_stack.pop().unwrap();
                    let sum = match (a, b) {
                        (Integer(a_val), Integer(b_val)) => Some(a_val + b_val),
                        _ => None,
                    };
                    self.operand_stack.push(Integer(sum.unwrap()));
                }
                LAdd => {
                    let a = self.operand_stack.pop().unwrap();
                    let b = self.operand_stack.pop().unwrap();
                    let sum = match (a, b) {
                        (Long(a_val), Long(b_val)) => Some(a_val + b_val),
                        _ => None,
                    };
                    self.operand_stack.push(Long(sum.unwrap()));
                }
                IfACmpNeq => {
                    let branch_byte_1 = self.code[self.ip + 1] as usize;
                    let branch_byte_2 = self.code[self.ip + 2] as usize;
                    let target_address_offset = (branch_byte_1 << 8) | branch_byte_2;

                    let a = self.operand_stack.pop().unwrap();
                    let b = self.operand_stack.pop().unwrap();
                    if a != b {
                        jumped = true;
                        self.ip += target_address_offset;
                    }
                }
                GOTO => {
                    let branch_byte_1 = self.code[self.ip + 1] as usize;
                    let branch_byte_2 = self.code[self.ip + 2] as usize;
                    let target_address_offset = (branch_byte_1 << 8) | branch_byte_2;

                    jumped = true;
                    self.ip += target_address_offset;
                }
                IReturn | LReturn | FReturn | DReturn | AReturn => {
                    // TODO implement synchronized
                    return self.operand_stack.pop().unwrap();
                }
                Return => {
                    return Void;
                }
                InvokeSpecial => {
                    // TODO handle this complexity later
                }
            };
            println!("OP {:?} STACK {:?}", op, self.operand_stack);
            if !jumped {
                // If we jumped, don't need to manually update ip
                self.ip += op.get_width();
            }
        }
        return Void;
    }
}

pub fn load_frame(method: String, class: &Class, args: Vec<DataType>) -> Frame {
    // Find first method in class of that name that contains code
    // TODO do better than unwrap
    println!("Executing method {}", method);
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
