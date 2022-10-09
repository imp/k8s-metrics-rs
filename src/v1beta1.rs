use super::*;

pub use node::NodeMetrics;

mod node;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Usage {
    pub cpu: String,
    pub memory: String,
}
