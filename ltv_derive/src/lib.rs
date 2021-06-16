pub use ltv_derive_impl::*;

#[cfg(test)]
mod tests {
    use ltv::*;

    #[derive(Debug, Ltv, Default, PartialEq, Eq)]
    #[object_id = 10]
    struct ExampleStruct {
        #[ltv_field(1)]
        field1: u8,
    }

    #[test]
    fn to_and_from_ltv() {
        let original_ltv = ExampleStruct { field1: 0x69 };
        let ltv_bytes = original_ltv.to_ltv_object();

        println!("{:?}", &ltv_bytes);
        let new_ltv = ExampleStruct::from_ltv_object(&ltv_bytes).unwrap();
        assert_eq!(original_ltv, new_ltv);
    }
}
