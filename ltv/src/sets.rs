pub trait LtvObjectSet {
    fn ltv_object(data: &[u8]) -> Self;
    fn ltv_bytes() -> Vec<u8>;
}
