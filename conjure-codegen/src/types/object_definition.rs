use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ObjectDefinition {
    type_name: Box<super::TypeName>,
    fields: Vec<super::FieldDefinition>,
    docs: Option<super::Documentation>,
}
impl ObjectDefinition {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn type_name(&self) -> &super::TypeName {
        &*self.type_name
    }
    #[inline]
    pub fn fields(&self) -> &[super::FieldDefinition] {
        &*self.fields
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    type_name: Option<Box<super::TypeName>>,
    fields: Vec<super::FieldDefinition>,
    docs: Option<super::Documentation>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn type_name(&mut self, type_name: super::TypeName) -> &mut Self {
        self.type_name = Some(Box::new(type_name));
        self
    }
    pub fn fields<T>(&mut self, fields: T) -> &mut Self
    where
        T: IntoIterator<Item = super::FieldDefinition>,
    {
        self.fields = fields.into_iter().collect();
        self
    }
    pub fn extend_fields<T>(&mut self, fields: T) -> &mut Self
    where
        T: IntoIterator<Item = super::FieldDefinition>,
    {
        self.fields.extend(fields);
        self
    }
    pub fn push_fields(&mut self, value: super::FieldDefinition) -> &mut Self {
        self.fields.push(value);
        self
    }
    pub fn docs<T>(&mut self, docs: T) -> &mut Self
    where
        T: Into<Option<super::Documentation>>,
    {
        self.docs = docs.into();
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> ObjectDefinition {
        ObjectDefinition {
            type_name: self.type_name.clone().expect("field type_name was not set"),
            fields: self.fields.clone(),
            docs: self.docs.clone(),
        }
    }
}
impl From<ObjectDefinition> for Builder {
    #[inline]
    fn from(_v: ObjectDefinition) -> Builder {
        Builder {
            type_name: Some(_v.type_name),
            fields: _v.fields,
            docs: _v.docs,
        }
    }
}
impl ser::Serialize for ObjectDefinition {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let mut size = 1usize;
        let skip_fields = self.fields.is_empty();
        if !skip_fields {
            size += 1;
        }
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let mut map = s.serialize_map(Some(size))?;
        map.serialize_entry(&"typeName", &self.type_name)?;
        if !skip_fields {
            map.serialize_entry(&"fields", &self.fields)?;
        }
        if !skip_docs {
            map.serialize_entry(&"docs", &self.docs)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for ObjectDefinition {
    fn deserialize<D_>(d: D_) -> Result<ObjectDefinition, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "ObjectDefinition",
            &["typeName", "fields", "docs"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = ObjectDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A_>(self, mut map_: A_) -> Result<ObjectDefinition, A_::Error>
    where
        A_: de::MapAccess<'de>,
    {
        let mut type_name = None;
        let mut fields = None;
        let mut docs = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::TypeName => type_name = Some(map_.next_value()?),
                Field_::Fields => fields = Some(map_.next_value()?),
                Field_::Docs => docs = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let type_name = match type_name {
            Some(v) => v,
            None => return Err(de::Error::missing_field("typeName")),
        };
        let fields = match fields {
            Some(v) => v,
            None => Default::default(),
        };
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(ObjectDefinition {
            type_name,
            fields,
            docs,
        })
    }
}
enum Field_ {
    TypeName,
    Fields,
    Docs,
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
            "typeName" => Field_::TypeName,
            "fields" => Field_::Fields,
            "docs" => Field_::Docs,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
