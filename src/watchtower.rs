use crate::{EncodedItem, WireItem, WireItemBoxedWriter, WireMessage};
use std::collections::HashSet;

#[derive(WireMessage)]
#[msg_type = 600]
pub struct Init {
    conn_features: RawFeatureVector,
    chain_hash: Hash,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Feature {
    DataLossProtectRequired, // 0
    DataLossProtectOptional, // 1
    InitialRoutingSync,      // 3
    GossipQueriesRequired,   // 6
    GossipQueriesOptional,   // 7
}
impl Feature {
    pub fn idx(&self) -> usize {
        use Feature::*;
        match self {
            DataLossProtectRequired => 0,
            DataLossProtectOptional => 1,
            InitialRoutingSync => 3,
            GossipQueriesRequired => 6,
            GossipQueriesOptional => 7,
        }
    }
}

pub struct RawFeatureVector(HashSet<Feature>);
impl WireItem for RawFeatureVector {
    fn encode(&self) -> Box<[u8]> {
        Vec::new().into_boxed_slice()
    }
}

pub struct Hash;
impl WireItem for Hash {
    fn encode(&self) -> Box<[u8]> {
        Vec::new().into_boxed_slice()
    }
}
