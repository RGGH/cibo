# Rust CBOR Serializer and SHA-256 Hasher

### Serialize data (such as transaction data or any other structured data) and then compute a SHA-256 hash of that serialized data

This repository provides a Rust implementation for serializing any `serde::Serialize` type into CBOR format using `ciborium` and hashing the serialized data using SHA-256. The resulting hash is displayed in hexadecimal format.

## Features
- Serialize any data structure that implements the `serde::Serialize` trait.
- Use CBOR (Concise Binary Object Representation) for serialization.
- Compute a SHA-256 hash of the serialized data.
- Output the hash in hexadecimal format.

---

## Code Explanation

### `Hash` Struct
```rust
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
```
The `Hash` struct encapsulates the hash value using `U256` from the `primitive_types` crate. The `to_hex` method converts the internal hash to a hexadecimal string for easy readability.

### `hash` Function
```rust
pub fn hash<T: Serialize>(data: &T) -> Hash {
    let mut serialized: Vec<u8> = vec![];
    if let Err(e) = ciborium::into_writer(data, &mut serialized) {
        panic!(
            "Failed to serialize data: {:?}. \
            This should not happen",
            e
        );
    }

    let mut hasher = Sha256::new();
    hasher.update(&serialized);
    let hash_bytes = hasher.finalize();

    let hash_array: [u8; 32] = hash_bytes.as_slice().try_into().unwrap();
    Hash(U256::from(hash_array))
}
```
This function:
1. Serializes the input data into CBOR format using `ciborium::into_writer`.
2. Computes the SHA-256 hash of the serialized data.
3. Converts the hash into a `U256` for storage in the `Hash` struct.

### Example Usage
```rust
#[derive(Serialize, Debug)]
struct MyData {
    id: u32,
    name: String,
}

fn main() {
    let data = MyData {
        id: 42,
        name: "Alice".to_string(),
    };

    let hashed = hash(&data);
    println!("Hash (hex): {}", hashed.to_hex());
}
```
This example demonstrates how to:
- Define a `serde::Serialize` struct (`MyData`).
- Serialize and hash an instance of the struct.
- Print the resulting hash in hexadecimal format.

---

## Prerequisites

Ensure you have Rust installed. If not, install it via [rustup](https://rustup.rs/).

---

## Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/sha256-cbor-hasher.git
   cd sha256-cbor-hasher
   ```

2. Add dependencies to your `Cargo.toml`:
   ```toml
   [dependencies]
   ciborium = "0.2"
   sha2 = "0.10"
   serde = { version = "1.0", features = ["derive"] }
   primitive-types = "0.10"
   hex = "0.4"
   ```

3. Build and run the project:
   ```bash
   cargo run
   ```

---

## Example Output
For the input data:
```rust
MyData {
    id: 42,
    name: "Alice".to_string(),
}
```
The output might look like:
```
Hash (hex): c9a2e8e9d15b45cf8dbf5c05fa0e30fca0a64d9b0c8d927cb7d2a253b4a2dc3b
```

---

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Contributions
Contributions are welcome! Feel free to submit a pull request or open an issue to suggest improvements or report bugs.

---

## References
- [ciborium](https://docs.rs/ciborium)
- [sha2](https://docs.rs/sha2)
- [serde](https://docs.rs/serde)
- [primitive-types](https://docs.rs/primitive-types)


