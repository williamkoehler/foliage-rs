use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorNew {
    #[error("failed to bind socket: {0}")]
    BindSocket(std::io::Error),
}

pub type ResultNew<T> = core::result::Result<T, ErrorNew>;

#[derive(Error, Debug)]
pub enum ErrorAccept {
    #[error("failed to accept socket: {0}")]
    AcceptSocket(std::io::Error),
}

pub type ResultAccept<T> = core::result::Result<T, ErrorAccept>;
