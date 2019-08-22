#![feature(never_type)]

#[macro_use]
extern crate lightning_wire_msgs_derive;

#[macro_use]
extern crate num_enum;

use std::io::{Read, Write};

pub mod watchtower;

fn write_varint<W: Write>(num: u64, w: &mut W) -> std::io::Result<usize> {
    match num {
        n if n < 0xfd => w.write(&[n as u8]),
        n if n < 0x10000 => {
            let mut count = 0;
            count += w.write(&[0xfd])?;
            count += w.write(&u16::to_be_bytes(n as u16))?;
            Ok(count)
        }
        n if n < 0x100000000 => {
            let mut count = 0;
            count += w.write(&[0xfe])?;
            count += w.write(&u32::to_be_bytes(n as u32))?;
            Ok(count)
        }
        n => {
            let mut count = 0;
            count += w.write(&[0xff])?;
            count += w.write(&u64::to_be_bytes(n))?;
            Ok(count)
        }
    }
}

fn read_varint<R: Read>(r: &mut R) -> std::io::Result<u64> {
    let mut b = [0_u8];
    r.read_exact(&mut b)?;
    Ok(match b[0] {
        0xff => {
            let mut b = [0_u8; 8];
            r.read_exact(&mut b)?;
            u64::from_be_bytes(b)
        }
        0xfe => {
            let mut b = [0_u8; 4];
            r.read_exact(&mut b)?;
            u32::from_be_bytes(b) as u64
        }
        0xfd => {
            let mut b = [0_u8; 2];
            r.read_exact(&mut b)?;
            u16::from_be_bytes(b) as u64
        }
        n => n as u64,
    })
}

fn peek_varint<'a, R: Read>(r: &mut PeekReader<'a, R>) -> std::io::Result<u64> {
    let mut b = [0_u8];
    r.peek_exact(&mut b)?;
    Ok(match b[0] {
        0xff => {
            let mut b = [0_u8; 8];
            r.peek_exact(&mut b)?;
            u64::from_be_bytes(b)
        }
        0xfe => {
            let mut b = [0_u8; 4];
            r.peek_exact(&mut b)?;
            u32::from_be_bytes(b) as u64
        }
        0xfd => {
            let mut b = [0_u8; 2];
            r.peek_exact(&mut b)?;
            u16::from_be_bytes(b) as u64
        }
        n => n as u64,
    })
}

