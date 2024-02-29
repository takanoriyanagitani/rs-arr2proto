use prost::{DecodeError, EncodeError, Message};

use bytes::{Buf, BufMut};
use prost::bytes;

use crate::array2d::v1::Float64;

/// [`Float64`] -> [`BufMut`]
pub fn f2buf64<B>(msg: &Float64, buf: &mut B) -> Result<(), EncodeError>
where
    B: BufMut,
{
    msg.encode(buf)
}

/// [`Float64`] -> [`Vec<u8>`]
pub fn f2vec64(msg: &Float64) -> Vec<u8> {
    msg.encode_to_vec()
}

/// [`Buf`] -> [`Float64`]
pub fn buf2f64<B>(buf: B) -> Result<Float64, DecodeError>
where
    B: Buf,
{
    Float64::decode(buf)
}

pub fn f64merge<B>(buf: B, f: &mut Float64) -> Result<(), DecodeError>
where
    B: Buf,
{
    f.merge(buf)
}

pub fn f64merge_slice(s: &[u8], f: &mut Float64) -> Result<(), DecodeError> {
    f64merge(s, f)
}

/// `&[u8]` -> [`Float64`]
pub fn slice2f64(s: &[u8]) -> Result<Float64, DecodeError> {
    buf2f64(s)
}

#[cfg(any(doc, target_arch = "wasm32"))]
pub mod wasm32f64 {
    use std::collections::BTreeMap;
    use std::sync::RwLock;

    use prost::Message;
    use prost_types::Value;

    use crate::array2d::v1::Meta;

    use crate::float64::Float64;

    static DECODED: RwLock<Option<Float64>> = RwLock::new(None);

    static SERIALIZED: RwLock<Option<Vec<u8>>> = RwLock::new(None);

    pub fn f64write<T, F>(f: F) -> Result<T, &'static str>
    where
        F: Fn(&mut Float64) -> Result<T, &'static str>,
    {
        crate::lib4wasm::write_opt(&DECODED, f)
    }

    pub fn f64write_init<T, F, I>(f: F, init: I) -> Result<T, &'static str>
    where
        F: Fn(&mut Float64) -> Result<T, &'static str>,
        I: Fn() -> Float64,
    {
        crate::lib4wasm::write_opt_init(&DECODED, f, init)
    }

    pub fn f64write_meta<T, F>(f: F) -> Result<T, &'static str>
    where
        F: Fn(&mut Meta) -> Result<T, &'static str>,
    {
        f64write(|mf: &mut Float64| {
            let om: &mut Option<Meta> = &mut mf.meta;
            match om {
                None => Err("no meta"),
                Some(m) => {
                    let rm: &mut Meta = m;
                    f(rm)
                }
            }
        })
    }

    pub fn f64read_serialized<T, F>(f: F) -> Result<T, &'static str>
    where
        F: Fn(&Vec<u8>) -> Result<T, &'static str>,
    {
        crate::lib4wasm::read_opt(&SERIALIZED, f)
    }

    pub fn f64read<T, F>(f: F) -> Result<T, &'static str>
    where
        F: Fn(&Float64) -> Result<T, &'static str>,
    {
        crate::lib4wasm::read_opt(&DECODED, f)
    }

    pub fn f64read_data<T, F>(f: F) -> Result<T, &'static str>
    where
        F: Fn(&[f64]) -> Result<T, &'static str>,
    {
        f64read(|f6: &Float64| {
            let data: &[f64] = &f6.data;
            f(data)
        })
    }

    pub fn f64read_meta<T, F>(f: F) -> Result<T, &'static str>
    where
        F: Fn(&Meta) -> Result<T, &'static str>,
    {
        f64read(|fdat: &Float64| {
            let om: Option<&Meta> = fdat.meta.as_ref();
            let m: &Meta = om.ok_or("no data")?;
            f(m)
        })
    }

