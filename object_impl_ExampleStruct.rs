#[automatically_derived] impl LTVItem < { :: ltv :: ByteOrder :: BE } > for
ExampleStruct
{
    fn from_ltv(field_id : u8, data : & [u8]) -> :: ltv :: LTVResult < Self >
    {
        use :: ltv :: LTVReader ; let reader = LTVReader :: <
        { :: ltv :: ByteOrder :: BE }, 1usize > :: new(& data) ;
        Ok(Self
           {
               field1 : reader.get_item :: < u8 > (1u8) ?, field2 :
               reader.get_item :: < [u8 ; 3] > (2u8) ?
           })
    } fn to_ltv(& self) -> Vec < u8 >
    {
        let mut buffer = LTVWriter :: < _, { :: ltv :: ByteOrder :: BE },
        1usize > :: new(Vec :: new()) ;
        buffer.write_ltv(1u8, & self.field1).ok() ;
        buffer.write_ltv(2u8, & self.field2).ok() ; buffer.into_inner()
    }
}