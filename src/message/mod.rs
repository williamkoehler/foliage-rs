mod encoder;
pub use encoder::BincodeFrameEncoder;

mod decoder;
pub use decoder::BincodeFrameDecoder;

use std::marker::PhantomData;

use serde::{de::*, ser::*, *};

pub type Tag = u16;
pub type Id = u16;
type Kind = u8;

pub enum Payload<Request, Response> {
    Request(Request),
    Response(Response),
    Error(String),
}

const REQUEST_KIND: Kind = 0x11;
const RESPONSE_KIND: Kind = 0x12;
const ERROR_KIND: Kind = 0x13;

pub struct Message<Request, Response> {
    pub tag: Tag,
    pub id: Id,
    pub payload: Payload<Request, Response>,
}

impl<Request, Response> Serialize for Message<Request, Response>
where
    Request: Serialize,
    Response: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tuple = serializer.serialize_tuple(4)?;

        // Write message tag
        tuple.serialize_element::<Tag>(&self.tag)?;

        // Write message id
        tuple.serialize_element::<Tag>(&self.id)?;

        // Write message kind and payload
        match &self.payload {
            Payload::Request(request) => {
                tuple.serialize_element::<Kind>(&REQUEST_KIND)?;
                tuple.serialize_element::<Request>(request)?;
            }
            Payload::Response(response) => {
                tuple.serialize_element::<Kind>(&RESPONSE_KIND)?;
                tuple.serialize_element::<Response>(response)?;
            }
            Payload::Error(error) => {
                tuple.serialize_element::<Kind>(&ERROR_KIND)?;
                tuple.serialize_element::<String>(error)?;
            }
        }

        tuple.end()
    }
}

impl<'de, Request, Response> Deserialize<'de> for Message<Request, Response>
where
    for<'de2> Request: Deserialize<'de2>,
    for<'de2> Response: Deserialize<'de2>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MessageVisitor<Request, Response> {
            phantom_data: PhantomData<(Request, Response)>,
        }

        impl<'de, Request, Response> Visitor<'de> for MessageVisitor<Request, Response>
        where
            for<'de2> Request: Deserialize<'de2>,
            for<'de2> Response: Deserialize<'de2>,
        {
            type Value = Message<Request, Response>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a tuple of a u16 tag, u8, kind and payload")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
                A::Error: de::Error,
            {
                // Read message tag
                let tag = match seq.next_element::<Tag>()? {
                    Some(tag) => tag,
                    None => return Err(de::Error::invalid_length(0, &self)),
                };

                // Read message id
                let id = match seq.next_element::<Id>()? {
                    Some(id) => id,
                    None => return Err(de::Error::invalid_length(1, &self)),
                };

                // Read message kind
                let kind = match seq.next_element::<Kind>()? {
                    Some(kind) => kind,
                    None => return Err(de::Error::invalid_length(2, &self)),
                }
                .into();

                // Read message payload
                let payload = {
                    let result = match kind {
                        REQUEST_KIND => seq.next_element::<Request>()?.map(Payload::Request),
                        RESPONSE_KIND => seq.next_element::<Response>()?.map(Payload::Response),
                        ERROR_KIND => seq.next_element::<String>()?.map(Payload::Error),
                        _ => Some(Payload::Error("invalid message kind".to_string())),
                    };

                    match result {
                        Some(payload) => payload,
                        None => return Err(de::Error::invalid_length(3, &self)),
                    }
                };

                Ok(Message { tag, id, payload })
            }
        }

        deserializer.deserialize_tuple(
            4,
            MessageVisitor {
                phantom_data: PhantomData,
            },
        )
    }
}
