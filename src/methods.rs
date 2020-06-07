use crate::attributes::*;
use crate::constants::*;
use crate::read::*;

use crate::methods::Visibility::*;
use bitflags;
use std::io::{Read, Result};

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
        const Synchronized = 0x0020;
        const Bridge = 0x0040;
        const Varargs = 0x0080;
        const Native = 0x0100;
        const Abstract = 0x0400;
        const Strict = 0x0800;
        const Synthetic = 0x1000;
    }
}

#[derive(Debug)]
struct AccessFlags {
    visibility: Visibility,
    is_static: bool,
    is_final: bool,
    is_synchronized: bool, // This probably won't matter unless I do multi-threading
    is_bridge: bool,
    is_varargs: bool,
    is_native: bool,
    is_abstract: bool,
    is_strict: bool,
    is_synthetic: bool,
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
        is_synchronized: flags.contains(AccessFlagsBits::Synchronized),
        is_bridge: flags.contains(AccessFlagsBits::Bridge),
        is_varargs: flags.contains(AccessFlagsBits::Varargs),
        is_native: flags.contains(AccessFlagsBits::Native),
        is_abstract: flags.contains(AccessFlagsBits::Abstract),
        is_strict: flags.contains(AccessFlagsBits::Strict),
        is_synthetic: flags.contains(AccessFlagsBits::Synthetic),
    })
}

#[derive(Debug)]
pub struct Method {
    access_flags: AccessFlags,
    name: String,
    descriptor: String, // TODO do I want this to be an enum?
    attributes: Vec<Attribute>,
}

pub fn parse_methods<'a>(
    reader: &mut dyn Read,
    constant_pool: &'a Vec<Constant>,
) -> Result<Vec<Method>> {
    let mut methods = Vec::new();
    let methods_count = read_u2(reader)?;

    for _ in 0..methods_count {
        let access_flags = parse_access_flags(read_u2(reader)?)?;

        let name = resolve_utf8(read_u2(reader)? as usize, constant_pool).unwrap();

        let descriptor = resolve_utf8(read_u2(reader)? as usize, constant_pool).unwrap();

        methods.push(Method {
            access_flags,
            name,
            descriptor,
            attributes: parse_attributes(reader, constant_pool)?,
        });
    }

    Ok(methods)
}
