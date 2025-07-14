use super::*;

pub mod v1beta1;

pub trait ExternalMetric {
    const KIND: &'static str;
    const URL_PATH_SEGMENT: &'static str;
}
