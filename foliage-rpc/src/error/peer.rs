use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorNew {
    #[error("failed to bind socket: {0}")]
    BindSocket(std::io::Error),
}

pub type ResultNew<T> = core::result::Result<T, ErrorNew>;

#[derive(Error, Debug)]
pub enum ErrorRpc<Error> {
    #[error("internal error: {message}")]
    InternalError { message: &'static str },

    #[error("stream is closed")]
    StreamClosed,

    #[error("rpc error: {0}")]
    RpcError(Error),
}

pub type ResultRpc<T, Error> = core::result::Result<T, ErrorRpc<Error>>;

pub fn error_rpc_match<A, B, FnRpcError>(err: ErrorRpc<A>, for_rpc_error: FnRpcError) -> ErrorRpc<B>
where
    FnRpcError: Fn(A) -> B,
{
    match err {
        ErrorRpc::InternalError { message } => ErrorRpc::InternalError { message },
        ErrorRpc::StreamClosed => ErrorRpc::StreamClosed,
        ErrorRpc::RpcError(a) => ErrorRpc::RpcError(for_rpc_error(a)),
    }
}
