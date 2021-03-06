use std::marker::PhantomData;

use crate::{
    error::{LTVError, LTVResult},
    ByteOrder, LTVItem, LTVItemMany,
};

pub struct LTVFieldIterator<'a, T: LTVItem<ED>, const ED: ByteOrder, const LENGTH_SIZE: usize> {
    _marker: PhantomData<T>,
    body: &'a [u8],
    i: usize,
}
impl<'a, T: LTVItem<ED>, const ED: ByteOrder, const LENGTH_SIZE: usize>
    LTVFieldIterator<'a, T, ED, LENGTH_SIZE>
{
    pub fn new(body: &'a [u8]) -> Self {
        Self {
            _marker: PhantomData::default(),
            body: body,
            i: 0,
        }
    }
}
impl<'a, T: LTVItem<ED>, const ED: ByteOrder, const LENGTH_SIZE: usize> Iterator
    for LTVFieldIterator<'a, T, ED, LENGTH_SIZE>
{
    type Item = LTVResult<T>;
    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.body.len() {
            let (length, ltv_id, data) =
                match LTVReader::<ED, LENGTH_SIZE>::parse_ltv(&self.body[self.i..]){
                    Ok(v) => v,
                    Err(e) => return Some(Err(e))
                };
            self.i += length;
            match T::from_ltv(ltv_id, data) {
                Ok(o) => return Some(Ok(o)),
                Err(e) => return Some(Err(e)),
            };
        }

        None
    }
}

pub struct LTVFieldBinary {
    pub field_id: u8,
    pub data: Vec<u8>,
}

impl<'a, const ED: ByteOrder> LTVItem<ED> for LTVFieldBinary {
    fn from_ltv(field_id: u8, data: &[u8]) -> LTVResult<Self> {
        Ok(Self {
            field_id,
            data: Vec::from(data),
        })
    }
    fn to_ltv(&self) -> Vec<u8> {
        unimplemented!()
    }
}

pub struct LTVReader<'a, const ED: ByteOrder, const LENGTH_SIZE: usize> {
    body: &'a [u8],
}

impl<'a, const ED: ByteOrder, const LENGTH_SIZE: usize> LTVReader<'a, ED, LENGTH_SIZE> {
    /// create a new reader from a body of an object (V)
    // [ L ] [ T ] [    V     ]
    // [04]  [01]   [02 01 FF]
    pub fn new(body: &'a [u8]) -> Self {
        Self { body }
    }

    pub fn iter<T: LTVItem<ED>>(&self) -> LTVFieldIterator<'a, T, ED, LENGTH_SIZE> {
        LTVFieldIterator::new(self.body)
    }
    pub fn get_item_optional<T: LTVItem<ED>>(&self, field_id: u8) -> LTVResult<Option<T>> {
        for o in self.iter::<LTVFieldBinary>() {
            let binary_field = o?;

            if binary_field.field_id == field_id {
                let o = T::from_ltv(field_id, &binary_field.data)?;
                return Ok(Some(o));
            }
        }

        Ok(None)
    }

    pub fn get_item<T: LTVItem<ED>>(&self, field_id: u8) -> LTVResult<T> {
        match self.get_item_optional::<T>(field_id)? {
            Some(o) => Ok(o),
            None => return T::not_found(field_id),
        }
    }

    pub fn get_many<T: LTVItem<ED>, M: LTVItemMany<ED>>(&self, field_id: u8) -> LTVResult<M> {
        let mut v = M::new();
        for o in self.iter::<LTVFieldBinary>() {
            let binary_field = o?;

            if binary_field.field_id == field_id {
                let o = M::Item::from_ltv(field_id, &binary_field.data)?;
                v.add_item(o);
            }
        }
        Ok(v)
    }

    pub fn parse_ltv<'b>(data: &'b [u8]) -> LTVResult<(usize, u8, &'b [u8])> {
        if data.len() < LENGTH_SIZE {
            return Err(LTVError::WrongSize {
                field_id: 0,
                expected: LENGTH_SIZE,
                recieved: data.len(),
            });
        }
        let body_length = match LENGTH_SIZE {
            1 => <u8 as LTVItem<ED>>::from_ltv(0, &data[..1])? as usize,
            2 => <u16 as LTVItem<ED>>::from_ltv(0, &data[..2])? as usize,
            _ => panic!("Unsuppoted length size {}", LENGTH_SIZE),
        };
        if body_length < 1 {
            return Ok((LENGTH_SIZE, 0, &[]));
        }
        let data_length = body_length-1;
        
        let header_size = LENGTH_SIZE + 1;
        
        let expected_length = data_length + header_size;
        if data.len() < expected_length {
            return Err(LTVError::WrongSize {
                field_id: 0,
                expected: expected_length,
                recieved: data.len(),
            });
        }
        let field_type = data[LENGTH_SIZE];
        
        let ltv_data = &data[header_size..header_size + data_length];

        Ok((expected_length, field_type, ltv_data))
    }

    pub fn get_field<T: LTVItem<ED>>(data: &'a [u8], field_id: u8) -> LTVResult<T> {
        let r = Self::new(data);
        match r.get_item_optional::<T>(field_id)? {
            Some(o) => Ok(o),
            None => return T::not_found(field_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic_reader() {
        let input_data: &[u8] = &[0x04, 0x01, 0x02, 0x01, 0xFF];
        let reader = LTVReader::<DEFAULT_ED, 1>::new(&input_data[2..]);

        let field_1 = reader.get_item::<u8>(0x1).unwrap();
        assert_eq!(field_1, 0xFF);
    }

    #[derive(Debug, PartialEq, Eq)]
    struct InnerStructData {
        field1: u8,
        field2: u16,
    }
    impl<const ED: ByteOrder> LTVItem<ED> for InnerStructData {
        fn from_ltv(_field_id: u8, data: &[u8]) -> LTVResult<Self> {
            let reader = LTVReader::<ED, 1>::new(&data);

            Ok(InnerStructData {
                field1: reader.get_item::<u8>(0x1)?,
                field2: reader.get_item::<u16>(0x2)?,
            })
        }

        fn to_ltv(&self) -> Vec<u8> {
            unimplemented!()
        }
    }

    #[test]
    fn basic_inner_struct_reader() {
        let input_data: &[u8] = &[
            0x02, 0x01, 0xFF, 0x08, 0x02, 0x02, 0x01, 0x55, 0x03, 0x02, 0x01, 0x00,
        ];
        //let input_data: &[u8] = &[ 0x02, 0x01, 0xFF];
        let reader = LTVReaderLE::<1>::new(&input_data[..]);

        let field_1 = reader.get_item::<u8>(0x1).unwrap();
        assert_eq!(field_1, 0xFF);

        let field_2 = reader.get_item::<InnerStructData>(0x2).unwrap();
        assert_eq!(
            field_2,
            InnerStructData {
                field1: 0x55,
                field2: 0x0001
            }
        );
    }
}
