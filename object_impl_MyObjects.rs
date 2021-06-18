#[automatically_derived] impl LTVItem < { :: ltv :: ByteOrder :: BE } > for
MyObjects
{
    type Item = Self ; fn from_ltv(field_id : u8, data : & [u8]) -> :: ltv ::
    LTVResult < Self >
    {
        match field_id
        {
            LTVObjectExample :: OBJECT_ID =>
            Ok(Self ::
               Object1(LTVObjectExample ::
                       from_ltv(LTVObjectExample :: OBJECT_ID, data) ?)),
            LTVObjectExample :: OBJECT_ID =>
            Ok(Self ::
               Object2(LTVObjectExample ::
                       from_ltv(LTVObjectExample :: OBJECT_ID, data) ?)), _ =>
            Err(:: ltv :: LTVError :: NotFound(field_id))
        }
    } fn to_ltv(& self) -> Vec < u8 >
    {
        match self
        { Self :: Object1(v) => v.to_ltv(), Self :: Object2(v) => v.to_ltv() }
    }
} impl < 'a > LTVObjectConvertable < 'a, { :: ltv :: ByteOrder :: BE }, 1usize
> for MyObjects
{
    fn from_ltv_object(data : & 'a [u8]) -> LTVResult < Self :: Item >
    {
        use :: ltv :: LTVReader ; let(_, obj_id, data) = LTVReader :: < 'a,
        { :: ltv :: ByteOrder :: BE }, 1usize > :: parse_ltv(data) ? ;
        Ok(Self :: from_ltv(obj_id, data) ?)
    } fn to_ltv_object(& self) -> Vec < u8 > { todo! () }
}