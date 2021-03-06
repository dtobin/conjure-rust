use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct BinaryAliasExample(pub conjure_object::ByteBuf);
impl std::ops::Deref for BinaryAliasExample {
    type Target = conjure_object::ByteBuf;
    #[inline]
    fn deref(&self) -> &conjure_object::ByteBuf {
        &self.0
    }
}
impl std::ops::DerefMut for BinaryAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::ByteBuf {
        &mut self.0
    }
}
impl ser::Serialize for BinaryAliasExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for BinaryAliasExample {
    fn deserialize<D_>(d: D_) -> Result<BinaryAliasExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(BinaryAliasExample)
    }
}
