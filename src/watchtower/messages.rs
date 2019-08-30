use super::items::{blob::Type as BlobType, error::ErrorCode};
use crate as lightning_wire_msgs;
use lightning_wire_msgs::items::{feature::RawFeatureVector, hash::Hash, Buffer};
use std::borrow::Borrow;

#[derive(AnyWireMessage)]
pub enum AnyWatchtowerMessage<T: Borrow<[u8]>> {
    Init(Init),
    Error(Error<T>),
}

#[derive(Clone, Debug, WireMessage)]
#[msg_type = 600]
pub struct Init {
    conn_features: RawFeatureVector,
    chain_hash: Hash,
    #[tlv_type = 0]
    ch: Option<Hash>,
}

#[derive(Clone, Debug, WireMessage)]
#[msg_type = 601]
pub struct Error<T: Borrow<[u8]>> {
    code: ErrorCode,
    data: Buffer<T>,
}

#[derive(Clone, Debug, WireMessage)]
#[msg_type = 602]
pub struct CreateSession {
    blob_type: BlobType,
}
