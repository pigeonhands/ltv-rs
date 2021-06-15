use std::{mem, u128};

use crate::{
    error::{LTVError, LTVResult},
    DefaultED,
};
use byteorder::ByteOrder;

pub trait LTVObjectGroup<'a, ED: ByteOrder = DefaultED>: Sized {
    fn to_ltv(&self) -> Vec<u8>;
    fn from_ltv(data: &'a [u8]) -> Option<Self>;
}

pub trait LTVItem<'a, ED: ByteOrder = DefaultED>: Sized {
    type Item: LTVItem<'a>;
    fn from_ltv(field_type: usize, data: &'a [u8]) -> LTVResult<Self::Item>;
    fn to_ltv(&self) -> Vec<u8>;
}

pub trait LTVObject<'a, ED: ByteOrder, const LENGTH_BYTE: usize>: LTVItem<'a, ED> {
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
            2 => ED::write_u16(&mut out_ltv, (data.len() + 1) as u16),
            _ => panic!("Unsupported length byte {}", LENGTH_BYTE),
        };
        out_ltv.push(object_id);
        out_ltv.append(&mut data);

        out_ltv
    }
}

impl<ED: ByteOrder> LTVItem<'_, ED> for Vec<u8> {
    type Item = Self;
    fn from_ltv(_field_id: usize, data: &[u8]) -> LTVResult<Self> {
        Ok(Vec::from(data))
    }

    fn to_ltv(&self) -> Vec<u8> {
        self.clone()
    }
}

impl<ED: ByteOrder> LTVItem<'_, ED> for u8 {
    type Item = u8;
    fn from_ltv(field_id: usize, data: &[u8]) -> LTVResult<Self> {
        const DATA_SIZE: usize = 1;

        if data.len() == DATA_SIZE {
            Ok(data[0])
        } else {
            Err(LTVError::WrongSize {
                field_id: field_id,
                expected: DATA_SIZE,
                recieved: data.len(),
            })
        }
    }

    fn to_ltv(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl<ED: ByteOrder> LTVItem<'_, ED> for i8 {
    type Item = i8;
    fn from_ltv(field_id: usize, data: &[u8]) -> LTVResult<Self> {
        const DATA_SIZE: usize = 1;

        if data.len() == DATA_SIZE {
            Ok(data[0] as i8)
        } else {
            Err(LTVError::WrongSize {
                field_id: field_id,
                expected: DATA_SIZE,
                recieved: data.len(),
            })
        }
    }

    fn to_ltv(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

impl<ED: ByteOrder> LTVItem<'_, ED> for u16 {
    type Item = u16;
    fn from_ltv(field_id: usize, data: &[u8]) -> LTVResult<Self> {
        const DATA_SIZE: usize = 2;

        if data.len() == DATA_SIZE {
            Ok(ED::read_u16(data))
        } else {
            Err(LTVError::WrongSize {
                field_id: field_id,
                expected: DATA_SIZE,
                recieved: data.len(),
            })
        }
    }

    fn to_ltv(&self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(mem::size_of::<Self>());
        ED::write_u16(&mut buff, *self);
        buff
    }
}

impl<ED: ByteOrder> LTVItem<'_, ED> for i16 {
    type Item = i16;
    fn from_ltv(field_id: usize, data: &[u8]) -> LTVResult<Self> {
        const DATA_SIZE: usize = 2;

        if data.len() == DATA_SIZE {
            Ok(ED::read_i16(data))
        } else {
            Err(LTVError::WrongSize {
                field_id: field_id,
                expected: DATA_SIZE,
                recieved: data.len(),
            })
        }
    }

    fn to_ltv(&self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(mem::size_of::<Self>());
        ED::write_i16(&mut buff, *self);
        buff
    }
}

impl<ED: ByteOrder> LTVItem<'_, ED> for u32 {
    type Item = u32;
    fn from_ltv(field_id: usize, data: &[u8]) -> LTVResult<Self> {
        const DATA_SIZE: usize = 4;

        if data.len() == DATA_SIZE {
            Ok(ED::read_u32(data))
        } else {
            Err(LTVError::WrongSize {
                field_id: field_id,
                expected: DATA_SIZE,
                recieved: data.len(),
            })
        }
    }

    fn to_ltv(&self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(mem::size_of::<Self>());
        ED::write_u32(&mut buff, *self);
        buff
    }
}

impl<ED: ByteOrder> LTVItem<'_, ED> for i32 {
    type Item = i32;
    fn from_ltv(field_id: usize, data: &[u8]) -> LTVResult<Self> {
        const DATA_SIZE: usize = 4;

        if data.len() == DATA_SIZE {
            Ok(ED::read_i32(data))
        } else {
            Err(LTVError::WrongSize {
                field_id: field_id,
                expected: DATA_SIZE,
                recieved: data.len(),
            })
        }
    }

    fn to_ltv(&self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(mem::size_of::<Self>());
        ED::write_i32(&mut buff, *self);
        buff
    }
}

impl<ED: ByteOrder> LTVItem<'_, ED> for u128 {
    type Item = u128;
    fn from_ltv(field_id: usize, data: &[u8]) -> LTVResult<Self> {
        const DATA_SIZE: usize = 16;

        if data.len() == DATA_SIZE {
            Ok(ED::read_u128(data))
        } else {
            Err(LTVError::WrongSize {
                field_id: field_id,
                expected: DATA_SIZE,
                recieved: data.len(),
            })
        }
    }

    fn to_ltv(&self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(mem::size_of::<Self>());
        ED::write_u128(&mut buff, *self);
        buff
    }
}

impl<ED: ByteOrder> LTVItem<'_, ED> for i128 {
    type Item = i128;
    fn from_ltv(field_id: usize, data: &[u8]) -> LTVResult<Self> {
        const DATA_SIZE: usize = 16;

        if data.len() == DATA_SIZE {
            Ok(ED::read_i128(data))
        } else {
            Err(LTVError::WrongSize {
                field_id: field_id,
                expected: DATA_SIZE,
                recieved: data.len(),
            })
        }
    }

    fn to_ltv(&self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(mem::size_of::<Self>());
        ED::write_i128(&mut buff, *self);
        buff
    }
}

impl<ED: ByteOrder> LTVItem<'_, ED> for f32 {
    type Item = f32;
    fn from_ltv(field_id: usize, data: &[u8]) -> LTVResult<Self> {
        const DATA_SIZE: usize = 4;

        if data.len() == DATA_SIZE {
            Ok(ED::read_f32(data))
        } else {
            Err(LTVError::WrongSize {
                field_id: field_id,
                expected: DATA_SIZE,
                recieved: data.len(),
            })
        }
    }

    fn to_ltv(&self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(mem::size_of::<Self>());
        ED::write_f32(&mut buff, *self);
        buff
    }
}
