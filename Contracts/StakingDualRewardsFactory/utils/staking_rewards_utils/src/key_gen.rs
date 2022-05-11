// Methods to generate keys for dicitonaries in Wise crates
extern crate alloc;
use alloc::{format, string::String, vec::Vec};

use casper_types::{bytesrepr::ToBytes, Key, U256};
use hex::encode;
use renvm_sig::keccak256;

pub fn generate_key_for_dictionary(key: &Key, id: &Vec<u32>) -> String {
    let key_str = format!("{}{:?}", key, id);
    encode(keccak256(key_str.as_bytes())) // since concatinated key is too long, hash it to reduce length
}

pub fn to_bytes16(x: U256) -> Vec<u16> {
    let x: Vec<u8> = x.to_bytes().unwrap_or_default();
    let result: Vec<u16> = x
        .chunks_exact(2)
        .into_iter()
        .map(|a| u16::from_ne_bytes([a[0], a[1]]))
        .collect(); // Create a native endian integer value

    result
}

pub fn generate_id(x: Key, y: U256, z: u8) -> Vec<u32> {
    let encoded: String = format!("{}{}{}", x, y, z);
    let hash: [u8; 32] = keccak256(encoded.as_bytes());

    let id_u16: Vec<u16> = hash
        .chunks_exact(2)
        .into_iter()
        .map(|a| u16::from_ne_bytes([a[0], a[1]]))
        .collect(); // Create a native endian integer value

    let mut id_u32: Vec<u32> = Vec::new();
    for n in id_u16 {
        id_u32.push(u32::from(n));
    }

    id_u32 // Casper doesnot support u16 therefore returning u32
}
