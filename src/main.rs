use ciborium;
use sha2::{Sha256, Digest};
use serde::Serialize;
use primitive_types::U256; // shitcoiner's crate

#[derive(Debug)]
pub struct Hash(U256);

impl Hash {
    pub fn to_hex(&self) -> String {
        // Convert the U256 value to bytes and encode as hexadecimal
        let mut bytes = [0u8; 32];
        self.0.to_big_endian(&mut bytes);
        hex::encode(bytes)
    }
}

pub fn hash<T: Serialize>(data: &T) -> Hash {
    // Serialize the data to CBOR using `into_writer`
    let mut serialized: Vec<u8> = vec![];
    if let Err(e) = ciborium::into_writer(data, &mut serialized) {
        panic!(
            "Failed to serialize data: {:?}. \
            This should not happen",
            e
        );
    }

    // Compute SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(&serialized);
    let hash_bytes = hasher.finalize();

    // Convert hash bytes to U256
    let hash_array: [u8; 32] = hash_bytes.as_slice().try_into().unwrap();
    Hash(U256::from(hash_array))
}

#[derive(Serialize, Debug)]
struct MyData {
    id: u32,
    name: String,
}

fn main() {
    // Example data
    let data = MyData {
        id: 42,
        name: "Alice".to_string(),
    };

    // Hash the data
    let hashed = hash(&data);

    // Print the hash in hexadecimal format
    println!("Hash (hex): {}", hashed.to_hex());
}

