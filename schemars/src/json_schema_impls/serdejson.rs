use crate::gen::SchemaGenerator;
use crate::{json_schema, JsonSchema, Schema};
use serde_json::{Map, Number, Value};
use std::borrow::Cow;
use std::collections::BTreeMap;

impl JsonSchema for Value {
    always_inline!();

    fn schema_name() -> Cow<'static, str> {
        "AnyValue".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        true.into()
    }
}

forward_impl!(Map<String, Value> => BTreeMap<String, Value>);

impl JsonSchema for Number {
    always_inline!();

    fn schema_name() -> Cow<'static, str> {
        "Number".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "number"
        })
    }
}

#[cfg(feature = "raw_value")]
forward_impl!(serde_json::value::RawValue => Value);
