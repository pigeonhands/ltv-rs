use crate::object::LTVItem;
use crate::ByteOrder;
use std::io;

pub trait LTVContainer<const ED: ByteOrder, const LENGTH_SIZE: usize> {
    fn write_ltv<'a, T>(&mut self, obj_id: usize, obj: &T) -> io::Result<usize>
    where
        T: LTVItem<ED>;
}

impl<W: io::Write, const ED: ByteOrder, const LENGTH_SIZE: usize> LTVContainer<ED, LENGTH_SIZE>
    for W
{
    fn write_ltv<'a, T: LTVItem<ED>>(&mut self, obj_id: usize, obj: &T) -> io::Result<usize> {
        let data = obj.to_ltv();
        if data.len() == 0 {
            return Ok(0);
        }

        let mut size: usize = 0;
        
        match LENGTH_SIZE {
            1 => size += self.write(&[(data.len() as u8) + 1, obj_id as u8])?,
            2 => {
                size += self.write(&match ED {
                    ByteOrder::LE => (data.len() as u16).to_le_bytes(),
                    ByteOrder::BE => (data.len() as u16).to_be_bytes(),
                })?
            }
            _ => panic!("Unsuppoted length size {}", LENGTH_SIZE),
        }

        size += self.write(&data)?;
        Ok(size)
    }
}

pub struct LTVWriter<
    W: LTVContainer<ED, LENGTH_SIZE>,
    const ED: ByteOrder,
    const LENGTH_SIZE: usize,
> {
    writer: W,
}

impl<W: LTVContainer<ED, LENGTH_SIZE>, const ED: ByteOrder, const LENGTH_SIZE: usize>
    LTVWriter<W, ED, LENGTH_SIZE>
{
    pub fn new(w: W) -> Self {
        Self { writer: w }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W: LTVContainer<ED, LENGTH_SIZE>, const ED: ByteOrder, const LENGTH_SIZE: usize>
    LTVContainer<ED, LENGTH_SIZE> for LTVWriter<W, ED, LENGTH_SIZE>
{
    fn write_ltv<'a, T: LTVItem<ED>>(&mut self, obj_id: usize, obj: &T) -> io::Result<usize> {
        self.writer.write_ltv(obj_id, obj)
    }
}
