use crate::constants::*;
use crate::instructions::*;
use crate::read::*;

use crate::attributes::Attribute::{Code, ConstantValue, Empty};
use crate::instructions::Instruction;
use std::io::{Read, Result};

#[derive(Debug)]
pub struct Exception {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: Option<String>,
}

fn parse_exception(reader: &mut dyn Read, constant_pool: &Vec<Constant>) -> Result<Exception> {
    let start_pc = read_u2(reader)?;
    let end_pc = read_u2(reader)?;
    let handler_pc = read_u2(reader)?;
    let catch_index = read_u2(reader)? as usize;

    Ok(Exception {
        start_pc,
        end_pc,
        handler_pc,
        catch_type: if catch_index > 0 {
            Some(resolve_utf8(catch_index, constant_pool).unwrap())
        } else {
            None
        },
    })
}

#[derive(Debug)]
pub struct BootstrapMethod {
    method_reference: Constant,
    num_arguments: u16,
    arguments: Vec<Constant>,
}

// TODO add more attributes
#[derive(Debug)]
pub enum Attribute {
    Empty, // Only used to satisfy Rust's match completeness check
    ConstantValue {
        name: String,
        value: crate::constants::ConstantValue,
    },
    Code {
        name: String,
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<Instruction>,
        exceptions: Vec<Exception>,
        attributes: Vec<Attribute>,
    },
    StackMapTable {
        name: String,
        // entries: Vec<StackMapFrame>,
        // TODO too much work, will revisit if I decide to do verification
    },
    BootstrapMethods {
        name: String,
        methods: Vec<BootstrapMethod>,
    },
    NestHost {
        name: String,
        host_class: Constant,
    },
    NestMembers {
        name: String,
        num_classes: u16,
        classes: Vec<Constant>,
    },
}

pub fn parse_attributes(
    reader: &mut dyn Read,
    constant_pool: &Vec<Constant>,
) -> Result<Vec<Attribute>> {
    let mut attributes = Vec::new();
    let attributes_count = read_u2(reader)?;

    for _ in 0..attributes_count {
        let name = resolve_utf8(read_u2(reader)? as usize, constant_pool).unwrap();

        let length = read_u4(reader)?;

        let attribute = match name.as_str() {
            "ConstantValue" => ConstantValue {
                name,
                value: get_constant_value(&constant_pool[read_u2(reader)? as usize], constant_pool),
            },
            "Code" => {
                let max_stack = read_u2(reader)?;
                let max_locals = read_u2(reader)?;
                let code_length = read_u4(reader)?;
                let code = parse_code(read_bytes(code_length as u64, reader)?).unwrap();

                let exceptions_length = read_u2(reader)?;
                let mut exceptions = Vec::new();
                for _ in 0..exceptions_length {
                    exceptions.push(parse_exception(reader, constant_pool)?);
                }

                Code {
                    name,
                    max_stack,
                    max_locals,
                    code_length,
                    code,
                    exceptions,
                    attributes: parse_attributes(reader, constant_pool)?,
                }
            }
            _ => {
                println!("Unkown attribute: {}", name);
                // Read anyway, so we can continue
                read_bytes(length as u64, reader);
                Empty
            }
        };

        attributes.push(attribute);
    }

    Ok(attributes)
}
