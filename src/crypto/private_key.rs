use anyhow::{anyhow, Result};
use rand::{CryptoRng, RngCore};
use sha2::{Digest, Sha512};

use crate::crypto::edwards25519::extended_group_element::ExtendedGroupElement;

pub const PRIVATE_KEY_LENGTH: usize = 64;
pub const SEED_LENGTH: usize = 32;

#[derive(Copy, Clone, Debug)]
pub struct PrivateKey(pub [u8; PRIVATE_KEY_LENGTH]);

impl PrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<PrivateKey> {
        match bytes.len() {
            SEED_LENGTH => {
                let mut h: Sha512 = Sha512::new();
                let mut hash: [u8; 64] = [0u8; 64];
                let mut digest: [u8; 32] = [0u8; 32];

                h.update(bytes);
                hash.copy_from_slice(h.finalize().as_slice());

                digest.copy_from_slice(&hash[..32]);

                digest[0] &= 248;
                digest[31] &= 127;
                digest[31] |= 64;

                let mut a = ExtendedGroupElement::default();
                a.ge_scalar_mult_base(digest);
                let public_key_bytes = a.to_bytes();

                let merge: Vec<u8> = [bytes.to_vec(), public_key_bytes.to_vec()]
                    .concat()
                    .into_iter()
                    .map(|b| b)
                    .collect();
                let mut bits: [u8; 64] = [0u8; 64];
                bits.copy_from_slice(&merge[..64]);

                Ok(PrivateKey(bits))
            }
            PRIVATE_KEY_LENGTH => {
                let mut bits: [u8; 64] = [0u8; 64];
                bits.copy_from_slice(&bytes[..64]);

                Ok(PrivateKey(bits))
            }
            _ => Err(anyhow!("Invalid secret key length")),
        }
    }

    pub fn generate<T>(r: &mut T) -> PrivateKey
    where
        T: CryptoRng + RngCore,
    {
        let mut secret_key = PrivateKey([0u8; 64]);

        r.fill_bytes(&mut secret_key.0);

        secret_key
    }

    pub fn to_bytes(&self) -> [u8; PRIVATE_KEY_LENGTH] {
        self.0
    }

    pub fn as_bytes<'a>(&'a self) -> &'a [u8; PRIVATE_KEY_LENGTH] {
        &self.0
    }
}

impl ToString for PrivateKey {
    fn to_string(&self) -> String {
        hex::encode(self.0[..32].to_vec())
    }
}
