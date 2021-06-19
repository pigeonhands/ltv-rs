# LTV
```
ltv="0.1"
```

[![](https://img.shields.io/crates/v/ltv?style=for-the-badge)](https://crates.io/crates/ltv)

----
## LTV Explination
```Rust
#[derive(Debug, Ltv, Default, PartialEq, Eq)]
#[object(id = 10, length_size=1, byte_order=BE)]
struct LTVObjectExample {
    #[ltv_field(1)]
    field1: u8,
}

#[derive(Debug, Ltv, Default, PartialEq, Eq)]
#[object(id = 11, length_size=1, byte_order=BE)]
struct AnotherStruct {
    #[ltv_field(1)]
    field1: u8,
}

#[derive(Debug, LtvCollection, PartialEq, Eq)]
enum MyCollection {
    Object1(LTVObjectExample),
    Object2(AnotherStruct),
}

let my_object_bytes = LTVObjectExample{ field1: 55 };
assert_eq!(my_object_bytes.to_ltv_object(), vec![
    4,   // Total Length (length can be 1 or two bytes by setting length_size)
    10, // Outer object ID (LTVObjectExample)
    2,   // Length of Field (field1)
    1,   // Field ID (field1)
    55   //Field Value
]);

assert_eq!(MyCollection::Object1(my_object_bytes).to_ltv_object(), vec![
    4,   // Total Length (length can be 1 or two bytes by setting length_size)
    10, // Outer object ID (LTVObjectExample)
    2,   // Length of Field (field1)
    1,   // Field ID (field1)
    55   //Field Value
]);

```


## Usage

```Rust

use ltv::*;

#[derive(Debug, PartialEq, Eq, Ltv, Default)]
struct InnerStructData {
    #[ltv_field(1)]
    field1: u8,
    #[ltv_field(2)]
    field2: u16,
}

#[derive(Debug, Ltv, Default, PartialEq, Eq)]
#[object(id = 10, length_size=1, byte_order=BE)]
struct ExampleStruct {
    #[ltv_field(1)]
    field1: InnerStructData,
    #[ltv_field(2)]
    field2: Option<u8>,
}

fn main() {
    let original_ltv = ExampleStruct {
        field1: InnerStructData {
            field1: 19,
            field2: 77,
        },
        field2: None,
    };

    let ltv_bytes = original_ltv.to_ltv_object();
    let new_ltv = ExampleStruct::from_ltv_object(&ltv_bytes).unwrap();
    assert_eq!(original_ltv, new_ltv);
}
```


## Basic usage

```Rust
#[derive(Debug, PartialEq, Eq)]
struct BasicLTV{
    field1: u8,
}

impl<'a> LTVItem<{ ByteOrder::LE }> for BasicLTV {
    type Item = Self;
    fn from_ltv(_: usize, data: &[u8]) -> LTVResult<Self::Item> {
        let reader = LTVReaderLE::<1>::new(data);
        Ok(BasicLTV {
            field1: reader.get_item::<u8>(0x01)?,
        })
    }
    fn to_ltv(&self) -> Vec<u8> {
        let mut writer = LTVWriterLE::new(Vec::with_capacity(3));
        writer.write_ltv(0x01, &self.field1).ok();
        writer.into_inner()
    }
}
```
```Rust
let original = BasicLTV{
    field1: 0x35
};

// to_ltv only returns the [v] infomation of an object
let buffer = original.to_ltv();

let out = BasicLTV::from_ltv(0x01, &buffer).unwrap();
assert_eq!(original, out);
assert_eq!(&buffer, &[2, 0x01, 0x35]);
```


## Struct Reading

```Rust
#[derive(Debug, PartialEq, Eq)]
struct InnerStructData {
    field1: u8,
    field2: u16,
}
impl<const ED: ByteOrder> LTVItem<ED> for InnerStructData {
    type Item = Self;
    fn from_ltv(_field_id: usize, data: &[u8]) -> LTVResult<Self> {
        let reader = LTVReader::<ED, 1>::new(&data);

        Ok(InnerStructData {
            field1: reader.get_item::<u8>(0x1)?,
            field2: reader.get_item::<u16>(0x2)?,
        })
    }

    fn to_ltv(&self) -> Vec<u8> {
        unimplemented!()
    }
}
```


```Rust
let input_data: &[u8] = &[
    0x04, 0x01, 0x02, 0x01, 0xFF, 0x08, 0x02, 0x02, 0x01, 0x55, 0x03, 0x02, 0x01, 0x00,
];
let reader = LTVReaderLE::<1>::new(&input_data[2..]);

let field_1 = reader.get_item::<u8>(0x1).unwrap();
assert_eq!(field_1, 0xFF);

let field_2 = reader.get_item::<InnerStructData>(0x2).unwrap();

assert_eq!(
    field_2,
    InnerStructData {
        field1: 0x55,
        field2: 0x0100
    }
);
```