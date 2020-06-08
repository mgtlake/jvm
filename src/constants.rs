use crate::read::*;

use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::io::{Read, Result};
use std::str;
use ConstantInfo::*;
use ConstantTag::*;

#[derive(Debug)]
pub enum ConstantValue {
    Empty, // Satisfy compiler
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(String),
}

// TODO do I even need this?
pub fn get_constant_value(constant: &Constant, constant_pool: &Vec<Constant>) -> ConstantValue {
    match constant.info {
        IntInfo { tag, value } => ConstantValue::Integer(value),
        FloatInfo { tag, value } => ConstantValue::Float(value),
        LongInfo { tag, value } => ConstantValue::Long(value),
        DoubleInfo { tag, value } => ConstantValue::Double(value),
        StringInfo { tag, index } => {
            ConstantValue::String(resolve_utf8(index as usize, constant_pool).unwrap())
        }
        _ => ConstantValue::Empty,
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum ConstantTag {
    Class = 7,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodRef = 11,
    ConstString = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
    Placeholder,
}

#[derive(Debug)]
pub enum ConstantInfo {
    Empty,
    ClassInfo {
        tag: ConstantTag,
        name_index: u16,
    },
    FieldInfo {
        tag: ConstantTag,
        name_index: u16,
        name_and_type_index: u16,
    },
    StringInfo {
        tag: ConstantTag,
        index: u16,
    },
    IntInfo {
        tag: ConstantTag,
        value: i32,
    },
    FloatInfo {
        tag: ConstantTag,
        value: f32,
    },
    LongInfo {
        tag: ConstantTag,
        value: i64,
    },
    DoubleInfo {
        tag: ConstantTag,
        value: f64,
    },
    NameAndTypeInfo {
        tag: ConstantTag,
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8Info {
        tag: ConstantTag,
        value: String,
    },
    MethodHandleInfo {
        tag: ConstantTag,
        kind: u8, // TODO see if I need an enum for this
        index: u16,
    },
    MethodTypeInfo {
        tag: ConstantTag,
        descriptor_index: u16,
    },
    DynamicInfo {
        tag: ConstantTag,
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    ModuleInfo {
        tag: ConstantTag,
        name_index: u16,
    },
    PackageInfo {
        tag: ConstantTag,
        name_index: u16,
    },
}

#[derive(Debug)]
pub struct Constant {
    tag: ConstantTag,
    info: ConstantInfo,
}

pub fn parse_constant_pool(reader: &mut dyn Read) -> Result<Vec<Constant>> {
    let mut pool = Vec::new();
    let constant_pool_count = read_u2(reader)?;

    // Insert a placeholder for double-width constants Long, Double
    let mut skip = false;

    for _ in 1..constant_pool_count {
        if skip {
            pool.push(Constant {
                tag: Placeholder,
                info: Empty,
            });
            skip = false;
            continue;
        }
        let tag_num = read_u1(reader)?;
        let tag = ConstantTag::try_from(tag_num).unwrap();
        let constant = Constant {
            tag,
            info: match tag {
                Class => ClassInfo {
                    tag,
                    name_index: read_u2(reader)?,
                },
                FieldRef | MethodRef | InterfaceMethodRef => FieldInfo {
                    tag,
                    name_index: read_u2(reader)?,
                    name_and_type_index: read_u2(reader)?,
                },
                ConstString => StringInfo {
                    tag,
                    index: read_u2(reader)?,
                },
                Integer => IntInfo {
                    tag,
                    value: read_u4(reader)? as i32,
                },
                Float => FloatInfo {
                    tag,
                    value: f32::from_bits(read_u4(reader)?),
                },
                Long => LongInfo {
                    tag,
                    value: read_u8(reader)? as i64,
                },
                Double => DoubleInfo {
                    tag,
                    value: f64::from_bits(read_u8(reader)?),
                },
                NameAndType => NameAndTypeInfo {
                    tag,
                    name_index: read_u2(reader)?,
                    descriptor_index: read_u2(reader)?,
                },
                Utf8 => {
                    let length = read_u2(reader)? as u64;
                    let bytes = read_bytes(length, reader)?;
                    Utf8Info {
                        tag,
                        value: str::from_utf8(bytes.as_slice()).unwrap().to_string(),
                    }
                }
                MethodHandle => MethodHandleInfo {
                    tag,
                    kind: read_u1(reader)?,
                    index: read_u2(reader)?,
                },
                MethodType => MethodTypeInfo {
                    tag,
                    descriptor_index: read_u2(reader)?,
                },
                Dynamic | InvokeDynamic => DynamicInfo {
                    tag,
                    bootstrap_method_attr_index: read_u2(reader)?,
                    name_and_type_index: read_u2(reader)?,
                },
                Module => ModuleInfo {
                    tag,
                    name_index: read_u2(reader)?,
                },
                Package => PackageInfo {
                    tag,
                    name_index: read_u2(reader)?,
                },
                _ => Empty,
            },
        };
        pool.push(constant);
        if tag == Double || tag == Long {
            // Skip next entry and insert placeholder, per spec
            skip = true;
        }
    }

    Ok(pool)
}

// TODO change this into a Result when I figure out error handling
pub fn resolve_utf8(index: usize, constant_pool: &Vec<Constant>) -> Option<String> {
    match &constant_pool[index - 1].info {
        ConstantInfo::Utf8Info { tag, value } => Some(value.to_string()),
        ConstantInfo::ClassInfo { tag, name_index } => {
            resolve_utf8(*name_index as usize, constant_pool)
        }
        a => {
            println!("{} {:?}", index, a);
            None
        } // TODO throw an actual error at some point
    }
}
