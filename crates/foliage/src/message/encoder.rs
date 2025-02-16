use bincode::Options;
use bytes::{BufMut, BytesMut};
use std::marker::PhantomData;
use tokio_util::codec::Encoder;

const MAX_LEN_BYTES: usize = 10;

pub struct BincodeFrameEncoder<T> {
    phantom_data: PhantomData<T>,
}

impl<T> Default for BincodeFrameEncoder<T> {
    fn default() -> Self {
        Self {
            phantom_data: PhantomData,
        }
    }
}

#[inline(always)]
fn encode_len(mut len: u64, buffer: &mut BytesMut) {
    // First encode into a temporary buffer
    let mut encoding = [0u8; MAX_LEN_BYTES];
    let mut idx = MAX_LEN_BYTES;

    loop {
        // Store 7 bits per byte
        let mut next_byte = (len & 0x7f) as u8;
        len >>= 7;

        if idx != MAX_LEN_BYTES {
            // If this is not the last byte, we set the top bit which indicates more bytes are
            // following
            next_byte |= 0x80;
        }

        idx -= 1;
        encoding[idx] = next_byte;

        if len == 0 {
            buffer.put_slice(&encoding[idx..]);
            return;
        }
    }
}

impl<T> Encoder<T> for BincodeFrameEncoder<T>
where
    T: serde::Serialize,
{
    type Error = std::io::Error;

    fn encode(&mut self, item: T, buffer: &mut BytesMut) -> Result<(), Self::Error> {
        let enc_len = bincode::DefaultOptions::new()
            .serialized_size(&item)
            .expect("No length limit");

        buffer.reserve(enc_len as usize + MAX_LEN_BYTES);

        encode_len(enc_len, buffer);

        let mut writer = buffer.writer();
        bincode::DefaultOptions::new()
            .serialize_into(&mut writer, &item)
            .expect("No length limit");

        Ok(())
    }
}
