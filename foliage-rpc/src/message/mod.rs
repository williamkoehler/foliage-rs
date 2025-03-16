mod encoder;
pub use encoder::BincodeFrameEncoder;

mod decoder;
pub use decoder::BincodeFrameDecoder;

use std::marker::PhantomData;

use serde::{de::*, ser::*, *};

pub type Id = u16;
type Kind = u8;

#[derive(Debug, PartialEq, Eq)]
pub enum Payload<Request, Response, Error> {
    Request(Request),
    Response(Response),
    Error(Error),
}

const REQUEST_KIND: Kind = 0x11;
const RESPONSE_KIND: Kind = 0x12;
const ERROR_KIND: Kind = 0x13;

pub struct Message<Request, Response, Error> {
    pub id: Id,
    pub payload: Payload<Request, Response, Error>,
}

impl<Request, Response, Error> Serialize for Message<Request, Response, Error>
where
    Request: Serialize,
    Response: Serialize,
    Error: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tuple = serializer.serialize_tuple(3)?;

        // Write message id
        tuple.serialize_element::<Id>(&self.id)?;

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
                tuple.serialize_element::<Error>(error)?;
            }
        }

        tuple.end()
    }
}

impl<'de, Request, Response, Error> Deserialize<'de> for Message<Request, Response, Error>
where
    for<'de2> Request: Deserialize<'de2>,
    for<'de2> Response: Deserialize<'de2>,
    for<'de2> Error: Deserialize<'de2>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MessageVisitor<Request, Response, Error> {
            phantom_data: PhantomData<(Request, Response, Error)>,
        }

        impl<'de, Request, Response, Error> Visitor<'de> for MessageVisitor<Request, Response, Error>
        where
            for<'de2> Request: Deserialize<'de2>,
            for<'de2> Response: Deserialize<'de2>,
            for<'de2> Error: Deserialize<'de2>,
        {
            type Value = Message<Request, Response, Error>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a tuple of a u16 tag, u8, kind and payload")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
                A::Error: de::Error,
            {
                // Read message id
                let id = match seq.next_element::<Id>()? {
                    Some(id) => id,
                    None => return Err(de::Error::invalid_length(0, &self)),
                };

                // Read message kind
                let kind = match seq.next_element::<Kind>()? {
                    Some(kind) => kind,
                    None => return Err(de::Error::invalid_length(1, &self)),
                }
                .into();

                // Read message payload
                let payload = {
                    let result = match kind {
                        REQUEST_KIND => seq.next_element::<Request>()?.map(Payload::Request),
                        RESPONSE_KIND => seq.next_element::<Response>()?.map(Payload::Response),
                        ERROR_KIND => seq.next_element::<Error>()?.map(Payload::Error),
                        _ => {
                            return Err(de::Error::unknown_variant(
                                "unknown message kind",
                                &["request message", "response message", "error message"],
                            ))
                        }
                    };

                    match result {
                        Some(payload) => payload,
                        None => return Err(de::Error::invalid_length(2, &self)),
                    }
                };

                Ok(Message { id, payload })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct Data {
        a: u32,
        b: bool,
        c: String,
        d: Vec<String>,
    }

    impl Data {
        pub fn new() -> Self {
            Data {
                a: 42,
                b: true,
                c: "Hello, world!".to_string(),
                d: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            }
        }
    }

    type SimpleMessage = Message<String, String, String>;
    type ComplexMessage = Message<Data, Data, String>;

    #[test]
    fn test_simple_request_serialization() {
        let message: SimpleMessage = SimpleMessage {
            id: 4052,
            payload: Payload::Request("Some random message".to_string()),
        };

        let raw = bincode::serialize(&message).unwrap();
        let message2: SimpleMessage = bincode::deserialize(&raw).unwrap();

        assert_eq!(message.id, message2.id);
        assert_eq!(message.payload, message2.payload);
    }

    #[test]
    fn test_simple_response_serialization() {
        let message: SimpleMessage = SimpleMessage {
            id: 6184,
            payload: Payload::Response("Some random message".to_string()),
        };

        let raw = bincode::serialize(&message).unwrap();
        let message2: SimpleMessage = bincode::deserialize(&raw).unwrap();

        assert_eq!(message.id, message2.id);
        assert_eq!(message.payload, message2.payload);
    }

    #[test]
    fn test_complex_request_serialization() {
        let message: ComplexMessage = ComplexMessage {
            id: 1943,
            payload: Payload::Request(Data::new()),
        };

        let raw = bincode::serialize(&message).unwrap();
        let message2: ComplexMessage = bincode::deserialize(&raw).unwrap();

        assert_eq!(message.id, message2.id);
        assert_eq!(message.payload, message2.payload);
    }

    #[test]
    fn test_complex_response_serialization() {
        let message: ComplexMessage = ComplexMessage {
            id: 2305,
            payload: Payload::Response(Data::new()),
        };

        let raw = bincode::serialize(&message).unwrap();
        let message2: ComplexMessage = bincode::deserialize(&raw).unwrap();

        assert_eq!(message.id, message2.id);
        assert_eq!(message.payload, message2.payload);
    }

    #[test]
    fn test_error_serialization() {
        let message: SimpleMessage = SimpleMessage {
            id: 4932,
            payload: Payload::Error("Oh no something happened!".to_string()),
        };

        let raw = bincode::serialize(&message).unwrap();
        let message2: SimpleMessage = bincode::deserialize(&raw).unwrap();

        assert_eq!(message.id, message2.id);
        assert_eq!(message.payload, message2.payload);
    }
}
