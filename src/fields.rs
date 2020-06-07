use crate::constants::*;
use crate::read::*;

use crate::fields::Visibility::*;
use bitflags::*;
use std::io::{Read, Result};

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

bitflags! {
    struct AccessFlagsBits: u16 {
        const Public = 0x0001;
        const Private = 0x0002;
        const Protected = 0x0004;
        const Static = 0x0008;
        const Final = 0x0010;
        const Volatile = 0x0040;
        const Transient = 0x0080;
        const Synthetic = 0x1000;
        const Enum = 0x4000;
    }
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
    let flags = AccessFlagsBits::from_bits(mask).unwrap();
    println!("bit flags : {:?}", flags);

    let visibility = if flags.contains(AccessFlagsBits::Private) {
        Private
    } else if flags.contains(AccessFlagsBits::Protected) {
        Protected
    } else if flags.contains(AccessFlagsBits::Public) {
        Public
    } else {
        // Default to public
        // TODO make this an error at some point
        Public
    };

    Ok(AccessFlags {
        visibility,
        is_static: flags.contains(AccessFlagsBits::Static),
        is_final: flags.contains(AccessFlagsBits::Final),
        is_volatile: flags.contains(AccessFlagsBits::Volatile),
        is_transient: flags.contains(AccessFlagsBits::Transient),
        is_synthetic: flags.contains(AccessFlagsBits::Synthetic),
        is_enum: flags.contains(AccessFlagsBits::Enum),
    })
}

pub fn parse_fields<'a>(
    reader: &'a mut dyn Read,
    constant_pool: &'a Vec<Constant>,
) -> Result<Vec<Field>> {
    let mut fields = Vec::new();
    let fields_count = read_u2(reader)?;

    // TODO come back to this when I'm testing a class file with fields
    for _ in 0..fields_count {
        let access_flags = parse_access_flags(read_u2(reader)?)?;

        let name = resolve_utf8(read_u2(reader)? as usize, constant_pool).unwrap();

        let descriptor = resolve_utf8(read_u2(reader)? as usize, constant_pool).unwrap();

        fields.push(Field {
            access_flags,
            name,
            descriptor,
            attributes: vec![], // TODO implement attributes
        });
    }

    Ok(fields)
}
