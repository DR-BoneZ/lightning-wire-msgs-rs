use crate::{AnyWireMessage, EncodedItem, WireItem, WireMessage};
use std::collections::BTreeSet;
use std::io::Write;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
}

pub struct Hash([u8; 32]);
impl WireItem for Hash {
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let count = w.write(&self.0)?;
        w.flush()?;
        Ok(count)
    }
}
