use std::{convert::{TryInto}, u128};

use crate::{
    error::{LTVError, LTVResult},
    ByteOrder,
};

pub trait LTVObjectGroup<'a, const ED: ByteOrder>: Sized {
    fn to_ltv(&self) -> Vec<u8>;
    fn from_ltv(data: &'a [u8]) -> Option<Self>;
}

pub trait LTVItem<const ED: ByteOrder>: Sized {
    fn from_ltv(field_type: u8, data: &[u8]) -> LTVResult<Self>;
    fn to_ltv(&self) -> Vec<u8>;

    fn not_found(field_id: u8) -> LTVResult<Self> {
        Err(LTVError::NotFound(field_id))
    }
}


pub trait LTVItemMany<const ED: ByteOrder>: Sized {
    type Item: LTVItem<ED>;
    fn new() -> Self;
    fn add_item(&mut self, v: Self::Item);
    fn get_items<'a>(&'a self) -> Vec<&'a Self::Item>;
}
impl<T: LTVItem<ED>, const ED: ByteOrder> LTVItemMany<ED> for Vec<T> {
    type Item = T;
    fn new() -> Self {
        Vec::new()
    }
    fn add_item(&mut self, v: Self::Item) {
        self.push(v)
    }
    fn get_items<'a>(&'a self) -> Vec<&'a Self::Item> {
        self.iter().collect()
    }
}

pub trait LTVObjectConvertable<'a, const ED: ByteOrder, const LENGTH_BYTE: usize>:
    LTVItem<ED>
{
    fn from_ltv_object(data: &'a [u8]) -> LTVResult<Self>;
    fn to_ltv_object(&self) -> Vec<u8>;

     // LTV without the [L]ength
     fn from_ltv_object_body(data: &'a [u8])-> LTVResult<Self>{
        let obj_id = data[0];
        let body = &data[1..];
        Ok(Self::from_ltv(obj_id, body)?)
    }
}

pub trait LTVObject<const LENGTH_BYTE: usize> {
    const OBJECT_ID: u8;
}

impl<
        'a,
        T: LTVObject<LENGTH_BYTE> + LTVItem<ED>,
        const ED: ByteOrder,
        const LENGTH_BYTE: usize,
    > LTVObjectConvertable<'a, ED, LENGTH_BYTE> for T
{
    fn from_ltv_object(data: &'a [u8]) -> LTVResult<Self> {
        use crate::reader::LTVReader;
        let (_, obj_id, data) = LTVReader::<'a, ED, LENGTH_BYTE>::parse_ltv(data)?;
        Ok(Self::from_ltv(obj_id, data)?)
    }

    fn to_ltv_object(&self) -> Vec<u8> {
        let mut data = self.to_ltv();
        let body_length = data.len() + 1;
        let mut out_ltv = Vec::with_capacity(LENGTH_BYTE + body_length);

        match LENGTH_BYTE {
            1 => out_ltv.push(body_length as u8),
            2 => {
                let lengthu16 = body_length as u16;
                let bytes = match ED {
                    ByteOrder::BE => lengthu16.to_be_bytes(),
                    ByteOrder::LE => lengthu16.to_le_bytes(),
                };
                out_ltv.extend_from_slice(&bytes);
            }
            _ => panic!("Unsupported length byte {}", LENGTH_BYTE),
        };

        out_ltv.push(Self::OBJECT_ID);
        out_ltv.append(&mut data);

        out_ltv
    }
}

impl<'a, const ED: ByteOrder> LTVItem<ED> for () {
    fn from_ltv(field_id: u8, _: &'_ [u8]) -> LTVResult<Self> {
        Err(LTVError::UnexpectedValue(
            field_id,
            format!("`()` value should not exist."),
        ))
    }

    fn to_ltv(&self) -> Vec<u8> {
        Vec::new()
    }

    fn not_found(_: u8) -> LTVResult<Self> {
        Ok(())
    }
}

impl<'a, T: LTVItem<ED>, const ED: ByteOrder> LTVItem<ED> for Option<T> {
    fn from_ltv(field_id: u8, data: &'_ [u8]) -> LTVResult<Self> {
        Ok(Some(T::from_ltv(field_id, data)?))
    }

    fn to_ltv(&self) -> Vec<u8> {
        if let Some(e) = self {
            e.to_ltv()
        } else {
            Vec::new()
        }
    }

    fn not_found(_: u8) -> LTVResult<Self> {
        Ok(None)
    }
}

impl<const ED: ByteOrder> LTVItem<ED> for Vec<u8> {
    fn from_ltv(_field_id: u8, data: &[u8]) -> LTVResult<Self> {
        Ok(Vec::from(data))
    }

    fn to_ltv(&self) -> Vec<u8> {
        self.clone()
    }
}

impl<const ED: ByteOrder, const LENGTH: usize> LTVItem<ED> for [u8; LENGTH] {
    fn from_ltv(field_id: u8, data: &[u8]) -> LTVResult<Self> {
        data.try_into().map_err(|_| LTVError::WrongSize {
            field_id: field_id,
            expected: LENGTH,
            recieved: data.len(),
        })
    }

    fn to_ltv(&self) -> Vec<u8> {
        (self as &[_]).into()
    }
}

macro_rules! impl_numeric_ltvitem {
    ($($i:ident),+) => {
    $(

    impl<const ED: ByteOrder> LTVItem<ED> for $i {
        fn from_ltv(field_id: u8, data: &[u8]) -> LTVResult<Self> {
            let numeric_value= data
                .try_into()
                .and_then(|b| Ok(match ED {
                    ByteOrder::LE=> $i::from_le_bytes(b),
                    ByteOrder::BE=> $i::from_be_bytes(b),
                }));

            match numeric_value {
                Ok(b) => Ok(b),
                Err(_) => Err(LTVError::WrongSize {
                    field_id: field_id,
                    expected: ($i::BITS/8) as usize,
                    recieved: data.len(),
                })
            }
        }
        fn to_ltv(&self) -> Vec<u8> {
            Vec::from(
                match ED {
                    ByteOrder::LE=> $i::to_le_bytes(*self),
                    ByteOrder::BE=> $i::to_be_bytes(*self),
                }
            )
        }
    }

    )*

    };
}

impl_numeric_ltvitem! {
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64, 
    i64,
    u128,
    i128
}
