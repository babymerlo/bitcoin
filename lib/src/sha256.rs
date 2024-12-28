use std::fmt;

use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::U256;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
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

    pub fn zero() -> Self {
        Hash(U256::zero())
    }

    pub fn as_bytes(&self) -> [u8; 32] {
        let mut bytes = vec![0; 32];
        self.0.to_little_endian(&mut bytes);
        bytes.as_slice().try_into().unwrap()
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl fmt::LowerHex for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::LowerHex::fmt(&val, f)
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
        assert_eq!(type_name::<_>(hash.0), "btclib::U256");
    }

    #[test]
    fn zero() {
        assert!(Hash::zero().0.is_zero(), "is zero");
    }

    fn type_name<T>(_: T) -> &'static str {
        std_type_name::<T>()
    }
}
