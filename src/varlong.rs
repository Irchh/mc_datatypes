use crate::TypeError;

#[derive(Debug)]
pub struct VarLong {
    pub value: i64,
    pub bytes: Vec<u8>
}

impl VarLong {
    pub fn new(value: i64) -> Self {
        let mut num = value as u64;
        let mut data = vec![];
        loop {
            if num >= 0x80u64 {
                let b = num as u8 & (!0x80);
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
            value |= (byte as i64&0x7F)<<shift;
            if byte&0x80 == 0 {
                break;
            }
            shift += 7;
            if shift >= 64 {
                return Err(TypeError::VarIntTooBig);
            }
        }
        Ok(Self {
            value,
            bytes,
        })
    }
}