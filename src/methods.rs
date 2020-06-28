use std::io::{Read, Result};

use bitflags;
use pest::iterators::Pair;
use pest::Parser;
use regex::Regex;

use crate::attributes::*;
use crate::constants::*;
use crate::methods::Visibility::*;
use crate::read::*;

#[derive(Debug)]
enum Visibility {
    Private,
    Protected,
    Public,
}

bitflags! {
    struct AccessFlagsBits: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const SYNCHRONIZED = 0x0020;
        const BRIDGE = 0x0040;
        const VARARGS = 0x0080;
        const NATIVE = 0x0100;
        const ABSTRACT = 0x0400;
        const STRICT = 0x0800;
        const SYNTHETIC = 0x1000;
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

    let visibility = if flags.contains(AccessFlagsBits::PRIVATE) {
        Private
    } else if flags.contains(AccessFlagsBits::PROTECTED) {
        Protected
    } else if flags.contains(AccessFlagsBits::PUBLIC) {
        Public
    } else {
        // Default to public
        // TODO make this an error at some point
        Public
    };

    Ok(AccessFlags {
        visibility,
        is_static: flags.contains(AccessFlagsBits::STATIC),
        is_final: flags.contains(AccessFlagsBits::FINAL),
        is_synchronized: flags.contains(AccessFlagsBits::SYNCHRONIZED),
        is_bridge: flags.contains(AccessFlagsBits::BRIDGE),
        is_varargs: flags.contains(AccessFlagsBits::VARARGS),
        is_native: flags.contains(AccessFlagsBits::NATIVE),
        is_abstract: flags.contains(AccessFlagsBits::ABSTRACT),
        is_strict: flags.contains(AccessFlagsBits::STRICT),
        is_synthetic: flags.contains(AccessFlagsBits::SYNTHETIC),
    })
}

#[derive(Debug)]
pub struct Method {
    access_flags: AccessFlags,
    pub name: String,
    return_type: ReturnDescriptor, // TODO do I want this to be an enum?
    arg_types: Vec<FieldDescriptor>, // TODO do I want this to be an enum?
    pub attributes: Vec<Attribute>,
}

impl Method {
    pub fn num_args(&self) -> usize {
        self.arg_types.len()
    }
}

#[derive(Debug)]
enum FieldDescriptor {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Ref(String),
    Short,
    Bool,
    //Array(FieldDescriptor), TODO work out how to store type
    Array,
    Placeholder,
}

#[derive(Parser)]
#[grammar = "method_descriptor.pest"]
pub struct MethodDescriptor;

fn parse_field(field: Pair<Rule>) -> FieldDescriptor {
    let inner = field.into_inner().peek().unwrap();
    match inner.as_rule() {
        Rule::base => match inner.as_span().as_str() {
            "B" => FieldDescriptor::Byte,
            "C" => FieldDescriptor::Char,
            "D" => FieldDescriptor::Double,
            "F" => FieldDescriptor::Float,
            "I" => FieldDescriptor::Int,
            "J" => FieldDescriptor::Long,
            "S" => FieldDescriptor::Short,
            "Z" => FieldDescriptor::Bool,
            _ => FieldDescriptor::Placeholder,
        },
        Rule::reference => FieldDescriptor::Ref(
            inner
                .into_inner()
                .peek()
                .unwrap()
                .as_span()
                .as_str()
                .parse()
                .unwrap(),
        ),
        Rule::array => FieldDescriptor::Array,
        x => {
            println!("Unknown field descriptor ({:?})", x);
            FieldDescriptor::Placeholder
        }
    }
}

#[derive(Debug)]
enum ReturnDescriptor {
    Return(FieldDescriptor),
    Void,
}

fn parse_return(pair: Pair<Rule>) -> ReturnDescriptor {
    match pair.as_str() {
        "V" => ReturnDescriptor::Void,
        _ => ReturnDescriptor::Return(parse_field(pair.into_inner().peek().unwrap())),
    }
}

// TODO turn this into proper error handing
fn parse_descriptor(descriptor: &str) -> Option<(Vec<FieldDescriptor>, ReturnDescriptor)> {
    let mut arg_types = vec![];
    let mut return_type = None;

    let parse = MethodDescriptor::parse(Rule::method, &descriptor).unwrap();
    let method = parse.peek().unwrap();

    let descriptors = method.into_inner();

    for pair in descriptors {
        match pair.as_rule() {
            Rule::field => {
                arg_types.push(parse_field(pair));
            }
            Rule::result => {
                return_type = Some(parse_return(pair));
            }
            _ => {}
        };
    }
    match return_type {
        Some(x) => Some((arg_types, x)),
        None => None,
    }
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
        // println!("Method: {}", name);
        let descriptor = resolve_utf8(read_u2(reader)? as usize, constant_pool).unwrap();

        let caps = Regex::new(r"\((.*)\)(.*)")
            .unwrap()
            .captures(descriptor.as_ref())
            .unwrap();
        let (arg_types, return_type) = parse_descriptor(caps.get(0).unwrap().as_str()).unwrap();

        methods.push(Method {
            access_flags,
            name,
            return_type,
            arg_types,
            attributes: parse_attributes(reader, constant_pool)?,
        });
    }

    Ok(methods)
}
