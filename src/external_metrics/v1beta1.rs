use std::collections::BTreeMap;
use std::marker::PhantomData;

use super::*;

/// ExternalMetricValue is a metric value for external metric
/// A single metric value is identified by metric name and a set of string labels.
/// For one metric there can be multiple values with different sets of labels.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "camelCase")]
pub struct ExternalMetricValue<M> {
    pub metadata: metav1::ObjectMeta,

    /// the name of the metric
    ///
    pub metric_name: String, // `json:"metricName" protobuf:"bytes,1,name=metricName"`

    /// a set of labels that identify a single time series for the metric
    ///
    pub metric_labels: BTreeMap<String, String>, // `json:"metricLabels" protobuf:"bytes,2,rep,name=metricLabels"`

    /// indicates the time at which the metrics were produced
    ///
    pub timestamp: metav1::Time, // `json:"timestamp" protobuf:"bytes,3,name=timestamp"`

    /// indicates the window ([Timestamp-Window, Timestamp]) from
    /// which these metrics were calculated, when returning rate
    /// metrics calculated from cumulative metrics (or zero for
    /// non-calculated instantaneous metrics).
    ///
    #[serde(default, rename = "window")]
    pub window_seconds: i64, // `json:"window,omitempty" protobuf:"bytes,4,opt,name=window"`

    /// the value of the metric
    ///
    pub value: resource::Quantity, // `json:"value" protobuf:"bytes,5,name=value"`

    phantom: PhantomData<M>,
}

impl<M: ExternalMetric> k8s::Resource for ExternalMetricValue<M> {
    const API_VERSION: &'static str = "external.metrics.k8s.io/v1beta1";
    const GROUP: &'static str = "external.metrics.k8s.io";
    const KIND: &'static str = M::KIND;
    const VERSION: &'static str = "v1beta1";
    const URL_PATH_SEGMENT: &'static str = M::URL_PATH_SEGMENT;
    type Scope = k8s::ClusterResourceScope;
}

impl<M: ExternalMetric> k8s::ListableResource for ExternalMetricValue<M> {
    const LIST_KIND: &'static str = "ExternalMetricValueList";
}

pub type ExternalMetricValueList<M> = k8s::List<ExternalMetricValue<M>>;
