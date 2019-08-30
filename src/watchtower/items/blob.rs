use crate::WireItem;
use std::io::{Read, Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum Flag {
    FlagReward = 0,
    FlagCommitOutputs = 1,
}
impl Flag {
    pub const fn flag(&self) -> u16 {
        1 << *self as u16
    }
}

#[derive(Clone, Debug)]
#[repr(u16)]
pub enum Type {
    TypeAltruistCommit = Flag::FlagCommitOutputs.flag(),
    TypeRewardCommit = Flag::FlagCommitOutputs.flag() | Flag::FlagReward.flag(),
}
impl WireItem for Type {
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        unimplemented!()
    }

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        unimplemented!()
    }
}
