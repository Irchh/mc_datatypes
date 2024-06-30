use crate::error::TypeError;

#[derive(Debug)]
pub struct VarInt {
    pub value: i32,
    pub bytes: Vec<u8>
}

impl VarInt {
    pub fn new(value: i32) -> Self {
        let mut num = value as u32;
        let mut data = vec![];
        loop {
            if num >= 0x80u32 {
                let b = num as u8 & 0x7F;
                data.push(b|0x80);
                num = num >> 7;
            } else {
                data.push(num as u8);
                break;
            }
        }
        Self {
            value,
            bytes: data,
        }
    }

    pub fn from(data: Vec<u8>) -> Result<Self, TypeError> {
        let mut iterator = data.iter();
        let mut bytes = vec![];
        let mut value = 0;
        let mut shift = 0;
        loop {
            let byte = *iterator.next().ok_or(TypeError::EndOfData)?;
            bytes.push(byte);
            value |= (byte as i32&0x7F)<<shift;
            if byte&0x80 == 0 {
                break;
            }
            shift += 7;
            if shift >= 32 {
                return Err(TypeError::VarIntTooBig);
            }
        }
        Ok(Self {
            value,
            bytes,
        })
    }
}