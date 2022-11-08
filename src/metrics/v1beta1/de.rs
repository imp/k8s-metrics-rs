use std::fmt;
use std::time;

use serde::de;

struct DurationVisitor;

pub(super) fn duration<'de, D>(d: D) -> Result<time::Duration, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_str(DurationVisitor)
}

impl<'de> de::Visitor<'de> for DurationVisitor {
    type Value = time::Duration;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a Duration in Kubernetes notation")
    }

    fn visit_str<E>(self, text: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        go_parse_duration::parse_duration(text)
            .map(|nanos| time::Duration::from_nanos(nanos as u64))
            .map_err(|_| de::Error::custom(format!("invalid duration: '{text}'")))
    }
}
