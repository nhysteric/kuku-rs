use crate::{ItemType, LocaltionType};
use blake3::Hasher;

const ARRAY_SIZE: usize = 4096;
#[derive(Debug, Clone, PartialEq)]
pub struct HashFunc {
    random_array: [LocaltionType; ARRAY_SIZE],
}

impl HashFunc {
    pub fn new(seed: u128) -> HashFunc {
        let mut result: [u32; ARRAY_SIZE] = [0; ARRAY_SIZE];
        let mut hasher = Hasher::new();
        let mut i = 0;
        while i < ARRAY_SIZE {
            hasher.update(&seed.to_ne_bytes());
            let hash_output = hasher.finalize();
            let hash_bytes = hash_output.as_bytes();
            for chunk in hash_bytes.chunks_exact(4) {
                result[i] = u32::from_ne_bytes(chunk.try_into().unwrap());
                i += 1;
            }
        }
        HashFunc {
            random_array: result,
        }
    }
    pub fn hash(&self, item: &ItemType) -> LocaltionType {
        let mut result: LocaltionType = 0;
        for (i, &byte) in item.iter().enumerate().take(16) {
            result ^= self.random_array[i * 256 + byte as usize];
        }
        result
    }
}
