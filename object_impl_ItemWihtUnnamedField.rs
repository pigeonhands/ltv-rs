#[automatically_derived] impl LTVItem < { :: ltv :: ByteOrder :: BE } > for
ItemWihtUnnamedField
{
    fn from_ltv(field_id : u8, data : & [u8]) -> :: ltv :: LTVResult < Self >
    {
        use :: ltv :: LTVReader ; let reader = LTVReader :: <
        { :: ltv :: ByteOrder :: BE }, 1usize > :: new(& data) ;
        Ok(Self { unnamed : reader.get_item :: < LTVObjectUnnamed > (1u8) ? })
    } fn to_ltv(& self) -> Vec < u8 >
    {
        let mut buffer = LTVWriter :: < _, { :: ltv :: ByteOrder :: BE },
        1usize > :: new(Vec :: new()) ;
        buffer.write_ltv(1u8, & self.unnamed).ok() ; buffer.into_inner()
    }
} #[automatically_derived] impl LTVObject < '_, { :: ltv :: ByteOrder :: BE },
1usize > for ItemWihtUnnamedField { const OBJECT_ID : u8 = 1u8 ; }