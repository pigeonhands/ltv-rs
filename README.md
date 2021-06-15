# LTV
```
ltv="1"
```

[![](https://img.shields.io/crates/v/ltv?style=for-the-badge)](https://crates.io/crates/ltv)


----
## Basic usage

```Rust
#[derive(Debug, PartialEq, Eq)]
struct BasicLTV{
    field1: u8,
}

impl<'a> LTVItem<'a, ed::BE> for BasicLTV {
    type Item = BasicLTV;
    fn from_ltv(_: usize, data: &'a [u8]) -> LTVResult<Self::Item> {
        let reader = LTVReader::<ed::BE, 1>::new(data);
        Ok(BasicLTV{
            field1: reader.get_item::<u8>(0x01)?,
        })
    }
    fn to_ltv(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(3);
        buffer.write_ltv(0x01, &self.field1).ok();
        buffer
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
struct inner_struct_data {
    field1: u8,
    field2: u16
}
impl<ED:ByteOrder> LTVItem<'_, ED> for inner_struct_data {
    type Item = inner_struct_data;
    fn from_ltv(field_id:usize, data: &[u8]) -> LTVResult<Self> {
        let reader = LTVReader::<ed::BE, 1>::new(&data);

        Ok(
            inner_struct_data{
                field1: reader.get_item::<u8>(0x1)?,
                field2: reader.get_item::<u16>(0x2)?,
            }
        )
    }
    
    fn to_ltv(&self) -> Vec<u8>{
        unimplemented!()
    }
}
```


```Rust
let input_data : &[u8] = &[
    0x04, 
    0x01,
    0x02,
    0x01,
    0xFF,
    0x08,
    0x02,
    0x02,
    0x01,
    0x55,
    0x03,
    0x02,
    0x01,
    0x00
];
let reader = LTVReader::<ed::BE, 1>::new(&input_data[2..]);

let field_1 = reader.get_item::<u8>(0x1).unwrap();
assert_eq!(field_1, 0xFF);

let field_2 = reader.get_item::<inner_struct_data>(0x2).unwrap();
assert_eq!(field_2, inner_struct_data{ field1: 0x55, field2: 0x0100 });
```