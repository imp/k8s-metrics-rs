use super::*;

pub trait QuantityExt {
    fn to_memory(&self) -> Result<i64, QuantityParseError>;
    fn to_f64(&self) -> Result<f64, QuantityParseError>;
}

impl QuantityExt for resource::Quantity {
    fn to_memory(&self) -> Result<i64, QuantityParseError> {
        if let Some((number, _unit)) = self.0.split_once("Ki") {
            number.parse::<i64>().map(|n| n * 1024)
        } else if let Some((number, _unit)) = self.0.split_once("Mi") {
            number.parse::<i64>().map(|n| n * 1024 * 1024)
        } else if let Some((number, _unit)) = self.0.split_once("Gi") {
            number.parse::<i64>().map(|n| n * 1024 * 1024 * 1024)
        } else {
            self.0.parse::<i64>()
        }
        .map_err(|_e| QuantityParseError::new(&self.0))
    }

    fn to_f64(&self) -> Result<f64, QuantityParseError> {
        let out = if let Some((number, _unit)) = self.0.split_once('n') {
            number.parse::<f64>().map(|f| f / 1_000_000_000_f64)
        } else if let Some((number, _unit)) = self.0.split_once('u') {
            number.parse::<f64>().map(|f| f / 1_000_000_f64)
        } else if let Some((number, _unit)) = self.0.split_once('m') {
            number.parse::<f64>().map(|f| f / 1_000_f64)
        } else {
            self.0.parse::<f64>()
        };

        out.map_err(|_e| QuantityParseError::new(&self.0))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Unexpected format: {0}")]
pub struct QuantityParseError(String);

impl QuantityParseError {
    fn new(text: &str) -> Self {
        Self(text.to_string())
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn memory_Ki() {
        let q = quantity("2Ki").to_memory().unwrap();
        assert_eq!(q, 2048);
    }

    #[test]
    fn memory_Mi() {
        let q = quantity("5Mi").to_memory().unwrap();
        assert_eq!(q, 5242880);
    }

    #[test]
    fn memory_Gi() {
        let q = quantity("3Gi").to_memory().unwrap();
        assert_eq!(q, 3221225472);
    }

    #[test]
    fn leading_zeros() {
        let q = quantity("0004Ki").to_memory().unwrap();
        assert_eq!(q, 4096);
    }

    #[test]
    fn cpu_nano() {
        let q = quantity("257n").to_f64().unwrap();
        assert_eq!(q, 0.000000257);
    }

    #[test]
    fn cpu_micro() {
        let q = quantity("303u").to_f64().unwrap();
        assert_eq!(q, 0.000303);
    }

    #[test]
    fn cpu_milli() {
        let q = quantity("3491m").to_f64().unwrap();
        assert_eq!(q, 3.491);
    }

    fn quantity(v: &str) -> resource::Quantity {
        resource::Quantity(v.to_string())
    }
}
