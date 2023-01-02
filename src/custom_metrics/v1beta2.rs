use super::*;

/// MetricIdentifier identifies a metric by name and, optionally, selector
///
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricIdentifier {
    /// name is the name of the given metric
    ///
    pub name: String,
    /// selector represents the label selector that could be used to select
    /// this metric, and will generally just be the selector passed in to
    /// the query used to fetch this metric.
    /// When left blank, only the metric's Name will be used to gather metrics.
    /// +optional
    ///
    pub selector: Option<metav1::LabelSelector>,
}

/// MetricValue is the metric value for some object
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "camelCase")]
pub struct MetricValue {
    pub metadata: metav1::ObjectMeta,

    /// a reference to the described object
    ///
    pub described_object: corev1::ObjectReference,

    pub metric: MetricIdentifier,

    /// indicates the time at which the metrics were produced
    pub timestamp: metav1::Time,

    /// indicates the window ([Timestamp-Window, Timestamp]) from
    /// which these metrics were calculated, when returning rate
    /// metrics calculated from cumulative metrics (or zero for
    /// non-calculated instantaneous metrics).
    ///
    pub window_seconds: i64, // `json:"windowSeconds,omitempty" protobuf:"bytes,4,opt,name=windowSeconds"`

    /// the value of the metric for this
    ///
    pub value: resource::Quantity, // `json:"value" protobuf:"bytes,5,name=value"`
}

impl k8s::Resource for MetricValue {
    const API_VERSION: &'static str = "custom.metrics.k8s.io/v1beta1";
    const GROUP: &'static str = "custom.metrics.k8s.io";
    const KIND: &'static str = "xxx";
    const VERSION: &'static str = "v1beta2";
    const URL_PATH_SEGMENT: &'static str = "xxx";
    type Scope = k8s::ClusterResourceScope;
}
