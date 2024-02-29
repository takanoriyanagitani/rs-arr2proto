use prost::{EncodeError, Message};

use bytes::BufMut;
use prost::bytes;

pub mod array2d {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/array2d.v1.rs"));
    }
}

pub fn msg2buf<M, B>(msg: &M, buf: &mut B) -> Result<(), EncodeError>
where
    M: Message,
    B: BufMut,
{
    msg.encode(buf)
}

pub fn msg2vec<M>(msg: &M) -> Vec<u8>
where
    M: Message,
{
    msg.encode_to_vec()
}

#[cfg(any(doc, target_arch = "wasm32", feature = "float64"))]
pub mod float64;

#[cfg(any(doc, target_arch = "wasm32", feature = "float64"))]
pub mod lib4wasm;
