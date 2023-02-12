use std::hash::Hasher;

use mur3::{Hasher128, murmurhash3_x64_128};
use murmur3::murmur3_x64_128;

pub fn mur3_fn_multi_values(values: &[Vec<u8>]) -> u64 {
    let mut buf = Vec::new();
    for i in 0..values.len() {
        buf.extend_from_slice(&values[i]);
    }
    murmurhash3_x64_128(&buf, 0).0
}

pub fn mur3_hasher_multi_values(values: &[Vec<u8>]) -> u64 {
    let mut hasher = Hasher128::with_seed(0);
    for b in values {
        hasher.write(b);
    }
    hasher.finish()
}

pub fn murmur3_multi_values(values: &[Vec<u8>]) -> u64 {
    let mut buf = Vec::new();
    for i in 0..values.len() {
        buf.extend_from_slice(&values[i]);
    }
    let mut bytes = &buf[..];
    let out = murmur3_x64_128(&mut bytes, 0).unwrap();
    out as u64
}