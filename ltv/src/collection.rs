pub trait LtvObjectCollection {
    fn from_ltv_object(object_id: usize, data: &[u8]) -> Self;
}
