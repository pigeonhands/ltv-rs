use std::{convert::TryInto, u128};

use crate::{
    error::{LTVError, LTVResult},
    ByteOrder,
};

pub trait LTVObjectGroup<'a, const ED: ByteOrder>: Sized {
    fn to_ltv(&self) -> Vec<u8>;
    fn from_ltv(data: &'a [u8]) -> Option<Self>;
}

pub trait LTVItem<const ED: ByteOrder> {
    type Item: LTVItem<ED>;
    fn from_ltv(field_type: usize, data: &[u8]) -> LTVResult<Self::Item>;
    fn to_ltv(&self) -> Vec<u8>;
}

pub trait LTVObject<'a, const ED: ByteOrder, const LENGTH_BYTE: usize>: LTVItem<ED> {
    fn from_ltv_object(data: &'a [u8]) -> LTVResult<Self::Item> {
        use crate::reader::LTVReader;
        let (_, obj_id, data) = LTVReader::<'a, ED, LENGTH_BYTE>::parse_ltv(data)?;
        Ok(Self::from_ltv(obj_id, data)?)
    }

    fn to_ltv_object(&self, object_id: u8) -> Vec<u8> {
        let mut data = self.to_ltv();
        let mut out_ltv = Vec::with_capacity(LENGTH_BYTE + 1);

        match LENGTH_BYTE {
            1 => out_ltv.push((data.len() + 1) as u8),
            2 => {
                let lengthu16 = (data.len() + 1) as u16;
                let bytes = match ED {
                    ByteOrder::BE => lengthu16.to_be_bytes(),
                    ByteOrder::LE => lengthu16.to_le_bytes(),
                };
                out_ltv.extend_from_slice(&bytes);
            }
            _ => panic!("Unsupported length byte {}", LENGTH_BYTE),
        };

        //ED::write_u16(&mut out_ltv, (data.len() + 1) as u16),
        out_ltv.push(object_id);
        out_ltv.append(&mut data);

        out_ltv
    }
}

impl<'a, T: LTVItem<ED>, const ED: ByteOrder> LTVItem<ED> for Option<T> {
    type Item = Option<T::Item>;
    fn from_ltv(_field_id: usize, _data: &'_ [u8]) -> LTVResult<Self::Item> {
        todo!()
        /*
        let _o = T::from_ltv(field_id, data);
        Ok(None)
         */
    }

    fn to_ltv(&self) -> Vec<u8> {
        Vec::new()
    }
}

impl<const ED: ByteOrder> LTVItem<ED> for Vec<u8> {
    type Item = Self;
    fn from_ltv(_field_id: usize, data: &[u8]) -> LTVResult<Self> {
        Ok(Vec::from(data))
    }

    fn to_ltv(&self) -> Vec<u8> {
        self.clone()
    }
}

macro_rules! impl_numeric_ltvitem {
    ($($i:ident),+) => {
    $(

    impl<const ED: ByteOrder> LTVItem<ED> for $i {
        type Item = $i;
        fn from_ltv(field_id: usize, data: &[u8]) -> LTVResult<Self> {
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
    u128,
    i128
}

/*
impl LTVItem<'_,  {ByteOrder::BE}> for u8 {
    type Item = u8;
    fn from_ltv(field_id: usize, data: &[u8]) -> LTVResult<Self> {
        match data.try_into().and_then(|b| Ok(u8::from_be_bytes(b))){
            Ok(b) => Ok(b),
            Err(e) => Err(LTVError::WrongSize {
                field_id: field_id,
                expected: (u8::BITS/8) as usize,
                recieved: data.len(),
            })
        }
    }

    fn to_ltv(&self) -> Vec<u8> {
        Vec::from(u8::to_le_bytes(*self))
    }
}

impl LTVItem<'_,  {ByteOrder::LE}> for u8 {
    type Item = u8;
    fn from_ltv(field_id: usize, data: &[u8]) -> LTVResult<Self> {
        match data.try_into().and_then(|b| Ok(u8::from_le_bytes(b))){
            Ok(b) => Ok(b),
            Err(e) => Err(LTVError::WrongSize {
                field_id: field_id,
                expected: (u8::BITS/8) as usize,
                recieved: data.len(),
            })
        }
    }

    fn to_ltv(&self) -> Vec<u8> {
        Vec::from(u8::to_le_bytes(*self))
    }
}

impl<const ED: ByteOrder> LTVItem<'_,  ED> for u8 {
    type Item = u8;
    fn from_ltv(field_id: usize, data: &[u8]) -> LTVResult<Self> {
        let numeric_value= data
            .try_into()
            .and_then(|b| Ok(match ED {
                ByteOrder::LE=> u8::from_le_bytes(b),
                ByteOrder::BE=> u8::from_be_bytes(b),
            }));

        match numeric_value {
            Ok(b) => Ok(b),
            Err(e) => Err(LTVError::WrongSize {
                field_id: field_id,
                expected: (u8::BITS/8) as usize,
                recieved: data.len(),
            })
        }
    }
    fn to_ltv(&self) -> Vec<u8> {
        Vec::from(
            match ED {
                ByteOrder::LE=> u8::to_le_bytes(*self),
                ByteOrder::BE=> u8::to_be_bytes(*self),
            }
        )
    }
}

*/
