use DataType::*;

use crate::class::*;
use crate::instructions::Instruction::*;
use crate::instructions::*;
use crate::methods::Method;

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
    method_name: &'a str,
}

impl Frame<'_> {
    pub fn exec(&mut self) -> DataType {
        println!("Code {:?}", self.code);
        while self.ip < self.code.len() {
            let op = self.code.get(self.ip).unwrap();
            println!("IP {} OP {:?}", self.ip, op);
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
                IStore1 => {
                    let value = self.operand_stack.pop().unwrap();
                    self.store_local(1, value);
                },
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
                InvokeStatic => {
                    let target_byte_1 = self.code[self.ip + 1] as usize;
                    let target_byte_2 = self.code[self.ip + 2] as usize;
                    let method_index = (target_byte_1 << 8) | target_byte_2;

                    let mut new_frame =
                        self.new_frame_for_static_method_from_constant(method_index);
                    let result = new_frame.exec();
                    self.operand_stack.push(result);
                    println!("Continue executing method {}", self.method_name);
                }
            };
            println!("\t↳ STACK {:?}", self.operand_stack);
            println!("\t↳ LOCALS {:?}", self.local_variables);
            if !jumped {
                // If we jumped, don't need to manually update ip
                self.ip += op.get_width();
            }
        }
        return Void;
    }

    fn new_frame_for_static_method_from_constant(&mut self, index: usize) -> Frame {
        // TODO implement a proper classpath
        let classes = vec![self.class];

        let (class_name, name_and_type) = self.class.get_method_ref_from_constant(index).unwrap();

        let method_class = classes
            .iter()
            .filter(|class| class.name == class_name)
            .last()
            .unwrap();
        let method = method_class.get_method(name_and_type).unwrap();

        let mut args = vec![];
        for _ in 0..method.num_args() {
            // TODO test that order is right
            args.push(self.operand_stack.pop().unwrap());
        }

        load_frame(&method.name, method_class, args)
    }

    fn store_local(&mut self, index: usize, value: DataType) {
        if index >= self.local_variables.len() {
            self.local_variables.resize(index + 1, Placeholder);
        }
        self.local_variables[index] = value;
    }
}

pub fn load_frame<'a>(method: &'a str, class: &'a Class, args: Vec<DataType>) -> Frame<'a> {
    // Find first method in class of that name that contains code
    // TODO do better than unwrap
    println!("Executing method {}", method);
    let code = class.get_code(method).unwrap();

    let locals = Vec::from(args);
    // TODO first item should be current object for instance invocation
    // Only for instance methods! - But why are we storing in position 1 then?
    // locals.push_left();

    Frame {
        local_variables: locals,
        operand_stack: vec![],
        class,
        ip: 0,
        code,
        method_name: method,
    }
}
