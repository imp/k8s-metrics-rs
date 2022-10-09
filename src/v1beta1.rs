use super::*;

pub use node::NodeMetrics;
pub use pod::PodMetrics;

mod node;
mod pod;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Usage {
    pub cpu: String,
    pub memory: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    pub usage: Usage,
}
