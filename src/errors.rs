use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImplantError {
    #[error("Response error")]
    ResponseError,
    #[error("Base64 decode error")]
    DecodeError,
    #[error("Arrayvec conversion error")]
    ArrayvecError,
    #[error("Bincode decode error")]
    BincodeError,
}

impl std::convert::From<reqwest::Error> for ImplantError {
    fn from(_err: reqwest::Error) -> Self {
        ImplantError::ResponseError
    }
}

impl std::convert::From<base64::DecodeError> for ImplantError {
    fn from(_err: base64::DecodeError) -> Self {
        ImplantError::DecodeError
    }
}

impl std::convert::From<arrayvec::ArrayVec<u8, 32_usize>> for ImplantError {
    fn from(_err: arrayvec::ArrayVec<u8, 32_usize>) -> Self {
        ImplantError::ArrayvecError
    }
}

impl std::convert::From<arrayvec::ArrayVec<u8, 24_usize>> for ImplantError {
    fn from(_err: arrayvec::ArrayVec<u8, 24_usize>) -> Self {
        ImplantError::ArrayvecError
    }
}

impl std::convert::From<Box<bincode::ErrorKind>> for ImplantError {
    fn from(_err: Box<bincode::ErrorKind>) -> Self {
        ImplantError::BincodeError
    }
}
