// Hash functions module (BLAKE2B)
//
// Copyright (c) 2019 Neutral Money Developers

//! These functions are used for hashing in NMTP
//! 
//! `libnmtp::crypto::hash` can be used to hash data.
//!
//! the checksum function uses the hash function BLAKE2b

extern crate orion;

use orion::hash;
use orion::errors;

/// Wraps Orion's `UnknownCryptoError` for standardization of the API
type HashErr = errors::UnknownCryptoError;

/// Wraps a four-wide `u8` array
type Checksum = [u8; 4];

/// Creates a new checksum 
/// This truncates a 32 byte hash into a 4 byte checksum
/// Intended use for NMTP is a addresses
pub fn new_checksum(data: &[u8]) -> Result<Checksum, HashErr> {
    let checksum = hash::digest(data)?;
    let bytes = checksum.as_bytes();

    Ok([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// Creates a new BLAKE2b, 256bit hash
/// No need for streaming api because everything in NMTP is small enough to fit
/// inside one buffer
/// Intended use for NMTP is general purpose hashing
pub fn new_digest(data: &[u8]) -> Result<hash::Digest, HashErr> {
    let hash = hash::digest(data)?;

    Ok(hash)
}

/// Testing 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_test() {
        let test_arr: [u8; 4] = [0x16, 0xe0, 0xbf, 0x1f];
        assert_eq!(new_checksum(b"123456789").unwrap(), test_arr);
    }

    #[test]
    fn digest_test() {
        // No need to test for test vectors
        // already done in the Orion library
        assert!(new_digest(b"").is_ok());
    }
}
