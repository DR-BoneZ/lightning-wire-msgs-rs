use crate as lightning_wire_msgs;
use lightning_wire_msgs::wire_items::{Hash, RawFeatureVector};

#[derive(AnyWireMessage)]
pub enum AnyWatchtowerMessage {
    Init(Init),
}

#[derive(WireMessage)]
#[msg_type = 600]
pub struct Init {
    conn_features: RawFeatureVector,
    chain_hash: Hash,
}
