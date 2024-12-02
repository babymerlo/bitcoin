use serde::{Deserialize, Serialize};
use sha256::digest;
use std::fmt;

use crate::U256;

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

    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
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
        let hash = Hash::hash(&foo);
        println!("{}", hash);

        assert_eq!(hash.0.bits(), 255);
        assert_eq!(type_name::<_>(hash.0), "btclib::U256")
    }

    fn type_name<T>(_: T) -> &'static str {
        std_type_name::<T>()
    }
}
