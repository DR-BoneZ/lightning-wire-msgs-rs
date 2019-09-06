use crate::WireItem;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(Clone, Copy, Debug)]
pub struct ErrorCode(pub u16);
impl ErrorCode {
    pub fn is_ok(&self) -> bool {
        self.0 == BaseError::OK as u16
    }
}
impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            BaseError::try_from(self.0)
                .map(|a| format!("Code{:?}", a))
                .or_else(|_| CreateSessionError::try_from(self.0)
                    .map(|a| format!("CreateSessionCode{:?}", a)))
                .or_else(|_| StateUpdateError::try_from(self.0)
                    .map(|a| format!("StateUpdateCode{:?}", a)))
                .or_else(|_| DeleteSessionError::try_from(self.0)
                    .map(|a| format!("DeleteSessionCode{:?}", a)))
                .unwrap_or_else(|_| format!("Unknown"))
        )
    }
}
impl WireItem for ErrorCode {
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        w.write(&u16::to_be_bytes(self.0))
    }

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        let mut code = [0_u8; 2];
        r.read_exact(&mut code)?;
        Ok(ErrorCode(u16::from_be_bytes(code)))
    }
}
impl<T> WireItem for Option<T>
where
    ErrorCode: From<T>,
    T: Copy + TryFrom<u16>,
{
    fn encode<W: Write>(&self, w: &mut W) -> std::io::Result<usize> {
        if let Some(t) = self {
            ErrorCode::from(*t)
        } else {
            <ErrorCode as From<BaseError>>::from(BaseError::OK)
        }
        .encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> std::io::Result<Self> {
        let ecode = ErrorCode::decode(r)?;
        if ecode.is_ok() {
            Ok(None)
        } else {
            T::try_from(ecode.0)
                .map(Some)
                .map_err(|_| std::io::ErrorKind::InvalidData.into())
        }
    }
}

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(u16)]
pub enum BaseError {
    // CodeOK signals that the request was successfully processed by the
    // watchtower
    OK = 0,

    // CodeTemporaryFailure alerts the client that the watchtower is
    // temporarily unavailable, but that it may try again at a later time.
    TemporaryFailure = 40,

    // CodePermanentFailure alerts the client that the watchtower has
    // permanently failed, and further communication should be avoided.
    PermanentFailure = 50,
}
impl From<BaseError> for ErrorCode {
    fn from(code: BaseError) -> ErrorCode {
        ErrorCode(code as u16)
    }
}

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(u16)]
pub enum CreateSessionError {
    // CreateSessionCodeAlreadyExists is returned when a session is already
    // active for the public key used to connect to the watchtower. The
    // response includes the serialized reward address in case the original
    // reply was never received and/or processed by the client.
    AlreadyExists = 60,

    // CreateSessionCodeRejectMaxUpdates the tower rejected the maximum
    // number of state updates proposed by the client
    RejectMaxUpdates = 61,

    // CreateSessionCodeRejectRewardRate the tower rejected the reward rate
    // proposed by the client.
    RejectRewardRate = 62,

    // CreateSessionCodeRejectSweepFeeRate the tower rejected the sweep fee
    // rate proposed by the client.
    RejectSweepFeeRate = 63,

    // CreateSessionCodeRejectBlobType is returned when the tower does not
    // support the proposed blob type.
    RejectBlobType = 64,
}
impl From<CreateSessionError> for ErrorCode {
    fn from(code: CreateSessionError) -> ErrorCode {
        ErrorCode(code as u16)
    }
}

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(u16)]
pub enum StateUpdateError {
    // StateUpdateCodeClientBehind signals that the client's sequence number
    // is behind what the watchtower expects based on its LastApplied. This
    // error should cause the client to record the LastApplied field in the
    // response, and initiate another attempt with the proper sequence
    // number.
    //
    // NOTE: Repeated occurrences of this could be interpreted as an attempt
    // to siphon state updates from the client. If the client believes it
    // is not violating the protocol, this could be grounds to blacklist
    // this tower from future session negotiation.
    ClientBehind = 70,

    // StateUpdateCodeMaxUpdatesExceeded signals that the client tried to
    // send a sequence number beyond the negotiated MaxUpdates of the
    // session.
    MaxUpdatesExceeded = 71,

    // StateUpdateCodeSeqNumOutOfOrder signals the client sent an update
    // that does not follow the required incremental monotonicity required
    // by the tower.
    SeqNumOutOfOrder = 72,
}
impl From<StateUpdateError> for ErrorCode {
    fn from(code: StateUpdateError) -> ErrorCode {
        ErrorCode(code as u16)
    }
}

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(u16)]
pub enum DeleteSessionError {
    // DeleteSessionCodeNotFound is returned when the watchtower does not
    // know of the requested session. This may indicate an error on the
    // client side, or that the tower had already deleted the session in a
    // prior request that the client may not have received.
    NotFound = 80,
}
impl From<DeleteSessionError> for ErrorCode {
    fn from(code: DeleteSessionError) -> ErrorCode {
        ErrorCode(code as u16)
    }
}
