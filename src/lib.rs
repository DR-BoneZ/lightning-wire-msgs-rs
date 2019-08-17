use std::borrow::Borrow;
use std::io::Write;

mod watchtower;

fn write_varint<W: Write>(num: u64, w: &mut W) -> std::io::Result<usize> {
    match num {
        n if n < 0xfd => w.write(&[n as u8]),
        n if n < 0x10000 => {
            let mut count = 0;
            count += w.write(&[0xfd])?;
            count += w.write(&u16::to_le_bytes(n as u16))?;
            Ok(count)
        }
        n if n < 0x100000000 => {
            let mut count = 0;
            count += w.write(&[0xfe])?;
            count += w.write(&u32::to_le_bytes(n as u32))?;
            Ok(count)
        }
        n => {
            let mut count = 0;
            count += w.write(&[0xff])?;
            count += w.write(&u64::to_le_bytes(n))?;
            Ok(count)
        }
    }
}

pub trait WireMessage {
    fn msg_type(&self) -> u16;
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn WireItemBoxedWriter> + 'a>;

    fn write_to<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let mut count = 0;
        count += w.write(&u16::to_be_bytes(self.msg_type()))?;
        let mut boxed_w: Box<&mut dyn Write> = Box::new(w);
        for item in self.iter() {
            count += item.write_to_boxed(&mut boxed_w)?;
        }
        w.flush()?;
        Ok(count)
    }
}

pub trait WireItem {
    fn item_type(&self) -> u64;
    fn encode(&self) -> Box<[u8]>;

    fn write_to<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let mut count = 0;
        let data = self.encode();
        count += write_varint(self.item_type(), w)?;
        count += write_varint(data.len() as u64, w)?;
        count += w.write(data.borrow())?;
        Ok(count)
    }
}

pub trait WireItemBoxedWriter {
    fn write_to_boxed(&self, w: &mut Box<&mut dyn Write>) -> std::io::Result<usize>;
}

impl<WI> WireItemBoxedWriter for WI
where
    WI: WireItem,
{
    fn write_to_boxed(&self, w: &mut Box<&mut dyn Write>) -> std::io::Result<usize> {
        self.write_to(w)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
