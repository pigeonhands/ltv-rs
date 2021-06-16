use crate::object::LTVItem;
use crate::ByteOrder;
use std::io;

pub trait LTVContainer<const ED: ByteOrder> {
    fn write_ltv<'a, T>(&mut self, obj_id: usize, obj: &T) -> io::Result<usize>
    where
        T: LTVItem<ED>;
}

impl<W: io::Write, const ED: ByteOrder> LTVContainer<ED> for W {
    fn write_ltv<'a, T: LTVItem<ED>>(&mut self, obj_id: usize, obj: &T) -> io::Result<usize> {
        let mut size: usize = 0;
        let data = obj.to_ltv();
        size += self.write(&[(data.len() as u8) + 1, obj_id as u8])?;
        size += self.write(&data)?;
        Ok(size)
    }
}

pub struct LTVWriter<W: LTVContainer<ED>, const ED: ByteOrder> {
    writer: W,
}

impl<W: LTVContainer<ED>, const ED: ByteOrder> LTVWriter<W, ED> {
    pub fn new(w: W) -> Self {
        Self { writer: w }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W: LTVContainer<ED>, const ED: ByteOrder> LTVContainer<ED> for LTVWriter<W, ED> {
    fn write_ltv<'a, T: LTVItem<ED>>(&mut self, obj_id: usize, obj: &T) -> io::Result<usize> {
        self.writer.write_ltv(obj_id, obj)
    }
}
