use std::{marker::PhantomData, sync::Arc};

use tokio::net::UnixListener;

use crate::error::host::*;
use crate::peer::Peer;

pub struct Host<MyService, OtherService>
where
    MyService: crate::MyService + 'static,
    OtherService: crate::OtherService + 'static,
{
    listener: UnixListener,
    service: Arc<MyService>,

    phantom_data: PhantomData<OtherService>,
}

impl<MyService, OtherService> Host<MyService, OtherService>
where
    MyService: crate::MyService + 'static,
    OtherService: crate::OtherService + 'static,
{
    pub async fn new(group_name: &str, service: MyService) -> ResultNew<Self> {
        let socket_path = crate::peer::socket_path(group_name);
        let _ = std::fs::remove_file(&socket_path); // Remove old socket path if there is one
        let listener = UnixListener::bind(socket_path).map_err(|err| ErrorNew::BindSocket(err))?;

        Ok(Self::new_raw(listener, Arc::new(service)))
    }

    pub fn new_raw(listener: UnixListener, service: Arc<MyService>) -> Self {
        Self {
            listener,
            service,

            phantom_data: PhantomData,
        }
    }

    /// Accepts new peer asynchronously.
    ///
    pub async fn accept(&self) -> ResultAccept<Peer<MyService, OtherService>> {
        let (stream, _) = self
            .listener
            .accept()
            .await
            .map_err(|err| ErrorAccept::AcceptSocket(err))?;
        let (read, write) = stream.into_split();

        Ok(Peer::new_raw(read, write, 0, self.service.clone()).await)
    }
}
