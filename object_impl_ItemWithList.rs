#[automatically_derived] impl LTVItem < { :: ltv :: ByteOrder :: BE } > for
ItemWithList
{
    fn from_ltv(field_id : u8, data : & [u8]) -> :: ltv :: LTVResult < Self >
    {
        use :: ltv :: LTVReader ; let reader = LTVReader :: <
        { :: ltv :: ByteOrder :: BE }, 1usize > :: new(& data) ;
        Ok(Self
           {
               items : reader.get_many :: << Vec < u8 > as LTVItemMany <
               { :: ltv :: ByteOrder :: BE } >> :: Item, _ > (1u8) ?
           })
    } fn to_ltv(& self) -> Vec < u8 >
    {
        let mut buffer = LTVWriter :: < _, { :: ltv :: ByteOrder :: BE },
        1usize > :: new(Vec :: new()) ; for o in < Vec < u8 > as LTVItemMany <
        { :: ltv :: ByteOrder :: BE } >> :: get_items(& self.items)
        { buffer.write_ltv(1u8, o).ok() ; } buffer.into_inner()
    }
} #[automatically_derived] impl LTVObject < '_, { :: ltv :: ByteOrder :: BE },
1usize > for ItemWithList { const OBJECT_ID : u8 = 1u8 ; }