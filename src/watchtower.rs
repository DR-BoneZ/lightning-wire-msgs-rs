use crate::{WireItem, WireItemBoxedWriter, WireMessage};

struct InitIter<'a> {
    idx: usize,
    parent: &'a Init,
}
impl<'a> From<&'a Init> for InitIter<'a> {
    fn from(a: &'a Init) -> Self {
        InitIter { idx: 0, parent: a }
    }
}
impl<'a> Iterator for InitIter<'a> {
    type Item = &'a dyn WireItemBoxedWriter;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.idx;
        self.idx += 1;
        match n {
            0 => Some(&self.parent.conn_features),
            1 => Some(&self.parent.chain_hash),
            _ => None,
        }
    }
}

pub struct Init {
    conn_features: RawFeatureVector,
    chain_hash: Hash,
}
impl WireMessage for Init {
    fn msg_type(&self) -> u16 {
        600
    }
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn WireItemBoxedWriter> + 'a> {
        Box::new(InitIter::from(self))
    }
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
