use super::*;

pub use node::NodeMetrics;
pub use pod::PodMetrics;

mod duration;
mod node;
mod pod;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Usage {
    pub cpu: resource::Quantity,
    pub memory: resource::Quantity,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    pub usage: Usage,
}

impl Usage {
    pub fn cpu(&self) -> Result<f64, QuantityParseError> {
        self.cpu.to_f64()
    }

    pub fn memory(&self) -> Result<i64, QuantityParseError> {
        self.memory.to_memory()
    }
}

#[cfg(test)]
mod tests;
