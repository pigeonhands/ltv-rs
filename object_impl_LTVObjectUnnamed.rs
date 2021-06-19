#[automatically_derived] impl LTVItem < { :: ltv :: ByteOrder :: BE } > for
LTVObjectUnnamed
{
    type Item = Self ; fn to_ltv(& self) -> Vec < u8 >
    {
        < u32 as LTVItem < { :: ltv :: ByteOrder :: BE } >> ::
        to_ltv(& self.0)
    } fn from_ltv(field_id : u8, data : & [u8]) -> :: ltv :: LTVResult < Self
    >
    {
        Ok(Self(< u32 as LTVItem < { :: ltv :: ByteOrder :: BE } >> ::
                from_ltv(field_id, data) ?))
    }
}