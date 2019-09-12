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
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum BaseError {
    OK = 0,
    TemporaryFailure = 40,
    PermanentFailure = 50,
}
#[doc = "automatically generated"]
impl std::convert::TryFrom<u16> for BaseError {
    type Error = u16;
    fn try_from(prim: u16) -> Result<Self, Self::Error> {
        const C0: u16 = BaseError::OK as u16;
        const C1: u16 = BaseError::TemporaryFailure as u16;
        const C2: u16 = BaseError::PermanentFailure as u16;
        match prim {
            C0 => Ok(BaseError::OK),
            C1 => Ok(BaseError::TemporaryFailure),
            C2 => Ok(BaseError::PermanentFailure),
            _ => Err(prim),
        }
    }
}
impl From<BaseError> for ErrorCode {
    fn from(code: BaseError) -> ErrorCode {
        ErrorCode(code as u16)
    }
}
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum CreateSessionError {
    AlreadyExists = 60,
    RejectMaxUpdates = 61,
    RejectRewardRate = 62,
    RejectSweepFeeRate = 63,
    RejectBlobType = 64,
}
#[doc = "automatically generated"]
impl std::convert::TryFrom<u16> for CreateSessionError {
    type Error = u16;
    fn try_from(prim: u16) -> Result<Self, Self::Error> {
        const C0: u16 = CreateSessionError::AlreadyExists as u16;
        const C1: u16 = CreateSessionError::RejectMaxUpdates as u16;
        const C2: u16 = CreateSessionError::RejectRewardRate as u16;
        const C3: u16 = CreateSessionError::RejectSweepFeeRate as u16;
        const C4: u16 = CreateSessionError::RejectBlobType as u16;
        match prim {
            C0 => Ok(CreateSessionError::AlreadyExists),
            C1 => Ok(CreateSessionError::RejectMaxUpdates),
            C2 => Ok(CreateSessionError::RejectRewardRate),
            C3 => Ok(CreateSessionError::RejectSweepFeeRate),
            C4 => Ok(CreateSessionError::RejectBlobType),
            _ => Err(prim),
        }
    }
}
impl From<CreateSessionError> for ErrorCode {
    fn from(code: CreateSessionError) -> ErrorCode {
        ErrorCode(code as u16)
    }
}
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum StateUpdateError {
    ClientBehind = 70,
    MaxUpdatesExceeded = 71,
    SeqNumOutOfOrder = 72,
}
#[doc = "automatically generated"]
impl std::convert::TryFrom<u16> for StateUpdateError {
    type Error = u16;
    fn try_from(prim: u16) -> Result<Self, Self::Error> {
        const C0: u16 = StateUpdateError::ClientBehind as u16;
        const C1: u16 = StateUpdateError::MaxUpdatesExceeded as u16;
        const C2: u16 = StateUpdateError::SeqNumOutOfOrder as u16;
        match prim {
            C0 => Ok(StateUpdateError::ClientBehind),
            C1 => Ok(StateUpdateError::MaxUpdatesExceeded),
            C2 => Ok(StateUpdateError::SeqNumOutOfOrder),
            _ => Err(prim),
        }
    }
}
impl From<StateUpdateError> for ErrorCode {
    fn from(code: StateUpdateError) -> ErrorCode {
        ErrorCode(code as u16)
    }
}
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum DeleteSessionError {
    NotFound = 80,
}
#[doc = "automatically generated"]
impl std::convert::TryFrom<u16> for DeleteSessionError {
    type Error = u16;
    fn try_from(prim: u16) -> Result<Self, Self::Error> {
        const C0: u16 = DeleteSessionError::NotFound as u16;
        match prim {
            C0 => Ok(DeleteSessionError::NotFound),
            _ => Err(prim),
        }
    }
}
impl From<DeleteSessionError> for ErrorCode {
    fn from(code: DeleteSessionError) -> ErrorCode {
        ErrorCode(code as u16)
    }
}
