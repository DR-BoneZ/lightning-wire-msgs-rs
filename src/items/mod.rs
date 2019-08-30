pub mod feature;
pub mod hash;

use crate::{TLVWireItem, WireItem};
use std::borrow::Borrow;
use std::io::{Read, Write};

macro_rules! impl_wire_item_for_nums {
    (
        $(
            $(#[$attr:meta])*
            $num_ty:ty[$bytes:literal],
        )*
    ) => {
        $(
            $(#[$attr])*
            impl WireItem for $num_ty {
                fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
                    w.write(&<$num_ty>::to_be_bytes(*self))
                }

                fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
                   let mut buf = [0_u8; $bytes];
                   r.read_exact(&mut buf)?;
                    Ok(<$num_ty>::from_be_bytes(buf))
                }
            }
        )*
    };
}

impl_wire_item_for_nums!(
    u8[1],
    i8[1],
    u16[2],
    i16[2],
    u32[4],
    i32[4],
    u64[8],
    i64[8],
    u128[16],
    i128[16],
    #[cfg(target_pointer_width = "16")]
    usize[2],
    #[cfg(target_pointer_width = "16")]
    isize[2],
    #[cfg(target_pointer_width = "32")]
    usize[4],
    #[cfg(target_pointer_width = "32")]
    isize[4],
    #[cfg(target_pointer_width = "64")]
    usize[8],
    #[cfg(target_pointer_width = "64")]
    isize[8],
);

pub enum MaybeOwned<'a, O: Borrow<B>, B> {
    Owned(O),
    Borrowed(&'a B),
}
impl<'a, O, B> Borrow<B> for MaybeOwned<'a, O, B>
where
    O: Borrow<B>,
{
    fn borrow(&self) -> &B {
        match self {
            MaybeOwned::Owned(o) => o.borrow(),
            MaybeOwned::Borrowed(b) => b,
        }
    }
}
impl<'a, O, B> WireItem for MaybeOwned<'a, O, B>
where
    O: Borrow<B> + WireItem,
    B: crate::WireItemWriter,
{
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        Borrow::<B>::borrow(self).encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        Ok(MaybeOwned::Owned(O::decode(r)?))
    }
}

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
