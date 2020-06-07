use crate::constants::*;
use crate::read::*;

use std::io::{Read, Result};
use enumflags2::BitFlags;

#[derive(Debug)]
struct Exception {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug)]
struct BootstrapMethod {
    method_reference: Constant,
    num_arguments: u16,
    arguments: Vec<Constant>,
}

// TODO add more attributes
#[derive(Debug)]
enum Attribute {
    ConstantValue {
        name: String,
        value: Constant,
    },
    Code {
        name: String,
        length: u32, // TODO is needed?
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u8>, // Bytes
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
        length: u32, // TODO is needed?
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

#[derive(Debug)]
enum Visibility {
    Private,
    Protected,
    Public,
}

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u16)]
enum AccessFlagsBits {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,
    Enum = 0x4000,
}

#[derive(Debug)]
struct AccessFlags {
    visibility: Visibility,
    is_static: bool,
    is_final: bool,
    is_volatile: bool, // Probably doesn't matter since we aren't caching anyway
    is_transient: bool,
    is_synthetic: bool,
    is_enum: bool,
}

#[derive(Debug)]
pub struct Field {
    access_flags: AccessFlags,
    name: String,
    descriptor: String,
    attributes: Vec<Attribute>,
}

fn parse_access_flags(mask: u16) -> Result<AccessFlags> {
    let test: enumflags2::BitFlags<AccessFlagsBits> = BitFlags::from_bits(mask).unwrap();
    println!("bit flags : {:?}", test);
    Ok(AccessFlags {
        visibility: Visibility::Private,
        is_static: false,
        is_final: false,
        is_volatile: false,
        is_transient: false,
        is_synthetic: false,
        is_enum: false
    })
}

pub fn parse_fields<'a>(
    reader: &'a mut dyn Read,
    constant_pool: &'a Vec<Constant>,
) -> Result<Vec<Field>> {
    let mut fields = Vec::new();
    let fields_count = read_u2(reader)?;
    println!("Fields count: {:?}", fields_count);

    // TODO come back to this when I'm testing a class file with fields
    for _ in 0..fields_count {
        let access_flags = parse_access_flags(read_u2(reader)?)?;

        let name_index = read_u2(reader)? as usize;
        let name = &constant_pool[name_index];

        fields.push(Field {
            access_flags,
            name: "TEST".parse().unwrap(),
            descriptor: "".to_string(),
            attributes: vec![],
        });
    }

    Ok(fields)
}
