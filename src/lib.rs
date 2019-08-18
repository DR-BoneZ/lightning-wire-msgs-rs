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
impl<'a, T> From<(Option<&'a T>, u64)> for EncodedItem<&'a dyn WireItemBoxedWriter>
where
    T: WireItemBoxedWriter,
{
    fn from(tup: (Option<&'a T>, u64)) -> Self {
        EncodedItem::TLV(tup.0.map(|t| t as &dyn WireItemBoxedWriter), tup.1)
    }
}
impl<'a, T> From<(&'a T,)> for EncodedItem<&'a dyn WireItemBoxedWriter>
where
    T: WireItemBoxedWriter,
{
    fn from(tup: (&'a T,)) -> Self {
        EncodedItem::Expected(tup.0 as &'a dyn WireItemBoxedWriter)
    }
}

pub trait WireMessage<'a>
where
    &'a Self: IntoIterator<Item = EncodedItem<&'a dyn WireItemBoxedWriter>> + 'a,
{
    fn msg_type(&self) -> u16;

    fn write_to<W: Write>(&'a self, w: &mut W) -> std::io::Result<usize> {
        let mut count = 0;
        count += w.write(&u16::to_be_bytes(self.msg_type()))?;
        let mut boxed_w: Box<&mut dyn Write> = Box::new(w);
        for item in (&self).into_iter() {
            match item {
                EncodedItem::Expected(t) => {
                    count += t.write_to_boxed(&mut boxed_w, None)?;
                }
                EncodedItem::TLV(Some(t), tlv_type) => {
                    count += t.write_to_boxed(&mut boxed_w, Some(tlv_type))?;
                }
                _ => (),
            }
        }
        w.flush()?;
        Ok(count)
    }
}

pub trait WireItem {
    fn encode(&self) -> Box<[u8]>;

    fn write_to<W: Write>(&self, w: &mut W, tlv_type: Option<u64>) -> std::io::Result<usize> {
        let mut count = 0;
        let data = self.encode();
        match tlv_type {
            Some(t) => {
                count += write_varint(t, w)?;
                count += write_varint(data.len() as u64, w)?;
            }
            None => (),
        }
        count += w.write(&data)?;
        Ok(count)
    }
}

pub trait WireItemBoxedWriter {
    fn write_to_boxed(
        &self,
        w: &mut Box<&mut dyn Write>,
        tlv_type: Option<u64>,
    ) -> std::io::Result<usize>;
}

impl<WI> WireItemBoxedWriter for WI
where
    WI: WireItem,
{
    fn write_to_boxed(
        &self,
        w: &mut Box<&mut dyn Write>,
        tlv_type: Option<u64>,
    ) -> std::io::Result<usize> {
        self.write_to(w, tlv_type)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
