use crate::WireItem;
use std::io::{Read, Write};

#[derive(Clone, Debug)]
pub struct SatPerKWeight(pub i64);
impl WireItem for SatPerKWeight {
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        self.0.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        i64::decode(r).map(SatPerKWeight)
    }
}