pub enum EncodedItem<T> {
    Expected(T),
    TLV(Option<T>, u64),
}
impl<'a, T, U> From<(&'a Option<T>, u64)> for EncodedItem<U>
where
    U: From<&'a T>,
{
    fn from(tup: (&'a Option<T>, u64)) -> Self {
        EncodedItem::TLV(tup.0.as_ref().map(U::from), tup.1)
    }
}
impl<'a, T, U> From<(&'a T,)> for EncodedItem<U>
where
    U: From<&'a T>,
{
    fn from(tup: (&'a T,)) -> Self {
        EncodedItem::Expected(U::from(tup.0))
    }
}

pub trait AnyWireMessage<'a>
where
    Self: Sized,
{
    fn msg_type(&self) -> u16;

    fn write_to<W: Write>(&'a self, w: &mut W) -> std::io::Result<usize>;

    fn read_from<R: Read>(r: &mut R) -> std::io::Result<Self>;
}

impl<'a, T> AnyWireMessage<'a> for T
where
    T: WireMessage<'a>,
    &'a T: IntoIterator<Item = EncodedItem<T::Item>> + 'a,
{
    fn msg_type(&self) -> u16 {
        T::MSG_TYPE
    }

    fn write_to<W: Write>(&'a self, w: &mut W) -> std::io::Result<usize> {
        self.write_to(w)
    }

    fn read_from<R: Read>(r: &mut R) -> std::io::Result<Self> {
        T::read_from(r, true)
    }
}

pub trait WireMessage<'a>
where
    &'a Self: IntoIterator<Item = EncodedItem<Self::Item>> + 'a,
    Self: Sized,
{
    type Item: WireItemWriter;

    const MSG_TYPE: u16;

    fn write_to<W: Write>(&'a self, w: &mut W) -> std::io::Result<usize> {
        let mut count = 0;
        count += w.write(&u16::to_be_bytes(Self::MSG_TYPE))?;
        let mut boxed_w: Box<&mut dyn Write> = Box::new(w);
        for item in (&self).into_iter() {
            match item {
                EncodedItem::Expected(t) => {
                    count += t.write_to(&mut boxed_w, None)?;
                }
                EncodedItem::TLV(Some(t), tlv_type) => {
                    count += t.write_to(&mut boxed_w, Some(tlv_type))?;
                }
                _ => (),
            }
        }
        w.flush()?;
        Ok(count)
    }

    fn read_from<R: Read>(r: &mut R, check_type: bool) -> std::io::Result<Self>;
}

pub struct PeekReader<'a, R: Read> {
    peeked: Vec<u8>,
    reader: &'a mut R,
}
impl<'a, R: Read> From<&'a mut R> for PeekReader<'a, R> {
    fn from(r: &'a mut R) -> Self {
        PeekReader {
            peeked: Vec::new(),
            reader: r,
        }
    }
}
impl<'a, R: Read> PeekReader<'a, R> {
    pub fn peek_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.reader.read_exact(buf)?;
        self.peeked.extend_from_slice(buf);
        Ok(())
    }

    pub fn flush_peeked(&mut self) -> () {
        self.peeked.truncate(0);
    }
}
impl<'a, R: Read> Read for PeekReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let expected = buf.len();
        let mut count = 0;
        let mut buf_slice = buf;
        if !self.peeked.is_empty() {
            count += self.peeked.as_slice().read(buf_slice)?;
            self.peeked.drain(0..count);
            if count < expected {
                buf_slice = &mut buf_slice[count..];
            }
        }
        count += self.reader.read(buf_slice)?;
        Ok(count)
    }
}

pub trait WireItem
where
    Self: Sized,
{
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize>;

    fn write_to<W: Write>(&self, w: &mut W, tlv_type: Option<u64>) -> std::io::Result<usize> {
        let mut count = 0;
        match tlv_type {
            Some(t) => {
                let mut data = Vec::new();
                self.encode(&mut data)?;
                count += write_varint(t, w)?;
                count += write_varint(data.len() as u64, w)?;
                count += w.write(&data)?;
            }
            None => {
                count += self.encode(w)?;
            }
        }
        Ok(count)
    }

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self>;

    fn read_from<'a, R: Read>(
        reader: &mut PeekReader<'a, R>,
        tlv_type: Option<u64>,
    ) -> std::io::Result<Option<Self>> {
        if let Some(tlv_type) = tlv_type {
            loop {
                use std::cmp::Ordering::*;

                let t = peek_varint(reader)?;
                match t.cmp(&tlv_type) {
                    Greater => return Ok(None),
                    Equal => {
                        reader.flush_peeked();
                        read_varint(reader)?;
                        break;
                    }
                    Less => {
                        reader.flush_peeked();
                        let skip = read_varint(reader)? as usize;
                        reader.read_exact(&mut vec![0_u8; skip])?;
                    }
                }
            }
        }
        Ok(Some(Self::decode(reader)?))
    }
}

pub trait WireItemWriter {
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize>;

    fn write_to<W: Write>(&self, w: &mut W, tlv_type: Option<u64>) -> std::io::Result<usize> {
        let mut count = 0;
        match tlv_type {
            Some(t) => {
                let mut data = Vec::new();
                self.encode(&mut data)?;
                count += write_varint(t, w)?;
                count += write_varint(data.len() as u64, w)?;
                count += w.write(&data)?;
            }
            None => {
                count += self.encode(w)?;
            }
        }
        Ok(count)
    }
}

impl<T> WireItemWriter for T
where
    T: WireItem,
{
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        self.encode(w)
    }
}
