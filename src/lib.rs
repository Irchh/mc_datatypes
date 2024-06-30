mod varint;
mod mcstring;
mod error;
mod varlong;
mod blockpos;

pub use error::*;
pub use varint::*;
pub use varlong::*;
pub use mcstring::*;
pub use blockpos::*;

#[cfg(test)]
mod tests {
    use crate::VarInt;

    #[test]
    fn negative_varint() {
        let varint = VarInt::new(-1).bytes;
        assert_eq!(varint, vec![0xff, 0xff, 0xff, 0xff, 0x0f])
    }
}