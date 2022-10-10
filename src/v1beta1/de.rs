use std::fmt;

use serde::de;

struct CpuVisitor;

pub(super) fn cpu_deserialize<'de, D>(d: D) -> Result<f64, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_str(CpuVisitor)
}

impl<'de> de::Visitor<'de> for CpuVisitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a CPU allocation in Kubernetes notation")
    }

    fn visit_str<E>(self, text: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let out = if let Some((number, _unit)) = text.split_once('n') {
            number.parse::<f64>().map(|f| f / 1_000_000_000_f64)
        } else if let Some((number, _unit)) = text.split_once('m') {
            number.parse::<f64>().map(|f| f / 1_000_f64)
        } else {
            text.parse::<f64>()
        };

        out.map_err(|_e| de::Error::custom(format!("Unexpected format: '{text}'")))
    }
}

struct MemoryVisitor;

pub(super) fn memory_deserialize<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_str(MemoryVisitor)
}

impl<'de> de::Visitor<'de> for MemoryVisitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a Memory allocation in Kubernetes notation")
    }

    fn visit_str<E>(self, text: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let out = if let Some((number, _unit)) = text.split_once("Ki") {
            number.parse::<u64>().map(|n| n * 1024)
        } else if let Some((number, _unit)) = text.split_once("Mi") {
            number.parse::<u64>().map(|n| n * 1024 * 1024)
        } else {
            text.parse::<u64>()
        };

        out.map_err(|_e| de::Error::custom(format!("Unexpected format: '{text}'")))
    }
}
