#[macro_use]
extern crate lightning_wire_msgs_derive;

use std::io::{Read, Write};

pub mod items;
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

pub trait AnyWireMessageWriter {
    fn msg_type(&self) -> u16;

    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize>;
}

pub trait AnyWireMessageReader
where
    Self: Sized,
{
    fn msg_type(&self) -> u16;

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self>;
}

pub trait AnyWireMessage
where
    Self: Sized,
{
    fn msg_type(&self) -> u16;

    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize>;

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self>;
}
impl<T> AnyWireMessageWriter for T
where
    T: AnyWireMessage,
{
    fn msg_type(&self) -> u16 {
        self.msg_type()
    }

    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        self.encode(w)
    }
}
impl<T> AnyWireMessageReader for T
where
    T: AnyWireMessage,
{
    fn msg_type(&self) -> u16 {
        self.msg_type()
    }

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        Self::decode(r)
    }
}

impl<T> AnyWireMessage for T
where
    T: WireMessage,
{
    fn msg_type(&self) -> u16 {
        T::MSG_TYPE
    }

    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        self.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        T::decode(r, true)
    }
}

pub trait WireMessageWriter {
    const MSG_TYPE: u16;
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize>;
}

pub trait WireMessageReader
where
    Self: Sized,
{
    const MSG_TYPE: u16;
    fn decode<R: Read>(r: &mut R, check_type: bool) -> std::io::Result<Self>;
}

pub trait WireMessage
where
    Self: Sized,
{
    const MSG_TYPE: u16;

    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize>;

    fn decode<R: Read>(r: &mut R, check_type: bool) -> std::io::Result<Self>;
}
impl<T> WireMessageWriter for T
where
    T: WireMessage,
{
    const MSG_TYPE: u16 = T::MSG_TYPE;

    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        self.encode(w)
    }
}
impl<T> WireMessageReader for T
where
    T: WireMessage,
{
    const MSG_TYPE: u16 = T::MSG_TYPE;
    fn decode<R: Read>(r: &mut R, check_type: bool) -> std::io::Result<Self> {
        Self::decode(r, check_type)
    }
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

pub trait WireItemWriter {
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize>;
}

pub trait WireItemReader
where
    Self: Sized,
{
    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self>;
}

pub trait WireItem
where
    Self: Sized,
{
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize>;

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self>;
}
impl<T> WireItemReader for T
where
    T: WireItem,
{
    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        T::decode(r)
    }
}

pub trait TLVWireItemWriter {
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize>;

    fn encode_tlv<W: Write>(&self, w: &mut W, tlv_type: u64) -> std::io::Result<usize> {
        let mut count = 0;
        let mut data = Vec::new();
        self.encode(&mut data)?;
        count += write_varint(tlv_type, w)?;
        count += write_varint(data.len() as u64, w)?;
        count += w.write(&data)?;
        Ok(count)
    }
}

pub trait TLVWireItemReader
where
    Self: Sized,
{
    fn decode<R: Read>(r: &mut R, len: usize) -> std::io::Result<Self>;

    fn decode_tlv<'a, R: Read>(
        reader: &mut PeekReader<'a, R>,
        tlv_type: u64,
    ) -> std::io::Result<Option<Self>> {
        loop {
            use std::cmp::Ordering::*;

            let t = match peek_varint(reader) {
                Ok(t) => t,
                Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
                Err(e) => return Err(e),
            };
            match t.cmp(&tlv_type) {
                Greater => return Ok(None),
                Equal => {
                    reader.flush_peeked();
                    let len = read_varint(reader)? as usize;
                    return Ok(Some(Self::decode(reader, len)?));
                }
                Less => {
                    reader.flush_peeked();
                    let skip = read_varint(reader)? as usize;
                    reader.read_exact(&mut vec![0_u8; skip])?;
                }
            }
        }
    }
}

pub trait TLVWireItem
where
    Self: Sized,
{
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize>;

    fn decode<R: Read>(r: &mut R, len: usize) -> std::io::Result<Self>;
}
impl<T> TLVWireItemWriter for T
where
    T: TLVWireItem,
{
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        self.encode(w)
    }
}
impl<T> TLVWireItemReader for T
where
    T: TLVWireItem,
{
    fn decode<R: Read>(r: &mut R, len: usize) -> std::io::Result<Self> {
        T::decode(r, len)
    }
}

impl<T> TLVWireItem for T
where
    T: WireItem,
{
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        self.encode(w)
    }

    fn decode<R: Read>(r: &mut R, _: usize) -> std::io::Result<Self> {
        T::decode(r)
    }
}
impl<T> WireItemWriter for T
where
    T: TLVWireItem,
{
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        self.encode(w)
    }
}

#[test]
fn bench() {
    use watchtower::messages::Init;
    let mut features = items::feature::RawFeatureVector::new();
    features.add(items::feature::Feature::DataLossProtectRequired);
    features.add(items::feature::Feature::GossipQueriesRequired);
    features.add(items::feature::Feature::InitialRoutingSync);
    let mut init = Init {
        conn_features: features,
        chain_hash: items::hash::Hash([0; 32]),
    };
    let mut expected = Vec::new();
    <Init as WireMessageWriter>::encode(&init, &mut expected).expect("encode");
    let now = std::time::Instant::now();
    let mut buf = Vec::new();
    for _ in 0..1_000_000 {
        buf.truncate(0);
        <Init as WireMessageWriter>::encode(&init, &mut buf).expect("encode");
        assert!(&buf == &expected);
        init = WireMessageReader::decode(&mut std::io::Cursor::new(&buf), true).expect("decode");
    }
    println!("{:?}", now.elapsed());
}
