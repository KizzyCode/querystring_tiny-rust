//! Implements percent-coding

use crate::error::Error;
use std::ops::Deref;

/// Some percent-encoded data
#[derive(Debug, Default)]
pub struct PercentCoded {
    /// The decoded data
    data: Vec<u8>,
}
impl PercentCoded {
    /// The allowed chars
    const ALLOWED_CHARS: &'static [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~";

    /// Creates a new percent coder
    pub fn new<T>(data: T) -> Self
    where
        T: Into<Vec<u8>>,
    {
        Self { data: data.into() }
    }
    /// Percent-decodes some data
    pub fn decode(source: &[u8]) -> Result<Self, Error> {
        // Decode all hex literals
        let (mut source, mut decoded) = (source.iter().copied(), Vec::new());
        while let Some(mut byte) = source.next() {
            // Decode percent literal if necessary
            if byte == b'%' {
                // Get the encoded bytes
                let high = source.next().ok_or(Error::PercentEncoding)?;
                let low = source.next().ok_or(Error::PercentEncoding)?;
                byte = Self::decode_byte(high, low)?;
            }

            // Write byte
            decoded.push(byte);
        }
        Ok(Self { data: decoded })
    }
    /// Encodes a nibble into a hex char
    fn decode_nibble(nibble: u8) -> Result<u8, Error> {
        match nibble {
            b'0'..=b'9' => Ok(nibble - b'0'),
            b'a'..=b'f' => Ok((nibble - b'a') + 0xA),
            b'A'..=b'F' => Ok((nibble - b'A') + 0xA),
            _ => Err(Error::PercentEncoding),
        }
    }
    /// Encodes a byte
    fn decode_byte(high: u8, low: u8) -> Result<u8, Error> {
        Ok(Self::decode_nibble(high)? << 4 | Self::decode_nibble(low)?)
    }

    /// Percent-encodes some data
    pub fn encode(&self) -> Vec<u8> {
        // Copy-encode all bytes
        let mut encoded = Vec::new();
        for &byte in self.data.iter() {
            // Encode the byte if necessary
            let (mut buf, mut buf_len) = ([byte, 0, 0], 1);
            if !Self::ALLOWED_CHARS.contains(&byte) {
                let (high, low) = Self::encode_byte(byte);
                buf = [b'%', high, low];
                buf_len = 3;
            }

            // Write `byte` and high+low if any
            encoded.extend(&buf[..buf_len]);
        }
        encoded
    }
    /// Encodes a nibble into a hex char
    fn encode_nibble(nibble: u8) -> u8 {
        match nibble {
            0x0..=0x9 => nibble + b'0',
            0xA..=0xF => (nibble - 0xA) + b'A',
            nibble => unreachable!("Invalid nibble value: {nibble}"),
        }
    }
    /// Encodes a byte
    fn encode_byte(byte: u8) -> (u8, u8) {
        let (high, low) = (byte >> 4, byte & 0xF);
        (Self::encode_nibble(high), Self::encode_nibble(low))
    }
}
impl Deref for PercentCoded {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
