use bincode::Options;
use bytes::Buf;
use std::marker::PhantomData;
use tokio_util::codec::Decoder;

pub struct BincodeFrameDecoder<T> {
    phantom_data: PhantomData<T>,
}

impl<T> Default for BincodeFrameDecoder<T> {
    fn default() -> Self {
        Self {
            phantom_data: PhantomData,
        }
    }
}

#[inline(always)]
fn decode_len(mut src: &[u8]) -> Option<(usize, usize)> {
    let mut len = 0usize;
    let mut len_len = 1usize;
    while !src.is_empty() {
        len = (len << 7) | (src[0] & 0x7f) as usize;
        if src[0] >> 7 == 0 {
            return Some((len + len_len, len_len));
        }
        len_len += 1;
        src = &src[1..];
    }

    None
}

impl<T> Decoder for BincodeFrameDecoder<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    type Item = T;
    type Error = std::io::Error;

    fn decode(&mut self, buffer: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let (len, len_len) = match decode_len(buffer) {
            None => return Ok(None),
            Some(len) => len,
        };

        if buffer.len() < len {
            buffer.reserve(len - buffer.len());
            return Ok(None);
        }

        let data = &buffer[len_len..len];
        let result = bincode::DefaultOptions::new().deserialize(data);
        buffer.advance(len);

        match result {
            Ok(v) => Ok(Some(v)),
            Err(err) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                err,
            )),
        }
    }
}
