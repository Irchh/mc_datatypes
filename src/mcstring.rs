use log::trace;
use crate::error::TypeError;
use crate::varint::VarInt;

#[derive(Debug)]
pub struct MCString {
    pub value: String,
    pub bytes: Vec<u8>
}

impl MCString {
    pub fn new(value: String) -> Result<Self, TypeError> {
        let mut string_bytes = value.as_bytes().to_vec();
        let mut length_bytes = VarInt::new(string_bytes.len() as i32).bytes;
        let mut bytes = vec![];
        bytes.append(&mut length_bytes);
        bytes.append(&mut string_bytes);
        Ok(Self {
            value,
            bytes,
        })
    }

    pub fn from(data: Vec<u8>) -> Result<Self, TypeError> {
        let length = VarInt::from(data.clone())?;
        trace!("MCString length: {}", length.value);
        trace!("MCString byte count: {}", length.bytes.len());
        let utf8 = data[length.bytes.len()..].iter().take(length.value as usize).map(|n| *n).collect::<Vec<u8>>();
        Ok(Self {
            value: String::from_utf8(utf8)?,
            bytes: vec![],
        })
    }
}
