use ltv::*;

#[derive(Debug, Ltv, Default, PartialEq, Eq)]
struct ExampleStruct {
    #[ltv_field(1)]
    field1: u8,
    #[ltv_field(1)]
    field2: Option<u8>,
}

fn main() {
    let original_ltv = ExampleStruct {
        field1: 0x69,
        field2: None,
    };
    let ltv_bytes = original_ltv.to_ltv_object(0x44);

    println!("{:?}", &ltv_bytes);
    let new_ltv = ExampleStruct::from_ltv_object(&ltv_bytes).unwrap();
    assert_eq!(original_ltv, new_ltv);
}
