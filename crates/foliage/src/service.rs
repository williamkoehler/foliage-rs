use serde::{de::DeserializeOwned, Serialize};

use crate::message::{Message, Tag};

/// Service declaration trait that defines the service of the opposite peer.
///
pub trait OtherService {
    /// Rpc request type
    type Request: Serialize + Send;

    /// Rpc response type
    type Response: DeserializeOwned + Send;

    /// Rpc error response type
    type ErrorResponse: ToString + Send;
}

/// Service implementation trait where the call function is executed asynchronously for each request.
///
pub trait MyService
where
    Self: Send + Sync,
{
    /// Rpc request type
    type Request: DeserializeOwned + Send;

    /// Rpc response type
    type Response: Serialize + Send;

    /// Rpc error response type
    type Error: ToString + Send;

    /// Normal rpc handler
    fn on_rpc(
        &self,
        tag: Tag,
        request: Self::Request,
    ) -> impl std::future::Future<Output = Result<Self::Response, Self::Error>> + Send;
}

#[allow(type_alias_bounds)]
pub(crate) type InputMessage<MyServiceDecl, OtherServiceDecl>
where
    MyServiceDecl: MyService,
    OtherServiceDecl: OtherService,
= Message<MyServiceDecl::Request, OtherServiceDecl::Response>;

#[allow(type_alias_bounds)]
pub(crate) type OutputMessage<MyServiceDecl, OtherServiceDecl>
where
    MyServiceDecl: MyService,
    OtherServiceDecl: OtherService,
= Message<OtherServiceDecl::Request, MyServiceDecl::Response>;
