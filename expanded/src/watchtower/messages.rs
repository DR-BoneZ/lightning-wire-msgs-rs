use super::items::{
    blob::Type as BlobType, error::CreateSessionError, error::DeleteSessionError, error::ErrorCode,
    error::StateUpdateError,
};
use crate as lightning_wire_msgs;
use crate::items::{feature::RawFeatureVector, fees::SatPerKWeight, hash::Hash, Buffer};
use std::borrow::Borrow;
#[derive(AnyWireMessage)]
pub enum AnyWatchtowerMessage<T: Borrow<[u8]>> {
    Init(Init),
    Error(Error<T>),
}
#[derive(Clone, Debug, WireMessage)]
#[msg_type = 600]
pub struct Init {
    pub conn_features: RawFeatureVector,
    pub chain_hash: Hash,
}
#[derive(Clone, Debug, WireMessage)]
#[msg_type = 601]
pub struct Error<T: Borrow<[u8]>> {
    pub code: ErrorCode,
    pub data: Buffer<T>,
}
#[derive(Clone, Debug, WireMessage)]
#[msg_type = 602]
pub struct CreateSession {
    pub blob_type: BlobType,
    pub max_updates: u16,
    pub reward_base: u32,
    pub reward_rate: u32,
    pub sweep_fee_rate: SatPerKWeight,
}
#[derive(Clone, Debug, WireMessage)]
#[msg_type = 603]
pub struct CreateSessionReply<T: Borrow<[u8]>> {
    pub code: Option<CreateSessionError>,
    pub last_applied: u16,
    pub data: Buffer<T>,
}
#[derive(Debug, Clone, WireMessage)]
#[msg_type = 604]
pub struct StateUpdate<T: Borrow<[u8]>> {
    pub seq_num: u16,
    pub last_applied: u16,
    pub is_complete: u8,
    pub hint: [u8; 16],
    pub encrypted_blob: Buffer<T>,
}
#[derive(Debug, Clone, WireMessage)]
#[msg_type = 605]
pub struct StateUpdateReply {
    pub code: Option<StateUpdateError>,
    pub last_applied: u16,
}
#[derive(Debug, Clone, WireMessage)]
#[msg_type = 606]
pub struct DeleteSession;
#[derive(Debug, Clone, WireMessage)]
#[msg_type = 607]
pub struct DeleteSessionReply {
    pub error: Option<DeleteSessionError>,
}
