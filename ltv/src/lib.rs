#![feature(const_generics)]

mod collection;
mod error;
mod object;
mod reader;
mod writer;

pub use ltv_derive::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ByteOrder {
    BE,
    LE,
}

pub const DEFAULT_ED: ByteOrder = ByteOrder::BE;

pub use error::{LTVError, LTVResult};
pub use object::{LTVItem, LTVItemMany, LTVObject, LTVObjectConvertable, LTVObjectGroup};
pub use reader::LTVReader;
pub use writer::LTVContainer;
pub use writer::LTVWriter;

//Helper types
pub type LTVWriterBE<W, const LENGTH_SIZE: usize> = LTVWriter<W, { ByteOrder::BE }, LENGTH_SIZE>;
pub type LTVWriterLE<W, const LENGTH_SIZE: usize> = LTVWriter<W, { ByteOrder::LE }, LENGTH_SIZE>;

pub type LTVReaderBE<'a, const LENGTH_SIZE: usize> = LTVReader<'a, { ByteOrder::BE }, LENGTH_SIZE>;
pub type LTVReaderLE<'a, const LENGTH_SIZE: usize> = LTVReader<'a, { ByteOrder::LE }, LENGTH_SIZE>;

pub fn get_ltv<T: LTVItem<ED>, const ED: ByteOrder>(obj: &T) -> Vec<u8> {
    obj.to_ltv()
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[derive(Debug, PartialEq, Eq)]
    struct BasicLTV {
        field1: u8,
    }

    impl<'a> LTVItem<{ ByteOrder::BE }> for BasicLTV {
        fn from_ltv(_: u8, data: &[u8]) -> LTVResult<Self> {
            let reader = LTVReaderLE::<1>::new(data);
            Ok(BasicLTV {
                field1: reader.get_item::<u8>(0x01)?,
            })
        }
        fn to_ltv(&self) -> Vec<u8> {
            let mut writer = LTVWriterLE::<_, 1>::new(Vec::with_capacity(3));
            writer.write_ltv(0x01, &self.field1).ok();
            writer.into_inner()
        }
    }
    #[test]
    fn writer_to_reader() {
        let original = BasicLTV { field1: 0x35 };

        let buffer = original.to_ltv();
        let out = BasicLTV::from_ltv(0x01, &buffer).unwrap();
        assert_eq!(original, out);
        assert_eq!(&buffer, &[2, 0x01, 0x35]);
    }
}
