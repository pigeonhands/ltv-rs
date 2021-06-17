pub use ltv_derive_impl::*;

#[cfg(test)]
mod tests {
    use ltv::*;

    #[derive(Debug, Ltv, Default, PartialEq, Eq)]
    #[object(id=10, byte_order=BE)]
    struct ExampleStruct {
        #[ltv_field(1)]
        field1: u8,
    }

    #[test]
    fn to_and_from_ltv_obj() {
        let original_ltv = ExampleStruct { field1: 0x69 };
        let ltv_bytes = original_ltv.to_ltv_object();

        println!("{:?}", &ltv_bytes);
        let new_ltv = ExampleStruct::from_ltv_object(&ltv_bytes).unwrap();
        assert_eq!(original_ltv, new_ltv);
    }

    #[test]
    fn to_and_from_ltv() {
        let original_ltv = ExampleStruct { field1: 0x69 };
        let ltv_bytes = original_ltv.to_ltv();

        let new_ltv = ExampleStruct::from_ltv(10, &ltv_bytes).unwrap();
        assert_eq!(original_ltv, new_ltv);
    }

    #[derive(Debug, Ltv, Default, PartialEq, Eq)]
    #[object(id = 10, length_size=1, byte_order=BE)]
    struct LTVObjectExample {
        #[ltv_field(1)]
        field1: u8,
    }
    #[test]
    fn output_test() {
        let my_object_bytes = LTVObjectExample { field1: 55 }.to_ltv_object();
        assert_eq!(
            my_object_bytes,
            vec![
                4,  // Length of object  (length can be 1 or two bytes by setting length_size)
                10, // Outer object ID (LTVObjectExample)
                2,  // Length of Field (field1)
                1,  // Field ID (field1)
                55  //Field Value
            ]
        )
    }
}
