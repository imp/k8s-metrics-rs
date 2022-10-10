use super::*;

pub use node::NodeMetrics;
pub use pod::PodMetrics;

mod de;
mod node;
mod pod;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Usage {
    #[serde(deserialize_with = "de::cpu_deserialize")]
    pub cpu: f64,
    #[serde(deserialize_with = "de::memory_deserialize")]
    pub memory: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    pub usage: Usage,
}

#[cfg(test)]
mod tests;
