use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CovariantListExample {
    items: Vec<conjure_object::Value>,
    external_items: Vec<String>,
}
impl CovariantListExample {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn items(&self) -> &[conjure_object::Value] {
        &*self.items
    }
    #[inline]
    pub fn external_items(&self) -> &[String] {
        &*self.external_items
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    items: Vec<conjure_object::Value>,
    external_items: Vec<String>,
}
impl Builder {
    pub fn items<T>(&mut self, items: T) -> &mut Self
    where
        T: IntoIterator<Item = conjure_object::Value>,
    {
        self.items = items.into_iter().collect();
        self
    }
    pub fn extend_items<T>(&mut self, items: T) -> &mut Self
    where
        T: IntoIterator<Item = conjure_object::Value>,
    {
        self.items.extend(items);
        self
    }
    pub fn push_items<T>(&mut self, value: T) -> &mut Self
    where
        T: conjure_object::serde::Serialize,
    {
        self.items
            .push(conjure_object::serde_value::to_value(value).expect("value failed to serialize"));
        self
    }
    pub fn external_items<T>(&mut self, external_items: T) -> &mut Self
    where
        T: IntoIterator<Item = String>,
    {
        self.external_items = external_items.into_iter().collect();
        self
    }
    pub fn extend_external_items<T>(&mut self, external_items: T) -> &mut Self
    where
        T: IntoIterator<Item = String>,
    {
        self.external_items.extend(external_items);
        self
    }
    pub fn push_external_items<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.external_items.push(value.into());
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> CovariantListExample {
        CovariantListExample {
            items: self.items.clone(),
            external_items: self.external_items.clone(),
        }
    }
}
impl From<CovariantListExample> for Builder {
    #[inline]
    fn from(_v: CovariantListExample) -> Builder {
        Builder {
            items: _v.items,
            external_items: _v.external_items,
        }
    }
}
impl ser::Serialize for CovariantListExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let mut size = 0usize;
        let skip_items = self.items.is_empty();
        if !skip_items {
            size += 1;
        }
        let skip_external_items = self.external_items.is_empty();
        if !skip_external_items {
            size += 1;
        }
        let mut map = s.serialize_map(Some(size))?;
        if !skip_items {
            map.serialize_entry(&"items", &self.items)?;
        }
        if !skip_external_items {
            map.serialize_entry(&"externalItems", &self.external_items)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for CovariantListExample {
    fn deserialize<D_>(d: D_) -> Result<CovariantListExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "CovariantListExample",
            &["items", "externalItems"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = CovariantListExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A_>(self, mut map_: A_) -> Result<CovariantListExample, A_::Error>
    where
        A_: de::MapAccess<'de>,
    {
        let mut items = None;
        let mut external_items = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Items => items = Some(map_.next_value()?),
                Field_::ExternalItems => external_items = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let items = match items {
            Some(v) => v,
            None => Default::default(),
        };
        let external_items = match external_items {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(CovariantListExample {
            items,
            external_items,
        })
    }
}
enum Field_ {
    Items,
    ExternalItems,
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
            "items" => Field_::Items,
            "externalItems" => Field_::ExternalItems,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
