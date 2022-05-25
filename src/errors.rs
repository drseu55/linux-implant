use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImplantError {
    #[error("Response error")]
    ResponseError,
}

impl std::convert::From<reqwest::Error> for ImplantError {
    fn from(_err: reqwest::Error) -> Self {
        ImplantError::ResponseError
    }
}
