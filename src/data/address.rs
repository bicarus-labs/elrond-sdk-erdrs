use anyhow::Result;
use bech32::{FromBase32, ToBase32, Variant};

use crate::crypto::public_key::PublicKey;

pub struct Address([u8; 32]);

impl Address {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn from_bech32_string(bech32: String) -> Result<Self> {
        let (_, data, _) = bech32::decode(bech32.as_str())?;
        let data = Vec::<u8>::from_base32(&data)?;

        let mut bits: [u8; 32] = [0u8; 32];
        bits.copy_from_slice(&data);

        Ok(Self(bits))
    }

    pub fn to_bech32_string(&self) -> Result<String> {
        let address = bech32::encode("erd", self.0.to_base32(), Variant::Bech32)?;
        Ok(address)
    }

    pub fn is_valid(&self) -> bool {
        self.0.len() == 32
    }
}

impl<'a> From<&'a PublicKey> for Address {
    fn from(public_key: &PublicKey) -> Address {
        let bytes = public_key.to_bytes();

        let mut bits: [u8; 32] = [0u8; 32];
        bits.copy_from_slice(&bytes);

        Address(bits)
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        self.to_bech32_string().unwrap()
    }
}
