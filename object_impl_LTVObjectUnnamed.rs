#[automatically_derived] impl LTVItem < { :: ltv :: ByteOrder :: BE } > for
LTVObjectUnnamed
{
    type Item = u32 ; fn to_ltv(& self) -> Vec < u8 > { todo! () } fn
    from_ltv(field_id : u8, data : & [u8]) -> :: ltv :: LTVResult < Self ::
    Item > { u32 :: from_ltv(field_id, data) }
}