    pub fn f64read_meta_by_key<T, F>(f: F) -> Result<T, &'static str>
    where
        F: Fn(&BTreeMap<String, Value>) -> Result<T, &'static str>,
    {
        f64read_meta(|m: &Meta| {
            let raw: &prost_types::Struct = m.raw.as_ref().ok_or("no raw meta info")?;
            let fields: &BTreeMap<String, Value> = &raw.fields;
            f(fields)
        })
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub fn f64clear_decoded() -> i32 {
        f64write(|f: &mut Float64| {
            f.clear();
            let dat: &Vec<f64> = &f.data;
            dat.capacity().try_into().map_err(|_| "invalid capacity")
        })
        .ok()
        .unwrap_or(-1)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub fn f64serialized_offset() -> *const u8 {
        crate::lib4wasm::read_opt(&SERIALIZED, |v: &Vec<u8>| Ok(v.as_ptr()))
            .ok()
            .unwrap_or_else(std::ptr::null)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub fn f64serialize_init() -> i32 {
        f64read(|inp: &Float64| {
            crate::lib4wasm::write_opt_init(
                &SERIALIZED,
                |v: &mut Vec<u8>| v.len().try_into().map_err(|_| "invalid size"),
                || Vec::with_capacity(inp.encoded_len()),
            )
        })
        .ok()
        .unwrap_or(-1)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub fn f64serialize() -> i32 {
        f64read(|inp: &Float64| {
            crate::lib4wasm::write_opt_init(
                &SERIALIZED,
                |v: &mut Vec<u8>| {
                    crate::float64::f2buf64(inp, v)
                        .map_err(|_| "unable to serialize Float64")
                        .and_then(|_| v.len().try_into().map_err(|_| "invalid size"))
                },
                || Vec::with_capacity(inp.encoded_len()),
            )
        })
        .ok()
        .unwrap_or(-1)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub fn f64sum() -> f64 {
        f64read_data(|s: &[f64]| Ok(s.iter().fold(0.0, |tot, next| tot + next)))
            .ok()
            .unwrap_or(f64::NAN)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub fn f64resize(width: i32, height: i32, init: f64) -> i32 {
        f64write_init(
            |f: &mut Float64| {
                let w: usize = width.try_into().map_err(|_| "invalid width")?;
                let h: usize = height.try_into().map_err(|_| "invalid height")?;
                let sz: usize = w * h;
                f.data.resize(sz, init);
                f.data.capacity().try_into().map_err(|_| "invalid size")
            },
            || {
                let w: usize = width.try_into().ok().unwrap_or_default();
                let h: usize = height.try_into().ok().unwrap_or_default();
                Float64 {
                    meta: Some(Meta {
                        raw: Some(prost_types::Struct {
                            fields: BTreeMap::new(),
                        }),
                        width: width.try_into().ok().unwrap_or_default(),
                        height: height.try_into().ok().unwrap_or_default(),
                    }),
                    data: Vec::with_capacity(w * h),
                }
            },
        )
        .ok()
        .unwrap_or(-1)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub fn f64decode(capacity: i32) -> i32 {
        f64read_serialized(|v: &Vec<u8>| {
            f64write_init(
                |f: &mut Float64| {
                    let s: &[u8] = v;
                    crate::float64::f64merge_slice(s, f)
                        .map_err(|_| "unable to serialize")
                        .and_then(|_| f.encoded_len().try_into().map_err(|_| "invalid len"))
                },
                || Float64 {
                    meta: Some(Meta::default()),
                    data: Vec::with_capacity(capacity.try_into().ok().unwrap_or_default()),
                },
            )
        })
        .ok()
        .unwrap_or(-1)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub fn f64set_dim(width: i64, height: i64) -> bool {
        f64write_meta(|m: &mut Meta| {
            let w: u64 = width.try_into().map_err(|_| "invalid width")?;
            let h: u64 = height.try_into().map_err(|_| "invalid height")?;
            m.width = w;
            m.height = h;
            Ok(())
        })
        .ok()
        .map(|_: ()| true)
        .unwrap_or(false)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub fn f64data_offset() -> *const f64 {
        f64read_data(|s: &[f64]| Ok(s.as_ptr()))
            .ok()
            .unwrap_or_else(std::ptr::null)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub fn f64data_count() -> i32 {
        f64read_data(|s: &[f64]| s.len().try_into().map_err(|_| "invalid size"))
            .ok()
            .unwrap_or(-1)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub fn f64height() -> i64 {
        f64read_meta(|m: &Meta| {
            let h: u64 = m.height;
            let i: i64 = h.try_into().map_err(|_| "invalid height")?;
            Ok(i)
        })
        .ok()
        .unwrap_or(-1)
    }

    #[allow(unsafe_code)]
    #[no_mangle]
    pub fn f64width() -> i64 {
        f64read_meta(|m: &Meta| {
            let w: u64 = m.width;
            let i: i64 = w.try_into().map_err(|_| "invalid width")?;
            Ok(i)
        })
        .ok()
        .unwrap_or(-1)
    }
}
