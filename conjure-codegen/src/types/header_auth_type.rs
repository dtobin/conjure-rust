use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Copy)]
pub struct HeaderAuthType {}
impl HeaderAuthType {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {}
impl Builder {
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> HeaderAuthType {
        HeaderAuthType {}
    }
}
impl From<HeaderAuthType> for Builder {
    #[inline]
    fn from(_v: HeaderAuthType) -> Builder {
        Builder {}
    }
}
impl ser::Serialize for HeaderAuthType {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let size = 0usize;
        let map = s.serialize_map(Some(size))?;
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for HeaderAuthType {
    fn deserialize<D_>(d: D_) -> Result<HeaderAuthType, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_struct("HeaderAuthType", &[], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = HeaderAuthType;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A_>(self, mut map_: A_) -> Result<HeaderAuthType, A_::Error>
    where
        A_: de::MapAccess<'de>,
    {
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        Ok(HeaderAuthType {})
    }
}
enum Field_ {
    Unknown_,
}
impl<'de> de::Deserialize<'de> for Field_ {
    fn deserialize<D_>(d: D_) -> Result<Field_, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_str(FieldVisitor_)
    }
}
struct FieldVisitor_;
impl<'de> de::Visitor<'de> for FieldVisitor_ {
    type Value = Field_;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("string")
    }
    fn visit_str<E_>(self, value: &str) -> Result<Field_, E_>
    where
        E_: de::Error,
    {
        let v = match value {
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
