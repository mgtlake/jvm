use crate::read::*;
use crate::constants::*;

use std::io::{Read, Result};

#[derive(Debug)]
enum Visibility {
    Private,
    Protected,
    Public,
}

#[derive(Debug)]
struct Attribute {
    name: String,
    // TODO handle attributes properly
}

#[derive(Debug)]
struct Field {
    visibility: Visibility,
    is_static: bool,
    is_final: bool,
    is_volatile: bool, // Probably doesn't matter since we aren't caching anyway
    is_transient: bool,
    is_synthetic: bool,
    is_enum: bool,
    name: String,
    descriptor: String,
    attributes: Vec<Attribute>,
}

fn parse_fields<'a>(reader: &'a mut dyn Read, constant_pool: &'a Vec<Constant>) -> Result<Vec<&'a Constant>> {
    let mut fields = Vec::new();
    let fields_count = read_u2(reader)?;

    for _ in 0..fields_count {
        fields.push(&constant_pool[read_u2(reader)? as usize]);
    }

    Ok(fields)
}