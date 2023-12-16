use elrond_codec::{
    DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
    NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};
use num_bigint::BigUint as BigUintLib;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct BigUint(BigUintLib);

impl From<BigUint> for BigUintLib {
    fn from(my_big_uint: BigUint) -> Self {
        my_big_uint.0
    }
}

impl From<BigUintLib> for BigUint {
    fn from(big_uint: BigUintLib) -> Self {
        BigUint(big_uint)
    }
}

impl TopEncode for BigUint {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.to_bytes_be().top_encode_or_handle_err(output, h)
    }
}

impl TopDecode for BigUint {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let bytes = Vec::<u8>::top_decode_or_handle_err(input, h)?;
        Ok(BigUint::from(BigUintLib::from_bytes_be(
            bytes.as_slice(),
        )))
    }
}

impl NestedEncode for BigUint {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.0.to_bytes_be().dep_encode_or_handle_err(dest, h)
    }
}

impl NestedDecode for BigUint {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let bytes = Vec::<u8>::dep_decode_or_handle_err(input, h)?;
        Ok(BigUint::from(BigUintLib::from_bytes_be(
            bytes.as_slice(),
        )))
    }
}
