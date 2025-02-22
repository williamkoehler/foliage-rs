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
pub(crate) fn encode_len(mut len: usize, buffer: &mut BytesMut) {
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
            .expect("No length limit") as usize;

        buffer.reserve(enc_len + MAX_LEN_BYTES);

        encode_len(enc_len, buffer);

        let mut writer = buffer.writer();
        bincode::DefaultOptions::new()
            .serialize_into(&mut writer, &item)
            .expect("No length limit");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;
    use tokio_util::codec::Encoder;

    #[test]
    fn test_length_encode() {
        let lengths: Vec<usize> = vec![0, 1, 10, 32, 128, 256, 3450];
        let encoded_lengths: Vec<Vec<u8>> = lengths
            .iter()
            .map(|len| {
                let mut buffer = BytesMut::new();
                encode_len(*len, &mut buffer);
                buffer.to_vec()
            })
            .collect();

        let expected_lengths: Vec<Vec<u8>> = vec![
            vec![0x00],
            vec![0x01],
            vec![0x0a],
            vec![0x20],
            vec![0x81, 0x00],
            vec![0x82, 0x00],
            vec![0x9a, 0x7a],
        ];

        for index in 0..lengths.len() {
            assert_eq!(
                encoded_lengths[index], expected_lengths[index],
                "testing encoding of length {}",
                lengths[index]
            );
        }
    }

    #[test]
    fn test_encode() {
        let decoded_content: Vec<u8> = vec![0x01, 0x12, 0x54, 0x85];

        let expected_content = bincode::DefaultOptions::new().serialize(&decoded_content).unwrap();
        let expected_length = {
            let mut buffer = BytesMut::new();
            encode_len(expected_content.len(), &mut buffer);
            buffer.to_vec()
        };

        // Encode
        let mut encoded = BytesMut::new();
        BincodeFrameEncoder::default()
            .encode(decoded_content, &mut encoded)
            .unwrap();

        assert_eq!(encoded.len(), expected_length.len() + expected_content.len());

        let encoded_length = encoded.split_to(expected_length.len());
        assert_eq!(encoded_length[..], expected_length);

        let encoded_content = encoded.split_to(expected_content.len());
        assert_eq!(encoded_content[..], expected_content);
    }
}
