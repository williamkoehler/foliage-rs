use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorNew {
    #[error("failed to bind socket: {0}")]
    BindSocket(std::io::Error),
}

pub type ResultNew<T> = core::result::Result<T, ErrorNew>;

#[derive(Error, Debug)]
pub enum ErrorRpc {
    #[error("internal error: {message}")]
    InternalError { message: &'static str },

    #[error("stream is closed")]
    StreamClosed,

    #[error("rpc error: {0}")]
    RpcError(String),
}

pub type ResultRpc<T> = core::result::Result<T, ErrorRpc>;
