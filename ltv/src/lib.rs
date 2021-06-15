mod reader;
mod object;
mod error;
mod writer;

//use ltv_derive::*;

use byteorder::{BigEndian};
pub type DefaultED = BigEndian;

pub use byteorder;
pub use object::{LTVItem, LTVObject, LTVObjectGroup};
pub use reader::LTVReader;
pub use writer::LTVContainer;
pub use error::{LTVError, LTVResult};

pub mod ed {
    pub use byteorder::{BE, LE};
}



#[cfg(test)]
mod tests {
   use std::u128;

use crate::*;
    
    #[derive(Debug, PartialEq, Eq)]
    struct BasicLTV{
        field1: u8,
    }

    impl<'a> LTVItem<'a, ed::BE> for BasicLTV {
        type Item = BasicLTV;
        fn from_ltv(_: usize, data: &'a [u8]) -> LTVResult<Self::Item> {
            let reader = LTVReader::<ed::BE, 1>::new(data);
            Ok(BasicLTV{
                field1: reader.get_item::<u8>(0x01)?,
            })
        }
        fn to_ltv(&self) -> Vec<u8> {
            let mut buffer = Vec::with_capacity(3);
            buffer.write_ltv(0x01, &self.field1).ok();
            buffer
        }
    }
    #[test]
    fn writer_to_reader() {
        let original = BasicLTV{
            field1: 0x35
        };

        let buffer = original.to_ltv();
        let out = BasicLTV::from_ltv(0x01, &buffer).unwrap();
        assert_eq!(original, out);
        assert_eq!(&buffer, &[2, 0x01, 0x35]);
    }

}

