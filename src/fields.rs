use std::io::{Read, Result};

use bitflags::*;

use crate::attributes::*;
use crate::constants::*;
use crate::fields::Visibility::*;
use crate::read::*;

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

pub fn parse_fields(reader: &mut dyn Read, constant_pool: &Vec<Constant>) -> Result<Vec<Field>> {
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
            attributes: parse_attributes(reader, constant_pool)?,
        });
    }

    Ok(fields)
}
