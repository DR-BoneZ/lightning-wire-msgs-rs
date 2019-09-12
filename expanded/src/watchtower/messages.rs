use super::items::{
    blob::Type as BlobType, error::CreateSessionError, error::DeleteSessionError, error::ErrorCode,
    error::StateUpdateError,
};
use crate as lightning_wire_msgs;
use crate::items::{feature::RawFeatureVector, fees::SatPerKWeight, hash::Hash, Buffer};
use std::borrow::Borrow;
pub enum AnyWatchtowerMessage<T: Borrow<[u8]>> {
    Init(Init),
    Error(Error<T>),
}
#[doc = "automatically generated"]
impl<T: Borrow<[u8]>> lightning_wire_msgs::AnyWireMessage for AnyWatchtowerMessage<T> {
    fn msg_type(&self) -> u16 {
        match self {
            AnyWatchtowerMessage::Init(_) => <Init as lightning_wire_msgs::WireMessage>::MSG_TYPE,
            AnyWatchtowerMessage::Error(_) => {
                <Error<T> as lightning_wire_msgs::WireMessage>::MSG_TYPE
            }
        }
    }
    fn encode<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
        match self {
            AnyWatchtowerMessage::Init(a) => lightning_wire_msgs::WireMessageWriter::encode(a, w),
            AnyWatchtowerMessage::Error(a) => lightning_wire_msgs::WireMessageWriter::encode(a, w),
        }
    }
    fn decode<R: std::io::Read>(r: &mut R) -> std::io::Result<Self> {
        let mut msg_type = [0_u8; 2];
        r.read_exact(&mut msg_type)?;
        let msg_type = u16::from_be_bytes(msg_type);
        Ok(match msg_type {
            <Init as lightning_wire_msgs::WireMessageReader>::MSG_TYPE => {
                AnyWatchtowerMessage::Init(
                    <Init as lightning_wire_msgs::WireMessageReader>::decode(r, false)?,
                )
            }
            <Error<T> as lightning_wire_msgs::WireMessageReader>::MSG_TYPE => {
                AnyWatchtowerMessage::Error(
                    <Error<T> as lightning_wire_msgs::WireMessageReader>::decode(r, false)?,
                )
            }
            _ => return Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        })
    }
}
#[derive(Clone, Debug)]
pub struct Init {
    pub conn_features: RawFeatureVector,
    pub chain_hash: Hash,
}
#[doc = "automatically generated"]
impl lightning_wire_msgs::WireMessage for Init {
    const MSG_TYPE: u16 = 600;
    fn encode<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let mut count = 0;
        count += w.write(&u16::to_be_bytes(Self::MSG_TYPE))?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.conn_features, w)?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.chain_hash, w)?;
        w.flush()?;
        Ok(count)
    }
    fn decode<R: std::io::Read>(reader: &mut R, check_type: bool) -> std::io::Result<Self> {
        if check_type {
            let mut msg_type = [0_u8; 2];
            reader.read_exact(&mut msg_type)?;
            let msg_type = u16::from_be_bytes(msg_type);
            if msg_type != Self::MSG_TYPE {
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }
        }
        let mut peek_reader = lightning_wire_msgs::PeekReader::from(reader);
        Ok(Init {
            conn_features: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
            chain_hash: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
        })
    }
}
#[derive(Clone, Debug)]
pub struct Error<T: Borrow<[u8]>> {
    pub code: ErrorCode,
    pub data: Buffer<T>,
}
#[doc = "automatically generated"]
impl<T: Borrow<[u8]>> lightning_wire_msgs::WireMessage for Error<T> {
    const MSG_TYPE: u16 = 601;
    fn encode<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let mut count = 0;
        count += w.write(&u16::to_be_bytes(Self::MSG_TYPE))?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.code, w)?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.data, w)?;
        w.flush()?;
        Ok(count)
    }
    fn decode<R: std::io::Read>(reader: &mut R, check_type: bool) -> std::io::Result<Self> {
        if check_type {
            let mut msg_type = [0_u8; 2];
            reader.read_exact(&mut msg_type)?;
            let msg_type = u16::from_be_bytes(msg_type);
            if msg_type != Self::MSG_TYPE {
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }
        }
        let mut peek_reader = lightning_wire_msgs::PeekReader::from(reader);
        Ok(Error {
            code: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
            data: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
        })
    }
}
#[derive(Clone, Debug)]
pub struct CreateSession {
    pub blob_type: BlobType,
    pub max_updates: u16,
    pub reward_base: u32,
    pub reward_rate: u32,
    pub sweep_fee_rate: SatPerKWeight,
}
#[doc = "automatically generated"]
impl lightning_wire_msgs::WireMessage for CreateSession {
    const MSG_TYPE: u16 = 602;
    fn encode<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let mut count = 0;
        count += w.write(&u16::to_be_bytes(Self::MSG_TYPE))?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.blob_type, w)?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.max_updates, w)?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.reward_base, w)?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.reward_rate, w)?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.sweep_fee_rate, w)?;
        w.flush()?;
        Ok(count)
    }
    fn decode<R: std::io::Read>(reader: &mut R, check_type: bool) -> std::io::Result<Self> {
        if check_type {
            let mut msg_type = [0_u8; 2];
            reader.read_exact(&mut msg_type)?;
            let msg_type = u16::from_be_bytes(msg_type);
            if msg_type != Self::MSG_TYPE {
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }
        }
        let mut peek_reader = lightning_wire_msgs::PeekReader::from(reader);
        Ok(CreateSession {
            blob_type: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
            max_updates: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
            reward_base: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
            reward_rate: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
            sweep_fee_rate: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
        })
    }
}
#[derive(Clone, Debug)]
pub struct CreateSessionReply<T: Borrow<[u8]>> {
    pub code: Option<CreateSessionError>,
    pub last_applied: u16,
    pub data: Buffer<T>,
}
#[doc = "automatically generated"]
impl<T: Borrow<[u8]>> lightning_wire_msgs::WireMessage for CreateSessionReply<T> {
    const MSG_TYPE: u16 = 603;
    fn encode<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let mut count = 0;
        count += w.write(&u16::to_be_bytes(Self::MSG_TYPE))?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.code, w)?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.last_applied, w)?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.data, w)?;
        w.flush()?;
        Ok(count)
    }
    fn decode<R: std::io::Read>(reader: &mut R, check_type: bool) -> std::io::Result<Self> {
        if check_type {
            let mut msg_type = [0_u8; 2];
            reader.read_exact(&mut msg_type)?;
            let msg_type = u16::from_be_bytes(msg_type);
            if msg_type != Self::MSG_TYPE {
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }
        }
        let mut peek_reader = lightning_wire_msgs::PeekReader::from(reader);
        Ok(CreateSessionReply {
            code: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
            last_applied: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
            data: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
        })
    }
}
#[derive(Debug, Clone)]
pub struct StateUpdate<T: Borrow<[u8]>> {
    pub seq_num: u16,
    pub last_applied: u16,
    pub is_complete: u8,
    pub hint: [u8; 16],
    pub encrypted_blob: Buffer<T>,
}
#[doc = "automatically generated"]
impl<T: Borrow<[u8]>> lightning_wire_msgs::WireMessage for StateUpdate<T> {
    const MSG_TYPE: u16 = 604;
    fn encode<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let mut count = 0;
        count += w.write(&u16::to_be_bytes(Self::MSG_TYPE))?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.seq_num, w)?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.last_applied, w)?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.is_complete, w)?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.hint, w)?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.encrypted_blob, w)?;
        w.flush()?;
        Ok(count)
    }
    fn decode<R: std::io::Read>(reader: &mut R, check_type: bool) -> std::io::Result<Self> {
        if check_type {
            let mut msg_type = [0_u8; 2];
            reader.read_exact(&mut msg_type)?;
            let msg_type = u16::from_be_bytes(msg_type);
            if msg_type != Self::MSG_TYPE {
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }
        }
        let mut peek_reader = lightning_wire_msgs::PeekReader::from(reader);
        Ok(StateUpdate {
            seq_num: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
            last_applied: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
            is_complete: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
            hint: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
            encrypted_blob: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
        })
    }
}
#[derive(Debug, Clone)]
pub struct StateUpdateReply {
    pub code: Option<StateUpdateError>,
    pub last_applied: u16,
}
#[doc = "automatically generated"]
impl lightning_wire_msgs::WireMessage for StateUpdateReply {
    const MSG_TYPE: u16 = 605;
    fn encode<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let mut count = 0;
        count += w.write(&u16::to_be_bytes(Self::MSG_TYPE))?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.code, w)?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.last_applied, w)?;
        w.flush()?;
        Ok(count)
    }
    fn decode<R: std::io::Read>(reader: &mut R, check_type: bool) -> std::io::Result<Self> {
        if check_type {
            let mut msg_type = [0_u8; 2];
            reader.read_exact(&mut msg_type)?;
            let msg_type = u16::from_be_bytes(msg_type);
            if msg_type != Self::MSG_TYPE {
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }
        }
        let mut peek_reader = lightning_wire_msgs::PeekReader::from(reader);
        Ok(StateUpdateReply {
            code: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
            last_applied: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
        })
    }
}
#[derive(Debug, Clone)]
pub struct DeleteSession {
    pub data: (),
}
#[doc = "automatically generated"]
impl lightning_wire_msgs::WireMessage for DeleteSession {
    const MSG_TYPE: u16 = 606;
    fn encode<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let mut count = 0;
        count += w.write(&u16::to_be_bytes(Self::MSG_TYPE))?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.data, w)?;
        w.flush()?;
        Ok(count)
    }
    fn decode<R: std::io::Read>(reader: &mut R, check_type: bool) -> std::io::Result<Self> {
        if check_type {
            let mut msg_type = [0_u8; 2];
            reader.read_exact(&mut msg_type)?;
            let msg_type = u16::from_be_bytes(msg_type);
            if msg_type != Self::MSG_TYPE {
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }
        }
        let mut peek_reader = lightning_wire_msgs::PeekReader::from(reader);
        Ok(DeleteSession {
            data: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
        })
    }
}
#[derive(Debug, Clone)]
pub struct DeleteSessionReply {
    pub error: Option<DeleteSessionError>,
}
#[doc = "automatically generated"]
impl lightning_wire_msgs::WireMessage for DeleteSessionReply {
    const MSG_TYPE: u16 = 607;
    fn encode<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
        let mut count = 0;
        count += w.write(&u16::to_be_bytes(Self::MSG_TYPE))?;
        count += lightning_wire_msgs::WireItemWriter::encode(&self.error, w)?;
        w.flush()?;
        Ok(count)
    }
    fn decode<R: std::io::Read>(reader: &mut R, check_type: bool) -> std::io::Result<Self> {
        if check_type {
            let mut msg_type = [0_u8; 2];
            reader.read_exact(&mut msg_type)?;
            let msg_type = u16::from_be_bytes(msg_type);
            if msg_type != Self::MSG_TYPE {
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }
        }
        let mut peek_reader = lightning_wire_msgs::PeekReader::from(reader);
        Ok(DeleteSessionReply {
            error: lightning_wire_msgs::WireItemReader::decode(&mut peek_reader)?,
        })
    }
}
