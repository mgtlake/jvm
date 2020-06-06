use std::io::{Error, ErrorKind, Read, Result};

pub fn read_u1(reader: &mut dyn Read) -> Result<u8> {
    let mut buffer = [0; 1];
    reader.read_exact(&mut buffer)?;
    Ok(u8::from_be_bytes(buffer))
}

pub fn read_u2(reader: &mut dyn Read) -> Result<u16> {
    let mut buffer = [0; 2];
    reader.read_exact(&mut buffer)?;
    Ok(u16::from_be_bytes(buffer))
}

pub fn read_u4(reader: &mut dyn Read) -> Result<u32> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(u32::from_be_bytes(buffer))
}

pub fn read_u8(reader: &mut dyn Read) -> Result<u64> {
    let mut buffer = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(u64::from_be_bytes(buffer))
}

pub fn read_bytes(count: u64, reader: &mut dyn Read) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let read_count = reader.take(count).read_to_end(&mut buffer)?;
    if read_count != count as usize {
        return Err(Error::from(ErrorKind::NotFound));
    }
    Ok(buffer)
}
