use crate::constants::*;
use crate::fields::*;
use crate::methods::*;
use crate::read::*;

use crate::attributes::Attribute::Code;
use crate::attributes::{parse_attributes, Attribute};
use crate::instructions::Instruction;
use std::fs::File;
use std::io::{Read, Result};

#[derive(Debug)]
pub struct Class {
    constant_pool: Vec<Constant>,
    name: String,
    super_name: String,
    // access_flags: TODO implement this
    interfaces: Vec<String>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    attributes: Vec<Attribute>,
}

// TODO see if there's any other impl opportunities
impl Class {
    pub fn get_code(&self, method_name: String) -> Option<&Vec<Instruction>> {
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
}

fn parse_interfaces(reader: &mut dyn Read, constant_pool: &Vec<Constant>) -> Result<Vec<String>> {
    let mut interfaces = Vec::new();
    let interfaces_count = read_u2(reader)?;

    for _ in 0..interfaces_count {
        interfaces.push(resolve_utf8(read_u2(reader)? as usize, constant_pool).unwrap());
    }

    Ok(interfaces)
}

pub fn parse_class() -> Result<Class> {
    // TODO abstract this out
    let reader = &mut File::open("/home/mgtlake/Code/jvm/test/Add/Add.class")?;

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
        println!("{} - {:?}", i, constant_pool[i - 1]);
    }

    let access_flags = read_u2(reader)?; // TODO parse

    let this_class = resolve_utf8(read_u2(reader)? as usize, &constant_pool).unwrap();
    println!("This: {:?}", this_class);
    let super_class = resolve_utf8(read_u2(reader)? as usize, &constant_pool).unwrap();
    println!("Super: {:?}", super_class);

    let interfaces = parse_interfaces(reader, &constant_pool)?;
    println!("{:?}", interfaces);

    let fields = parse_fields(reader, &constant_pool)?;
    println!("{:?}", fields);

    let methods = parse_methods(reader, &constant_pool)?;
    for i in 0..methods.len() {
        println!("{} - {:?}", i, methods[i]);
    }

    let attributes = parse_attributes(reader, &constant_pool)?;
    for i in 0..attributes.len() {
        println!("{} - {:?}", i, attributes[i]);
    }

    Ok(Class {
        constant_pool: constant_pool,
        name: this_class,
        super_name: super_class,
        interfaces,
        fields,
        methods,
        attributes,
    })
}
