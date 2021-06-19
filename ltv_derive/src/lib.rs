#![feature(const_generics)]
pub use ltv_derive_impl::*;

#[cfg(test)]
mod tests {
    use ltv::*;

    #[derive(Debug, Ltv, Default, PartialEq, Eq)]
    struct ExampleStruct {
        #[ltv_field(1)]
        field1: u8,

        #[ltv_field(2)]
        field2: [u8; 3],
    }

    #[test]
    fn to_and_from_ltv() {
        let original_ltv = ExampleStruct {
            field1: 0x69,
            field2: [12, 34, 56],
        };
        let ltv_bytes = <ExampleStruct as LTVItem<{ ByteOrder::BE }>>::to_ltv(&original_ltv);

        let new_ltv =
            <ExampleStruct as LTVItem<{ ByteOrder::BE }>>::from_ltv(10, &ltv_bytes).unwrap();
        assert_eq!(original_ltv, new_ltv);
    }

    #[derive(Debug, Ltv, Default, PartialEq, Eq)]
    #[object(id = 10, length_size = 1, byte_order=LE)]
    struct LTVObjectExample {
        #[ltv_field(1)]
        field1: u8,
        #[ltv_field(2)]
        field2: Option<u8>,
    }

    #[test]
    fn to_and_from_ltv_obj() {
        let original_ltv = LTVObjectExample {
            field1: 55,
            field2: None,
        };
        let ltv_bytes = original_ltv.to_ltv_object();

        println!("{:?}", &ltv_bytes);
        let new_ltv =
            <LTVObjectExample as LTVObjectConvertable<{ ByteOrder::LE }, 1>>::from_ltv_object(
                &ltv_bytes,
            )
            .unwrap();
        assert_eq!(original_ltv, new_ltv);
    }

    #[test]
    fn output_test() {
        let my_object_bytes = LTVObjectExample {
            field1: 55,
            field2: None,
        }
        .to_ltv_object();
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
    #[object(byte_order=LE)]
    enum MyObjects {
        Object1(LTVObjectExample),
    }

    #[test]
    fn collection_test() {
        let my_object = LTVObjectExample {
            field1: 55,
            field2: None,
        };
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
        let my_object = LTVObjectExample {
            field1: 55,
            field2: None,
        };
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
        let o1 = LTVObjectExample {
            field1: 55,
            field2: None,
        };
        assert_eq!(o1.to_ltv_object(), MyObjects::Object1(o1).to_ltv_object());
    }

    #[test]
    fn from_ltv_to_collection() {
        let o1 = LTVObjectExample {
            field1: 55,
            field2: None,
        };
        let sbytes = o1.to_ltv_object();
        let s1 = MyObjects::Object1(o1);

        let s2 = MyObjects::from_ltv_object(&sbytes).unwrap();
        assert_eq!(s1, s2);
    }

    #[derive(Debug, Ltv, Default, PartialEq, Eq)]
    #[object(id = 10, length_size = 1)]
    struct LTVObjectUnnamed(u32);

    #[test]
    fn ltv_unnamed_strct() {
        let num: u32 = 1234567;
        let obj = LTVObjectUnnamed(num);

        assert_eq!(
            ltv::get_ltv::<_, { ByteOrder::BE }>(&num),
            ltv::get_ltv::<_, { ByteOrder::BE }>(&obj)
        );

        assert_eq!(
            obj,
            <LTVObjectUnnamed as LTVItem<{ ByteOrder::BE }>>::from_ltv(
                10,
                &ltv::get_ltv::<_, { ByteOrder::BE }>(&num)
            )
            .unwrap()
        );
    }

    #[derive(Debug, Ltv, Default, PartialEq, Eq)]
    #[object(id = 1, length_size = 1)]
    struct ItemWihtUnnamedField {
        #[ltv_field(1)]
        pub unnamed: LTVObjectUnnamed,
    }

    #[test]
    fn item_with_unnamed_struct_field() {
        let num: u32 = 1234567;
        let obj = ItemWihtUnnamedField {
            unnamed: LTVObjectUnnamed(num),
        };

        assert_eq!(
            obj,
            <ItemWihtUnnamedField as LTVItem<{ ByteOrder::BE }>>::from_ltv(
                10,
                &ltv::get_ltv::<_, { ByteOrder::BE }>(&obj)
            )
            .unwrap()
        );
    }

    //#[object(id = 1, length_size = 1)]
    #[derive(Debug, Default, PartialEq, Eq)]
    struct ItemWithList {
        //#[ltv_field_list(1)]
        pub items: Vec<u8>,
    }

    impl<const ED: ::ltv::ByteOrder> LTVItem<ED> for ItemWithList {
        fn from_ltv(field_id: u8, data: &[u8]) -> ::ltv::LTVResult<Self> {
            use ::ltv::LTVReader;
            let reader = LTVReader::<ED, 1usize>::new(&data);
            Ok(Self {
                items: reader.get_many::<<Vec<u8> as LTVItemMany<ED>>::Item, _>(1u8)?,
            })
        }
        fn to_ltv(&self) -> Vec<u8> {
            let mut buffer = LTVWriter::<_, ED, 1usize>::new(Vec::new());
            for o in <Vec<u8> as LTVItemMany<ED>>::get_items(&self.items) {
                buffer.write_ltv(1u8, o).ok();
            }
            buffer.into_inner()
        }
    }
    impl LTVObject<1> for ItemWithList {
        const OBJECT_ID: u8 = 1u8;
    }

    #[test]
    fn item_with_list() {
        let obj = ItemWithList {
            items: vec![1, 2, 3, 4, 5, 6],
        };

        let bytes = &ltv::get_ltv::<_, { ByteOrder::BE }>(&obj);

        assert_eq!(
            obj,
            <ItemWithList as LTVItem<{ ByteOrder::BE }>>::from_ltv(10, bytes).unwrap()
        );
    }

    #[derive(Ltv, Debug, Default, PartialEq, Eq)]
    pub struct MacAddress([u8; 6]);

    #[derive(Ltv, Debug, Default, PartialEq, Eq)]
    #[object(id = 0x02)]
    pub struct ModuleInfo {
        #[ltv_field(0x1)]
        pub softdevice_version: u16,
        #[ltv_field(0x2)]
        pub chipset_id: u32,
        //#[ltv_field(0x3)]
        //pub mac_address: MacAddress,
        #[ltv_field(0x4)]
        pub bootloader_version: u16,
        #[ltv_field(0x5)]
        pub firmware_version: u16,
        #[ltv_field(0x6)]
        pub entrypoint: Option<u8>,
    }

     #[derive(Debug, LtvCollection, PartialEq, Eq)]
     #[object(byte_order=LE)]
    enum ReaderStuff {
        Object1(ModuleInfo),
    }

    #[test]
    fn rrrrr() {
        let obj = ItemWithList {
            items: vec![1, 2, 3, 4, 5, 6],
        };

        let bytes = &ltv::get_ltv::<_, { ByteOrder::BE }>(&obj);

        assert_eq!(
            obj,
            <ItemWithList as LTVItem<{ ByteOrder::BE }>>::from_ltv(10, bytes).unwrap()
        );
    }
}
