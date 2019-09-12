use crate::WireItem;
use std::io::{Read, Write};
pub type Sats = i64;
#[derive(Clone, Copy, Debug)]
pub struct SatPerKWeight(pub Sats);
impl SatPerKWeight {
    pub fn fee_for_weight(&self, wu: i64) -> Sats {
        self.0 * wu / 1000
    }
}
impl WireItem for SatPerKWeight {
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        self.0.encode(w)
    }
    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        i64::decode(r).map(SatPerKWeight)
    }
}
