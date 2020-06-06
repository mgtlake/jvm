use crate::read::*;
use crate::constants::*;

use std::io::{Read, Result};
use std::fs::{File};

#[derive(Debug)]
struct Class {
    constant_pool: Vec<Constant>,
    // TODO
}

fn parse_interfaces<'a>(reader: &'a mut dyn Read, constant_pool: &'a Vec<Constant>) -> Result<Vec<&'a Constant>> {
    let mut interfaces = Vec::new();
    let interfaces_count = read_u2(reader)?;

    for _ in 0..interfaces_count {
        interfaces.push(&constant_pool[read_u2(reader)? as usize]);
    }

    Ok(interfaces)
}

pub fn parse_class() -> Result<()> {
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
    println!("{:?}", constant_pool);

    let access_flags = read_u2(reader)?; // TODO parse

    let this_class = &constant_pool[read_u2(reader)? as usize];
    let super_class = &constant_pool[read_u2(reader)? as usize];

    let interfaces = parse_interfaces(reader, &constant_pool)?;
    println!("{:?}", interfaces);

    Ok(())
}