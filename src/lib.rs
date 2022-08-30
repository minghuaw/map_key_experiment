use rand;

use rand::prelude::*;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::collections::hash_map::{HashMap};
use std::hash::{Hash, Hasher};

pub struct ShortKey<'a>(pub &'a str);

pub trait Key {
    // Return the tuple of the first byte of a key plus the rest of the key
    fn key(&self) -> (Option<u8>, &[u8]);
}

impl<'a> Key for ShortKey<'a> {
    fn key(&self) -> (Option<u8>, &[u8]) {
        // Pretend we are a String with "_" being the first character!
        (Some(b'_'), self.0.as_bytes())
    }
}

impl Key for String {
    fn key(&self) -> (Option<u8>, &[u8]) {
        let bytes = self.as_bytes();
        if bytes.is_empty() {
            (None, b"")
        } else {
            // Split the first byte and return a byte slice
            // corresponding to the rest of the string
            (Some(bytes[0]), &bytes[1..])
        }
    }
}

impl<'a> Borrow<dyn Key + 'a> for String {
    fn borrow(&self) -> &(dyn Key + 'a) {
        self
    }
}

impl<'a> Eq for (dyn Key + 'a) {}

impl<'a> PartialEq for (dyn Key + 'a) {
    fn eq(&self, other: &dyn Key) -> bool {
        self.key() == other.key()
    }
}

impl<'a> Ord for (dyn Key + 'a) {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key().cmp(&other.key())
    }
}

impl<'a> PartialOrd for (dyn Key + 'a) {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key().partial_cmp(&other.key())
    }
}

impl<'a> Hash for (dyn Key + 'a) {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.key() {
            (Some(c), rest) => {
                state.write_u8(c);
                state.write(rest);
                state.write_u8(0xff)
            }
            (None, s) => s.hash(state),
        }
    }
}

pub fn gen_keys(len: usize, key_len: usize) -> Vec<String> {
    let mut vec = Vec::new();
    let mut rng = StdRng::from_seed([0; 32]); // No randomness!
    for _ in 0..len {
        let key = (0..key_len)
            .map(|_| rng.sample(rand::distributions::Alphanumeric))
            .collect::<Vec<u8>>();
        vec.push(String::from_utf8(key).unwrap());
    }
    vec
}

pub fn prepare_hash_map(keys: &[String]) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::default();
    for key in keys {
        map.insert(format!("_{}", key), 1);
        map.insert(key.clone(), 1);
    }
    map
}

pub fn prepare_btree_map(keys: &[String]) -> BTreeMap<String, usize> {
    let mut map: BTreeMap<String, usize> = BTreeMap::default();
    for key in keys {
        map.insert(format!("_{}", key), 1);
        map.insert(key.clone(), 1);
    }
    map
}


pub const LEN: usize = 1000;
pub const KEY_LEN: usize = 10;