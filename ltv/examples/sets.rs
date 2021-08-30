use ltv::*;

#[derive(Debug, Ltv, Default, PartialEq, Eq)]
#[object(id = 10, byte_order=BE)]
struct ExampleSet {
    #[ltv_field(1)]
    field1: u8,
}

fn main() {
    let original_ltv = ExampleSet { field1: 0x69 };
    let ltv_bytes = original_ltv.to_ltv_object();

    println!("{:?}", &ltv_bytes);
    let new_ltv = ExampleSet::from_ltv_object(&ltv_bytes).unwrap();
    assert_eq!(original_ltv, new_ltv);
}
