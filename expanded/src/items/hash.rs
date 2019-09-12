use crate::WireItem;
use std::io::{Read, Write};
#[derive(Clone, Debug)]
pub struct Hash(pub [u8; 32]);
impl WireItem for Hash {
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let count = w.write(&self.0)?;
        w.flush()?;
        Ok(count)
    }
    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        let mut hash = [0_u8; 32];
        r.read_exact(&mut hash)?;
        Ok(Hash(hash))
    }
}
