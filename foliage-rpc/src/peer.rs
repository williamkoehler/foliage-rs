use std::{collections::VecDeque, marker::PhantomData, sync::Arc};

use futures::{stream::FuturesOrdered, SinkExt, StreamExt, TryStreamExt};
use sha3::{Digest, Sha3_256};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    select,
    sync::{mpsc, oneshot},
};

use crate::error::peer::*;
use crate::message::*;

pub(crate) fn socket_path(group_name: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(group_name.as_bytes());
    let hash = hasher.finalize();
    let mut hash2 = [0u8; 32];
    hash2.copy_from_slice(&hash);
    format!("/tmp/{}.sock", hex::encode(&hash2))
}

struct RequestDispatch<OtherService>
where
    OtherService: crate::OtherService + 'static,
{
    request: OtherService::Request,
    response_sender: oneshot::Sender<Result<OtherService::Response, OtherService::Error>>,
}

pub struct Peer<OtherService>
where
    OtherService: crate::OtherService + 'static,
{
    request_dispatch_sender: mpsc::UnboundedSender<RequestDispatch<OtherService>>,

    phantom_data: PhantomData<OtherService>,
}

impl<OtherService> Peer<OtherService>
where
    OtherService: crate::OtherService + 'static,
{
    /// Creates new peer using a unix socket.
    ///
    pub async fn new<MyService>(group_name: &str, service: MyService) -> ResultNew<Self>
    where
        MyService: crate::MyService + 'static,
    {
        let stream = tokio::net::UnixStream::connect(socket_path(group_name))
            .await
            .map_err(|err| ErrorNew::BindSocket(err))?;
        let (read, write) = stream.into_split();

        Ok(Self::new_raw(read, write, Arc::new(service)).await)
    }

    /// Creates new peer using raw read and write stream pair
    pub async fn new_raw<MyService, Read, Write>(
        read: Read,
        write: Write,
        service: Arc<MyService>,
    ) -> Self
    where
        MyService: crate::MyService + 'static,
        Read: AsyncRead + Send + Unpin + 'static,
        Write: AsyncWrite + Send + Unpin + 'static,
    {
        // Frame reader and writer
        let read = tokio_util::codec::FramedRead::new(
            read,
            BincodeFrameDecoder::<crate::InputMessage<MyService, OtherService>>::default(),
        );
        let write = tokio_util::codec::FramedWrite::new(
            write,
            BincodeFrameEncoder::<crate::OutputMessage<MyService, OtherService>>::default(),
        );

        // Create call channel
        let (call_sender, call_receiver) = mpsc::unbounded_channel();

        // Start worker task
        tokio::spawn(async move {
            Self::worker(read, write, service, call_receiver).await;
        });

        Self {
            request_dispatch_sender: call_sender,
            phantom_data: PhantomData,
        }
    }

    async fn worker<MyService, Read, Write>(
        mut read: tokio_util::codec::FramedRead<
            Read,
            BincodeFrameDecoder<crate::InputMessage<MyService, OtherService>>,
        >,
        mut write: tokio_util::codec::FramedWrite<
            Write,
            BincodeFrameEncoder<crate::OutputMessage<MyService, OtherService>>,
        >,
        service: Arc<MyService>,
        mut request_dispatch_receiver: mpsc::UnboundedReceiver<RequestDispatch<OtherService>>,
    ) where
        MyService: crate::MyService + 'static,
        Read: AsyncRead + Unpin,
        Write: AsyncWrite + Unpin,
    {
        let mut outgoing_id_counter: u16 = 0;
        let mut incoming_id_counter: u16 = 0;

        let mut outgoing_rpcs: VecDeque<
            oneshot::Sender<Result<OtherService::Response, OtherService::Error>>,
        > = VecDeque::new();

        let mut pending_responses = FuturesOrdered::new();

        loop {
            select! {
                Ok(Some(message)) = read.try_next() => {
                    match message.payload {
                        Payload::Request(request) => {
                            // Clone service and run rpc handler
                            let service = service.clone();
                            pending_responses.push_back(async move { (message.id, service.on_rpc(request).await) });
                        }
                        Payload::Response(response) => {
                            // Check message id
                            if message.id != incoming_id_counter {
                                // Invalid message id
                                // An error must have occured on the other side... weird... since I am the otherside as well...
                                break;
                            }

                            // Get ongoing rpc response sender
                            let rpc_resp_send = match outgoing_rpcs.pop_front() {
                                Some(call) => call,
                                None => break,
                            };
                            incoming_id_counter += 1; // Increment id counter

                            // Send (ok) response result
                            let _ = rpc_resp_send.send(Ok(response));
                        }
                        Payload::Error(err) => {
                            // Check message id
                            if message.id != incoming_id_counter {
                                // Invalid message id
                                // An error must have occured on the other side... weird... since I am the otherside as well...
                                break;
                            }

                            // Get ongoing rpc response sender
                            let rpc_resp_send = match outgoing_rpcs.pop_front() {
                                Some(call) => call,
                                None => break,
                            };
                            incoming_id_counter += 1; // Increment id counter

                            // Send error result
                            let _ = rpc_resp_send.send(Err(err));
                        }
                    }
                }
                Some(request_dispatch) = request_dispatch_receiver.recv() => {
                    let message = Message {
                        id: outgoing_id_counter,
                        payload: Payload::Request(request_dispatch.request),
                    };

                    let _ = write.send(message).await;
                    let _ = write.flush().await;

                    outgoing_id_counter += 1; // Increment id counter
                    outgoing_rpcs.push_back(request_dispatch.response_sender);
                }
                Some((id, pending_response)) = pending_responses.next() => {
                    let message = Message {
                        id: id,
                        payload: match pending_response {
                            Ok(response) => Payload::Response(response),
                            Err(err) => Payload::Error(err),
                        },
                    };
                    let _ = write.send(message).await;
                    let _ = write.flush().await;
                }
                else => {
                    // A channel has closed or an error occurred.
                    // As a result the worker task is being exited.
                    break;
                }
            }
        }
    }

    /// Sends an rpc request and waits asynchronously for a response.
    pub async fn rpc(
        &mut self,
        request: OtherService::Request,
    ) -> ResultRpc<OtherService::Response, OtherService::Error> {
        // Create response channel
        let (response_sender, response_receiver) = oneshot::channel();

        // Send request
        self.request_dispatch_sender
            .send(RequestDispatch {
                request,
                response_sender,
            })
            .map_err(|_| ErrorRpc::StreamClosed)?;

        // Wait for response
        match response_receiver.await {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(err)) => Err(ErrorRpc::RpcError(err)),
            Err(_) => Err(ErrorRpc::StreamClosed),
        }
    }
}

impl<OtherService> Clone for Peer<OtherService>
where
    OtherService: crate::OtherService + 'static,
{
    fn clone(&self) -> Self {
        Self {
            request_dispatch_sender: self.request_dispatch_sender.clone(),
            phantom_data: PhantomData,
        }
    }
}
