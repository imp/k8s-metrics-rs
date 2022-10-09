use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PodMetrics {
    pub metadata: metav1::ObjectMeta,
    pub containers: Vec<Container>,
    pub timestamp: metav1::Time,
    pub window: String,
}

impl k8s::Resource for PodMetrics {
    const API_VERSION: &'static str = "metrics.k8s.io/v1beta1";
    const GROUP: &'static str = "metrics.k8s.io";
    const KIND: &'static str = "pod";
    const VERSION: &'static str = "v1beta1";
    const URL_PATH_SEGMENT: &'static str = "pods";
    type Scope = k8s::NamespaceResourceScope;
}

impl k8s::Metadata for PodMetrics {
    type Ty = metav1::ObjectMeta;

    fn metadata(&self) -> &<Self as k8s::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut <Self as k8s::Metadata>::Ty {
        &mut self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json as json;

    const POD: &str = r#"{
  "kind": "PodMetrics",
  "apiVersion": "metrics.k8s.io/v1beta1",
  "metadata": {
    "name": "metrics-server-6db985556d-nqbdz",
    "namespace": "kube-system",
    "creationTimestamp": "2022-10-09T11:51:23Z",
    "labels": {
      "k8s-app": "metrics-server",
      "pod-template-hash": "6db985556d"
    }
  },
  "timestamp": "2022-10-09T11:51:20Z",
  "window": "14.982s",
  "containers": [
    {
      "name": "metrics-server",
      "usage": {
        "cpu": "6082165n",
        "memory": "22272Ki"
      }
    }
  ]
}"#;

    #[test]
    fn resource() {
        let pod: PodMetrics = json::from_str(POD).unwrap();
        assert_eq!(
            pod.metadata.name.as_deref(),
            Some("metrics-server-6db985556d-nqbdz")
        );
        assert_eq!(pod.containers.len(), 1);
    }
}
