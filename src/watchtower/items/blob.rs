use crate::WireItem;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, TryFromPrimitive)]
#[repr(u16)]
pub enum Flag {
    Reward = 0,
    CommitOutputs = 1,
}
impl Flag {
    pub const fn flag(&self) -> u16 {
        1 << *self as u16
    }
    pub const fn in_bitvec(&self, bitvec: u16) -> bool {
        bitvec & self.flag() != 0
    }
    pub const fn all() -> [Flag; 2] {
        use Flag::*;
        [Reward, CommitOutputs]
    }
}
impl std::fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Flag{:?}", self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Type(pub u16);
impl Type {
    pub const fn has(&self, f: Flag) -> bool {
        f.in_bitvec(self.0)
    }
    pub const fn is(&self, t: KnownType) -> bool {
        self.0 == t as u16
    }
    pub fn has_unknown_flags(&self) -> bool {
        std::iter::successors(Some(15), |i| if *i > 0 { Some(i - 1) } else { None })
            .filter(|i: &u16| Flag::try_from(*i).is_err())
            .map(|i| 1 << i)
            .filter(|i| self.0 & i == 0)
            .next()
            .is_some()
    }
}
impl<T> From<T> for Type
where
    u16: From<T>,
{
    fn from(t: T) -> Self {
        Type(u16::from(t))
    }
}
impl std::iter::FromIterator<Flag> for Type {
    fn from_iter<I: IntoIterator<Item = Flag>>(iter: I) -> Self {
        Type(iter.into_iter().fold(0_u16, |acc, x| acc & x.flag()))
    }
}
impl WireItem for Type {
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        self.0.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        u16::decode(r).map(Type)
    }
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_unknown_flags() {
            write!(f, "{:016b}", self.0)?;
        }
        write!(f, "[")?;
        let mut first = true;
        for flag in Flag::all().iter() {
            if !first {
                write!(f, "|")?;
            } else {
                first = false;
            }
            if self.has(*flag) {
                write!(f, "{}", flag)?;
            } else {
                write!(f, "No-{}", flag)?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum KnownType {
    TypeAltruistCommit = Flag::CommitOutputs.flag(),
    TypeRewardCommit = Flag::CommitOutputs.flag() | Flag::Reward.flag(),
}
impl From<KnownType> for u16 {
    fn from(t: KnownType) -> Self {
        t as u16
    }
}
impl WireItem for KnownType {
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        (*self as u16).encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        KnownType::try_from(u16::decode(r)?).map_err(|_| std::io::ErrorKind::InvalidData.into())
    }
}
impl TryFrom<Type> for KnownType {
    type Error = Type;

    fn try_from(t: Type) -> Result<Self, Type> {
        KnownType::try_from(t.0).map_err(Type)
    }
}
