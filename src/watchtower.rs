use crate as lightning_wire_msgs;
use lightning_wire_msgs::WireItem;
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(AnyWireMessage)]
pub enum AnyWatchtowerMessage {
    Init(Init),
}

#[derive(WireMessage)]
#[msg_type = 600]
pub struct Init {
    conn_features: RawFeatureVector,
    chain_hash: Hash,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
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

pub struct RawFeatureVector(BTreeSet<Feature>);
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
        let mut byte_idx = len as usize - 1;
        for _ in 0..len {
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
            byte_idx -= 1;
        }
        Ok(RawFeatureVector(ret))
    }
}

pub struct Hash([u8; 32]);
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
