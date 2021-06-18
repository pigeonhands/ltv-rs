pub use ltv_derive_impl::*;

#[cfg(test)]
mod tests {
    use ltv::*;

    #[derive(Debug, Ltv, Default, PartialEq, Eq)]
    struct ExampleStruct {
        #[ltv_field(1)]
        field1: u8,

        #[ltv_field(2)]
        field2: [u8;3],
    }

   

    #[test]
    fn to_and_from_ltv() {
        let original_ltv = ExampleStruct { field1: 0x69, field2: [12,34,56] };
        let ltv_bytes = original_ltv.to_ltv();

        let new_ltv = ExampleStruct::from_ltv(10, &ltv_bytes).unwrap();
        assert_eq!(original_ltv, new_ltv);
    }

    #[derive(Debug, Ltv, Default, PartialEq, Eq)]
    #[object(id = 10, length_size=1, byte_order=BE)]
    struct LTVObjectExample {
        #[ltv_field(1)]
        field1: u8,
        #[ltv_field(2)]
        field2: Option<u8>
    }

    #[test]
    fn to_and_from_ltv_obj() {
        let original_ltv = LTVObjectExample { field1: 55, field2: None };
        let ltv_bytes = original_ltv.to_ltv_object();

        println!("{:?}", &ltv_bytes);
        let new_ltv = LTVObjectExample::from_ltv_object(&ltv_bytes).unwrap();
        assert_eq!(original_ltv, new_ltv);
    }

    #[test]
    fn output_test() {
        let my_object_bytes = LTVObjectExample { field1: 55, field2: None }.to_ltv_object();
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


    #[derive(Debug, LtvCollection, PartialEq, Eq)]
    enum MyObjects {
        Object1(LTVObjectExample),
    }

    #[test]
    fn collection_test() {
        let my_object = LTVObjectExample { field1: 55, field2: None };
        let obj_bytes = MyObjects::Object1(my_object).to_ltv();

        assert_eq!(
            obj_bytes,
            vec![
                2,  // Length of Field (field1)
                1,  // Field ID (field1)
                55  //Field Value
            ]
        )
    }

    #[test]
    fn collection_obj_test() {
        let my_object = LTVObjectExample { field1: 55, field2: None };
        let obj_bytes = MyObjects::Object1(my_object).to_ltv_object();
        assert_eq!(
            obj_bytes,
            vec![
                4,  // Length of object  (length can be 1 or two bytes by setting length_size)
                10, // Outer object ID (LTVObjectExample)
                2,  // Length of Field (field1)
                1,  // Field ID (field1)
                55  //Field Value
            ]
        )
    }

    #[test]
    fn collection_obj_test_same_as_single() {
        let o1 = LTVObjectExample { field1: 55, field2: None };
        assert_eq!(
            o1.to_ltv_object(),
            MyObjects::Object1(o1).to_ltv_object()
        );

    }

    #[test]
    fn from_ltv_to_collection() {
        let o1 = LTVObjectExample { field1: 55, field2: None };
        let sbytes = o1.to_ltv_object();
        let s1 = MyObjects::Object1(o1);
       
        
        let s2 = MyObjects::from_ltv_object(&sbytes).unwrap();
        assert_eq!(
            s1,
            s2
        );
    }
     

    #[derive(Debug, Ltv, Default, PartialEq, Eq)]
    #[object(id = 10, length_size=1, byte_order=BE)]
    struct LTVObjectUnnamed(u32);

    #[test]
    fn ltv_unnamed_strct() {
        let num : u32 = 1234567;
        let obj = LTVObjectUnnamed(num); 

        assert_eq!(
            ltv::get_ltv::<_, {ByteOrder::BE}>(&num),
            ltv::get_ltv::<_, {ByteOrder::BE}>(&obj)
        );
    }

}
