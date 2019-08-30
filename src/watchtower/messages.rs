use super::items::{blob::Type as BlobType, error::ErrorCode};
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
