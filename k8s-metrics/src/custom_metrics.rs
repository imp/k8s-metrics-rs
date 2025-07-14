use super::*;

pub mod v1beta2;

pub trait CustomMetric {
    const KIND: &'static str;
    const URL_PATH_SEGMENT: &'static str;
    type Scope: k8s::ResourceScope;
}
