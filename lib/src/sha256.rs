use crate::U256;
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Hash(U256);

impl Hash {
    pub fn hash<T: serde::Serialize>(data: &T) -> Self {
        let mut serialized: Vec<u8> = vec![];
        if let Err(e) = ciborium::into_writer(data, &mut serialized) {
            panic!("Failed to serialize data {:?}", e);
        }

        let hash = digest(&serialized);
        let hash_bytes = hex::decode(hash).unwrap();
        let hash_array: [u8; 32] = hash_bytes.as_slice().try_into().unwrap();

        Hash(U256::from(hash_array))
    }
}

#[cfg(test)]
mod hash {
    use super::*;
    use std::any::type_name as std_type_name;

    #[derive(Serialize, Deserialize)]
    struct Foo(usize);

    #[test]
    fn hash() {
        let foo = Foo(144);
        let hash = Hash::hash(&foo).0;
        println!("hash - {:?}", hash);

        assert_eq!(hash.bits(), 255);
        assert_eq!(type_name::<_>(hash), "btclib::U256")
    }

    fn type_name<T>(_: T) -> &'static str {
        std_type_name::<T>()
    }
}
