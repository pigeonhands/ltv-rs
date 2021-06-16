#![feature(const_generics)]

mod error;
mod object;
mod reader;
mod sets;
mod writer;

pub use ltv_derive::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ByteOrder {
    BE,
    LE,
}

pub const DEFAULT_ED: ByteOrder = ByteOrder::BE;

pub use error::{LTVError, LTVResult};
pub use object::{LTVItem, LTVObject, LTVObjectGroup};
pub use reader::LTVReader;
pub use sets::LtvObjectSet;
pub use writer::LTVContainer;
pub use writer::LTVWriter;

//Helper types
pub type LTVWriterBE<W> = LTVWriter<W, { ByteOrder::BE }>;
pub type LTVWriterLE<W> = LTVWriter<W, { ByteOrder::LE }>;

pub type LTVReaderBE<'a, const LENGTH_SIZE: usize> = LTVReader<'a, { ByteOrder::BE }, LENGTH_SIZE>;
pub type LTVReaderLE<'a, const LENGTH_SIZE: usize> = LTVReader<'a, { ByteOrder::LE }, LENGTH_SIZE>;

#[cfg(test)]
mod tests {

    use crate::*;

    #[derive(Debug, PartialEq, Eq)]
    struct BasicLTV {
        field1: u8,
    }

    impl<'a> LTVItem<{ ByteOrder::BE }> for BasicLTV {
        type Item = BasicLTV;
        fn from_ltv(_: usize, data: &[u8]) -> LTVResult<Self::Item> {
            let reader = LTVReaderLE::<1>::new(data);
            Ok(BasicLTV {
                field1: reader.get_item::<u8>(0x01)?,
            })
        }
        fn to_ltv(&self) -> Vec<u8> {
            let mut writer = LTVWriterLE::new(Vec::with_capacity(3));
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
