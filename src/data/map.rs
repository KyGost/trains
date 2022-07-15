use {
    crate::{Stop, Switch, Train},
    async_trait::async_trait,
    freds::{
        Data, DataExt, Error, Inline, InlineData, Reader, ReferentialData, Value, Write, Writer,
    },
};
#[derive(Default, Debug, Clone)]
pub struct Map {
    pub trains: Vec<Train>,
    pub stops: Vec<Stop>,
    pub switches: Vec<Switch>,
}

impl Data for Map {
    const KIND: [u8; 1] = [b'M'];
    const IS_INLINE: bool = false;
}
impl InlineData for Map {}
impl ReferentialData for Map {
    fn into_bytes(self) -> Result<Vec<u8>, Error> {
        self.trains
            .into_iter()
            .map(|t| t.into_bytes())
            .collect::<Result<Vec<Vec<u8>>, _>>()
            .map(|a| a.concat())
    }
}
impl Write for Map {
    fn write(self) -> Vec<u8> {
        let mut writer = Writer::default();
        let core = self.into_inline(&mut writer).unwrap();
        writer.set_core(core);
        writer.into_bytes()
    }
}
#[async_trait]
impl Value for Map {
    async fn from_bytes(
        reader: &mut Reader<Map>,
        kind: [u8; freds::constants::SIZE_KIND],
        bytes: Vec<u8>,
    ) -> Result<Self, Error> {
        value_from_bytes(reader, kind, bytes).await
    }
    async fn from_inline(reader: &mut Reader<Map>, inline: Inline) -> Result<Self, Error> {
        value_from_inline(reader, inline).await
    }
}

macro_rules! referential_convert_enum {
    [$($kind: ty),*] => {
        async fn value_from_bytes(reader: &mut Reader<Value>, kind: [u8; crate::data::constants::SIZE_KIND], bytes: Vec<u8>) -> Result<Value, Error> {
            use crate::{Data, ReferentialData, implementations::serde_json::IntoValue};
            Ok(match kind {
                Null::KIND => Value::Null,
                $(<$kind>::KIND => <$kind>::from_bytes(bytes)?.into_value(reader).await?),*,
                _ => Value::Null,
            })
        }
    }
}
referential_convert_enum![Array, Map, Train, Switch, Stop, Position];

macro_rules! inline_convert_enum {
    [$($kind: ty),*] => {
        async fn value_from_inline(reader: &mut Reader<Value>, inline: Inline) -> Result<Value, Error> {
            use crate::{Data, InlineData, implementations::serde_json::IntoValue};
            Ok(match inline.kind {
                Null::KIND => Value::Null,
                $(<$kind>::KIND => <$kind>::from_inline_data(inline.data)?.into_value(reader).await?),*,
                _ => Value::Null,
            })
        }
    }
}
inline_convert_enum![u64, bool];
