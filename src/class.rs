use std::io::{Read, Result};

use crate::attributes::Attribute::Code;
use crate::attributes::{parse_attributes, Attribute};
use crate::constants::Constant::{FieldInfo, NameAndTypeInfo};
use crate::constants::*;
use crate::execution::DataType;
use crate::fields::*;
use crate::instructions::Instruction;
use crate::methods::*;
use crate::read::*;

#[derive(Debug)]
pub struct Class {
    constant_pool: Vec<Constant>,
    pub name: String,
    super_name: String,
    // access_flags: TODO implement this
    interfaces: Vec<String>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    attributes: Vec<Attribute>,
}

// TODO see if there's any other impl opportunities
impl Class {
    // TODO consider if I want to make it an option
    // imo no, because failures aren't recoverable and should never ocur
    // But other places want it as an option
    pub fn get_constant(&self, i: usize) -> Option<&Constant> {
        self.constant_pool.get(i - 1)
    }

    pub fn get_code(&self, method_name: &str) -> Option<&Vec<Instruction>> {
        self.methods
            .iter()
            .filter(|method| method.name == method_name)
            .filter_map(|method| {
                method
                    .attributes
                    .iter()
                    .filter_map(|attribute| match attribute {
                        Code {
                            name,
                            max_stack,
                            max_locals,
                            code_length,
                            code,
                            exceptions,
                            attributes,
                        } => Some(code),
                        _ => None,
                    })
                    .next()
            })
            .next()
    }

    pub fn get_constant_value(&self, index: usize) -> Option<DataType> {
        let constant = match self.get_constant(index) {
            Some(c) => c,
            None => return None,
        };
        constant.get_constant_value(&self.constant_pool)
    }

    pub fn get_method_ref_from_constant(&self, index: usize) -> Option<(String, &Constant)> {
        let (class_index, name_and_type_index) = match self.get_constant(index) {
            Some(FieldInfo {
                tag,
                class_index,
                name_and_type_index,
            }) => (*class_index as usize, *name_and_type_index as usize),
            _ => return None,
        };

        let class_name = resolve_utf8(class_index, &self.constant_pool).unwrap();
        Some((
            class_name,
            self.get_constant(name_and_type_index).unwrap(),
        ))
    }

    pub fn has_method(&self, name: String) -> bool {
        self.methods.iter().any(|method| method.name == name)
    }

    pub fn get_method(&self, name_and_type: &Constant) -> Option<&Method> {
        match name_and_type {
            NameAndTypeInfo {
                tag,
                name_index,
                descriptor_index,
            } => self
                .methods
                .iter()
                .filter(|method| {
                    method.name == resolve_utf8(*name_index as usize, &self.constant_pool).unwrap()
                })
                .last(),
            _ => None,
        }
    }
}

fn parse_interfaces(reader: &mut dyn Read, constant_pool: &Vec<Constant>) -> Result<Vec<String>> {
    let mut interfaces = Vec::new();
    let interfaces_count = read_u2(reader)?;

    for _ in 0..interfaces_count {
        interfaces.push(resolve_utf8(read_u2(reader)? as usize, constant_pool).unwrap());
    }

    Ok(interfaces)
}

pub fn parse_class(reader: &mut dyn Read) -> Result<Class> {
    // Read first 4 bytes as magic value and check if it's valid
    let magic = read_u4(reader)?;
    if magic != 0xCAFEBABE {
        println!("{:x?}", magic);
        // TODO return Err(SomeError);
    }

    // Read next 4 bytes as version number
    // Ignore this since we don't care if the class is valid so long as it works
    let minor_version = read_u2(reader)?;
    let major_version = read_u2(reader)?;

    // Read constant pool
    let constant_pool = parse_constant_pool(reader)?;
    for i in 1..=constant_pool.len() {
        println!("Constant {}\n\t{:?}", i, constant_pool[i - 1]);
    }

    let access_flags = read_u2(reader)?; // TODO parse

    let this_class = resolve_utf8(read_u2(reader)? as usize, &constant_pool).unwrap();
    println!("This: {:?}", this_class);
    let super_class_index = read_u2(reader)?;
    let super_class = match super_class_index {
        0 => "N/A".to_string(), // This class must be Object, with no superclass
        _ => resolve_utf8(super_class_index as usize, &constant_pool).unwrap(),
    };
    println!("Super: {:?}", super_class);

    let interfaces = parse_interfaces(reader, &constant_pool)?;
    println!("Interfaces {:?}", interfaces);

    let fields = parse_fields(reader, &constant_pool)?;
    println!("Fields {:?}", fields);

    let methods = parse_methods(reader, &constant_pool)?;
    for i in 0..methods.len() {
        println!("Method {}\n\t{:?}", i, methods[i]);
    }

    let attributes = parse_attributes(reader, &constant_pool)?;
    for i in 0..attributes.len() {
        println!("Attribute {}\n\t{:?}", i, attributes[i]);
    }
    println!();

    Ok(Class {
        constant_pool,
        name: this_class,
        super_name: super_class,
        interfaces,
        fields,
        methods,
        attributes,
    })
}
