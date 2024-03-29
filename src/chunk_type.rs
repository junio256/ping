#![allow(unused_variables)]

use std::fmt::Display;
use std::str::FromStr;

use crate::chunk::Chunk;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType(Chunk);

#[derive(Debug)]
pub enum ChunkError {
    ErrorConvertFromString(Chunk),
    InvalidChunk,
}

impl TryFrom<Chunk> for ChunkType {
    type Error = ChunkError;

    fn try_from(value: Chunk) -> Result<Self, Self::Error> {
        if value.is_ascii() {
            return Ok(ChunkType(value))
        } else {
            return Err(ChunkError::InvalidChunk)
        }
    }
}

impl FromStr for ChunkType {
    type Err = ChunkError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let alpha_only = value.chars().all(char::is_alphabetic);
        let parseable = Self::from_slice(value.as_bytes());

        match (alpha_only, parseable) {
            (true, Some(x)) => Ok(x),
            _ => Err(ChunkError::InvalidChunk)
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from_utf8(Vec::from(self.0)).unwrap())
    }
}

impl ChunkType {
    /// Parse a slice into [u8; 4]
    fn from_slice(array: &[u8]) -> Option<Self> {
        match array.len() {
            4 => {
                let mut temp: [u8; 4] = [0,0,0,0];
                for i in 0..array.len() {
                    temp[i] = array[i] as u8;
                }
                Some(ChunkType(temp))
            },
            _ => None
        }
    }
    pub fn bytes(self) -> Chunk {
        self.0
    }
    pub fn is_critical(self) -> bool {
        self.0[0].is_ascii_uppercase()
    }
    pub fn is_public(self) -> bool {
        self.0[1].is_ascii_uppercase()
    }
    pub fn is_reserved_bit_valid(self) -> bool {
        self.0[2].is_ascii_uppercase()
    }
    pub fn is_safe_to_copy(self) -> bool {
        self.0[3].is_ascii_lowercase()
    }
    pub fn is_valid(self) -> bool {
        self.is_reserved_bit_valid()
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, format};
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
