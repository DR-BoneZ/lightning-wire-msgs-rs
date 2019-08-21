#![feature(never_type)]

#[macro_use]
extern crate lightning_wire_msgs_derive;

use std::io::Write;

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

pub trait AnyWireMessage<'a> {
    fn msg_type(&self) -> u16;

    fn write_to<W: Write>(&'a self, w: &mut W) -> std::io::Result<usize>;
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
}

pub trait WireMessage<'a>
where
    &'a Self: IntoIterator<Item = EncodedItem<Self::Item>> + 'a,
{
    type Item: WireItem;

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
}

pub trait WireItem {
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
