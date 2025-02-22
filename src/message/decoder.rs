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
pub(crate) fn decode_len(mut src: &[u8]) -> Option<(usize, usize)> {
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
            Err(err) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{BufMut, BytesMut};

    #[test]
    fn test_length_decode() {
        let lengths: Vec<Vec<u8>> = vec![
            // Valid lengths
            vec![0x00, 0x00, 0x00, 0x00, 0x00],
            vec![0x01, 0x00, 0x00, 0x00, 0x00],
            vec![0x0a, 0x00, 0x00, 0x00, 0x00],
            vec![0x20, 0x00, 0x00, 0x00, 0x00],
            vec![0x81, 0x00, 0x00, 0x00, 0x00, 0x00],
            vec![0x82, 0x00, 0x00, 0x00, 0x00, 0x00],
            vec![0x9a, 0x7a, 0x00, 0x00, 0x00, 0x00],
            // Invalid lengths
            vec![],
            vec![0x81],
            vec![0x81, 0xff],
            vec![0x82, 0x80, 0xf1],
            vec![0xff, 0x93, 0x80, 0xa2],
        ];
        let decoded_lengths: Vec<Option<(usize, usize)>> =
            lengths.iter().map(|len| decode_len(&len[..])).collect();

        let expected_lengths: Vec<Option<(usize, usize)>> = vec![
            Some((0 + 1, 1)),
            Some((1 + 1, 1)),
            Some((10 + 1, 1)),
            Some((32 + 1, 1)),
            Some((128 + 2, 2)),
            Some((256 + 2, 2)),
            Some((3450 + 2, 2)),
            None,
            None,
            None,
            None,
            None,
        ];

        for index in 0..lengths.len() {
            assert_eq!(
                decoded_lengths[index].map(|x| x.0),
                expected_lengths[index].map(|x| x.0),
                "testing decoding length length of {}",
                hex::encode(&lengths[index])
            );
            assert_eq!(
                decoded_lengths[index].map(|x| x.1),
                expected_lengths[index].map(|x| x.1),
                "testing decoding length of {}",
                hex::encode(&lengths[index])
            );
        }
    }

    #[test]
    fn test_decode() {
        let expected_content: Vec<u8> = vec![0x01, 0x12, 0x54, 0x85];
        let encoded_content = bincode::DefaultOptions::new()
            .serialize(&expected_content)
            .unwrap();

        let expected_length = encoded_content.len();
        let encoded_length = {
            let mut buffer = BytesMut::new();
            super::super::encoder::encode_len(expected_length, &mut buffer);
            buffer.to_vec()
        };

        let mut encoded = BytesMut::new();
        encoded.put_slice(&encoded_length);
        encoded.put_slice(&encoded_content);

        // Decode
        let decoded_content: Vec<u8> = BincodeFrameDecoder::default()
            .decode(&mut encoded)
            .unwrap()
            .unwrap();

        assert_eq!(decoded_content[..], expected_content[..]);
    }

    #[test]
    fn test_decode_with_missing_length_bytes() {
        let expected_length = 3456;
        let encoded_length = {
            let mut buffer = BytesMut::new();
            super::super::encoder::encode_len(expected_length, &mut buffer);
            buffer.to_vec()
        };

        let mut encoded = BytesMut::new();
        encoded.put_slice(&encoded_length[..(encoded_length.len() - 1)]); // Don't add last byte

        // Try Decode
        let x: Option<Vec<u8>> = BincodeFrameDecoder::default()
            .decode(&mut encoded)
            .unwrap();

        assert_eq!(x, None, "testing decoding with missing length bytes");
    }

    #[test]
    fn test_decode_with_missing_length_data() {
        let expected_content: Vec<u8> = vec![0x01, 0x12, 0x54, 0x85];
        let encoded_content = bincode::DefaultOptions::new()
            .serialize(&expected_content)
            .unwrap();

        let expected_length = encoded_content.len();
        let encoded_length = {
            let mut buffer = BytesMut::new();
            super::super::encoder::encode_len(expected_length, &mut buffer);
            buffer.to_vec()
        };

        let mut encoded = BytesMut::new();
        encoded.put_slice(&encoded_length);
        encoded.put_slice(&encoded_content[..(encoded_content.len() - 2)]); // Don't add last 2 bytes

        // Try Decode
        let x: Option<Vec<u8>> = BincodeFrameDecoder::default()
            .decode(&mut encoded)
            .unwrap();

        assert_eq!(x, None, "testing decoding with missing content bytes");
    }

    #[test]
    fn test_decode_with_invalid_data() {
        let expected_content: Vec<u8> = vec![0x01, 0x12, 0x54, 0x85];
        let mut encoded_content = bincode::DefaultOptions::new()
            .serialize(&expected_content)
            .unwrap();

        // Mess data up
        encoded_content[0] += 0x03;

        let expected_length = encoded_content.len();
        let encoded_length = {
            let mut buffer = BytesMut::new();
            super::super::encoder::encode_len(expected_length, &mut buffer);
            buffer.to_vec()
        };

        let mut encoded = BytesMut::new();
        encoded.put_slice(&encoded_length);
        encoded.put_slice(&encoded_content);

        // Try Decode
        let x: Result<Option<Vec<u8>>, std::io::Error> = BincodeFrameDecoder::default()
            .decode(&mut encoded);

        assert!(x.is_err(), "testing decoding with invalid data");
    }
}
