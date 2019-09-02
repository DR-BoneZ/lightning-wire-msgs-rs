use crate::WireItem;
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TryFromPrimitive)]
#[repr(usize)]
pub enum Feature {
    DataLossProtectRequired = 0,
    DataLossProtectOptional = 1,
    InitialRoutingSync = 3,
    GossipQueriesRequired = 6,
    GossipQueriesOptional = 7,
}
impl Feature {
    pub fn idx(&self) -> usize {
        *self as usize
    }
}

#[derive(Clone, Debug)]
pub struct RawFeatureVector(pub BTreeSet<Feature>);
impl RawFeatureVector {
    pub fn new() -> Self {
        RawFeatureVector(BTreeSet::new())
    }
    pub fn add(&mut self, f: Feature) -> bool {
        self.0.insert(f)
    }
}
impl WireItem for RawFeatureVector {
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let len = self
            .0
            .iter()
            .next_back()
            .map(|a| a.idx() / 8 + 1)
            .unwrap_or(0) as u16;
        let mut count = w.write(&u16::to_be_bytes(len))?;
        let mut feat_iter = self.0.iter();
        let mut current = feat_iter.next_back();
        let mut byte = 0_u8;
        let mut byte_idx = len as usize - 1;
        while let Some(feat) = current {
            let new_byte_idx = feat.idx() / 8;
            while byte_idx > new_byte_idx {
                count += w.write(&[byte])?;
                byte = 0;
                byte_idx -= 1;
            }
            let bit_idx = feat.idx() % 8;
            byte |= 1 << bit_idx;
            current = feat_iter.next_back()
        }
        count += w.write(&[byte])?;
        w.flush()?;

        Ok(count)
    }

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        let mut len = [0_u8; 2];
        r.read_exact(&mut len)?;
        let len = u16::from_be_bytes(len);
        let mut ret = BTreeSet::new();
        if len == 0 {
            return Ok(RawFeatureVector(ret));
        }
        let mut byte_idx = len as usize;
        for _ in 0..len {
            byte_idx -= 1;
            let mut byte = [0_u8];
            r.read_exact(&mut byte)?;
            for bit_idx in 0..8 {
                if byte[0] & (1 << bit_idx) != 0 {
                    let feat = 8 * byte_idx + bit_idx;
                    ret.insert(
                        Feature::try_from(feat).map_err(|_| std::io::ErrorKind::InvalidData)?,
                    );
                }
            }
        }
        Ok(RawFeatureVector(ret))
    }
}
