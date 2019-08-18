use crate::{WireItem, WireItemBoxedWriter, WireMessage};

#[derive(WireMessage)]
#[msg_type = 600]
pub struct Init {
    conn_features: RawFeatureVector,
    chain_hash: Hash,
}

pub struct RawFeatureVector;
impl WireItem for RawFeatureVector {
    fn item_type(&self) -> u64 {
        0
    }
    fn encode(&self) -> Box<[u8]> {
        Vec::new().into_boxed_slice()
    }
}

pub struct Hash;
impl WireItem for Hash {
    fn item_type(&self) -> u64 {
        0
    }
    fn encode(&self) -> Box<[u8]> {
        Vec::new().into_boxed_slice()
    }
}
