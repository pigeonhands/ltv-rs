#![feature(const_generics)]
use std::fmt::Debug;

use ltv::*;

#[derive(Debug, PartialEq, Eq, Ltv, Default)]
struct InnerStructData {
    #[ltv_field(1)]
    field1: u8,
    #[ltv_field(2)]
    field2: u16,
}

#[derive(Debug, Ltv, Default, PartialEq, Eq)]
struct ExampleStruct {
    #[ltv(LE)]
    #[ltv_field(1)]
    field1: InnerStructData,
    #[ltv_field(2)]
    field2: Option<u8>,
}
fn print_item<T: LTVItem<ED> + Debug, const ED: ByteOrder>(obj: &T) {
    println!("{:?} -> ({:?})", obj, ED);
}
fn main() {
    let original_ltv = ExampleStruct {
        field1: InnerStructData {
            field1: 19,
            field2: 77,
        },
        field2: None,
    };

    print_item(&original_ltv);

    let ltv_bytes = original_ltv.to_ltv_object(0x44);

    println!("{:?}", &ltv_bytes);
    let new_ltv = ExampleStruct::from_ltv_object(&ltv_bytes).unwrap();
    assert_eq!(original_ltv, new_ltv);
}
