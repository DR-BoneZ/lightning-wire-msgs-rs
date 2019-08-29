pub mod feature;
pub mod hash;

use crate::{TLVWireItem, WireItem};
use std::borrow::Borrow;
use std::io::{Read, Write};

impl WireItem for () {
    fn encode<W: Write>(&self, _: &mut W) -> std::io::Result<usize> {
        Ok(0)
    }

    fn decode<R: Read>(_: &mut R) -> std::io::Result<Self> {
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Buffer<T: Borrow<[u8]>> {
    Vector(Vec<u8>),
    Other(T),
}
impl<T> Buffer<T>
where
    T: Borrow<[u8]>,
{
    // Clones if not Vec variant
    pub fn to_vec(self) -> Vec<u8> {
        match self {
            Buffer::Vector(a) => a,
            Buffer::Other(a) => a.borrow().to_vec(),
        }
    }
}
impl<T> Borrow<[u8]> for Buffer<T>
where
    T: Borrow<[u8]>,
{
    fn borrow(&self) -> &[u8] {
        match self {
            Buffer::Vector(a) => a.borrow(),
            Buffer::Other(a) => a.borrow(),
        }
    }
}
impl<T> From<T> for Buffer<T>
where
    T: Borrow<[u8]>,
{
    fn from(t: T) -> Self {
        Buffer::Other(t)
    }
}
impl<T> WireItem for Buffer<T>
where
    T: Borrow<[u8]>,
{
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let mut count = 0;
        let slice: &[u8] = self.borrow();
        count += crate::write_varint(slice.len() as u64, w)?;
        count += w.write(slice)?;
        Ok(count)
    }

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        let len = crate::read_varint(r)? as usize;
        let mut buf = vec![0_u8; len];
        r.read_exact(&mut buf)?;
        Ok(Buffer::Vector(buf))
    }
}

#[derive(Clone, Debug)]
pub enum TLVBuffer<T: Borrow<[u8]>> {
    Vector(Vec<u8>),
    Other(T),
}
impl<T> Borrow<[u8]> for TLVBuffer<T>
where
    T: Borrow<[u8]>,
{
    fn borrow(&self) -> &[u8] {
        match self {
            TLVBuffer::Vector(a) => a.borrow(),
            TLVBuffer::Other(a) => a.borrow(),
        }
    }
}
impl<T> From<T> for TLVBuffer<T>
where
    T: Borrow<[u8]>,
{
    fn from(t: T) -> Self {
        TLVBuffer::Other(t)
    }
}
impl<T> TLVWireItem for TLVBuffer<T>
where
    T: Borrow<[u8]>,
{
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        w.write(self.borrow())
    }

    fn decode<R: Read>(r: &mut R, len: usize) -> std::io::Result<Self> {
        let mut buf = vec![0_u8; len];
        r.read_exact(&mut buf)?;
        Ok(TLVBuffer::Vector(buf))
    }
}
