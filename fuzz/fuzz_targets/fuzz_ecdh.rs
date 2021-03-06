#![no_main]
use libfuzzer_sys::fuzz_target;

extern crate secp256k1zkp;

use secp256k1zkp::{Secp256k1, PublicKey, SecretKey};
use secp256k1zkp::ecdh::SharedSecret;

fuzz_target!(|data: &[u8]| {
    if data.len() < 32 {
        return ();
    }

    let s = Secp256k1::new();

    if let Ok(sk) = SecretKey::from_slice(&s, &data[..32]) {
        match PublicKey::from_secret_key(&s, &sk) {
            Ok(pk) => { let _ = SharedSecret::new(&s, &pk, &sk); () },
            Err(e) => panic!("cannot create public key from secret: {}", e),
        }
    }
});

