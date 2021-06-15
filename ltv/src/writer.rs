use std::io;

use byteorder::ByteOrder;

use crate::object::LTVItem;
use crate::DefaultED;

pub trait LTVContainer<ED: ByteOrder = DefaultED> {
    fn write_ltv<'a, T: LTVItem<'a, ED>>(&mut self, obj_id: usize, obj: &T) -> io::Result<usize>;
}

impl<W: io::Write> LTVContainer for W {
    fn write_ltv<'a, T: LTVItem<'a>>(&mut self, obj_id: usize, obj: &T) -> io::Result<usize> {
        let mut size: usize = 0;
        let data = obj.to_ltv();
        size += self.write(&[(data.len() as u8) + 1, obj_id as u8])?;
        size += self.write(&data)?;
        Ok(size)
    }
}
