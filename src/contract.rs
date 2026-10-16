use crate::Result;
use serde::{
    Deserialize, Deserializer,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use serde_json::{Map, Value};
use std::{collections::HashSet, fmt, path::Path, sync::LazyLock};

pub const APP_IDS: [&str; 3] = ["breakfree.docs", "breakfree.sheets", "breakfree.slides"];
pub const APP_NAMES: [&str; 3] = ["BreakFree Docs", "BreakFree Sheets", "BreakFree Slides"];
pub const SCHEMA: &str = include_str!("../contracts/v1/app-manifest.schema.json");
static VALIDATOR: LazyLock<jsonschema::Validator> = LazyLock::new(|| {
    jsonschema::validator_for(&serde_json::from_str(SCHEMA).expect("checked-in schema JSON"))
        .expect("checked-in schema must validate")
});

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Format {
    pub id: String,
    pub extensions: Vec<String>,
    pub read: bool,
    pub write: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub protocol_version: u32,
    pub app_id: String,
    pub app_version: String,
    pub display_name: String,
    pub capabilities: Vec<String>,
    pub formats: Vec<Format>,
}

impl Manifest {
    pub fn supports(&self, action: &str) -> bool {
        self.capabilities.iter().any(|value| value == action)
    }

    pub fn can_open(&self, path: &Path) -> bool {
        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        self.supports("open")
            && self
                .formats
                .iter()
                .any(|f| f.read && f.extensions.contains(&extension))
    }
}

// serde_json::Value alone silently retains the last duplicate key. Reject duplicates
// before schema validation, including duplicates in nested format objects.
struct Unique(Value);
impl<'de> Deserialize<'de> for Unique {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        struct StrictVisitor;
        impl<'de> Visitor<'de> for StrictVisitor {
            type Value = Unique;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("strict JSON")
            }
            fn visit_map<M: MapAccess<'de>>(
                self,
                mut map: M,
            ) -> std::result::Result<Unique, M::Error> {
                let mut result = Map::new();
                while let Some((key, value)) = map.next_entry::<String, Unique>()? {
                    if result.insert(key, value.0).is_some() {
                        return Err(de::Error::custom("duplicate JSON key"));
                    }
                }
                Ok(Unique(Value::Object(result)))
            }
            fn visit_seq<S: SeqAccess<'de>>(
                self,
                mut seq: S,
            ) -> std::result::Result<Unique, S::Error> {
                let mut result = Vec::new();
                while let Some(value) = seq.next_element::<Unique>()? {
                    result.push(value.0);
                }
                Ok(Unique(Value::Array(result)))
            }
            fn visit_bool<E: de::Error>(self, v: bool) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_f64<E: de::Error>(self, v: f64) -> std::result::Result<Unique, E> {
                serde_json::Number::from_f64(v)
                    .map(|n| Unique(Value::Number(n)))
                    .ok_or_else(|| de::Error::custom("non-finite JSON number"))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_string<E: de::Error>(self, v: String) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_unit<E: de::Error>(self) -> std::result::Result<Unique, E> {
                Ok(Unique(Value::Null))
            }
        }
        deserializer.deserialize_any(StrictVisitor)
    }
}

pub fn strict_json(bytes: &[u8]) -> Result<Value> {
    serde_json::from_slice::<Unique>(bytes)
        .map(|u| u.0)
        .map_err(|_| {
            "Expected one strict UTF-8 JSON object; duplicate keys and trailing data are rejected."
                .into()
        })
}

pub fn validate(bytes: &[u8], expected_id: Option<&str>) -> Result<Manifest> {
    let value = strict_json(bytes)?;
    let version = value
        .get("protocol_version")
        .and_then(Value::as_i64)
        .ok_or("protocol_version must be a JSON integer.")?;
    if version != 1 {
        return Err(format!(
            "Unsupported protocol {version}; Office requires protocol 1."
        ));
    }
    if !VALIDATOR.is_valid(&value) {
        return Err("Capability manifest does not match the v1 schema.".into());
    }
    let manifest: Manifest =
        serde_json::from_value(value).map_err(|_| "Invalid capability fields.")?;
    if expected_id.is_some_and(|id| id != manifest.app_id) {
        return Err("Executable returned the wrong application identity.".into());
    }
    if manifest.display_name.trim().is_empty() {
        return Err("Display name cannot be blank.".into());
    }
    let mut ids = HashSet::new();
    let mut extensions = HashSet::new();
    for format in &manifest.formats {
        if !ids.insert(&format.id) {
            return Err("Format IDs must be unique.".into());
        }
        for ext in &format.extensions {
            if !extensions.insert(ext) {
                return Err("Format extensions must be unambiguous.".into());
            }
        }
        if format.read && !manifest.supports("open") {
            return Err("Readable formats require the open capability.".into());
        }
    }
    Ok(manifest)
}
