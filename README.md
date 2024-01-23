# Kubernetes Metrics API Resource definitions

Portions of the code are copied from other projects, like kdash

# Usage example

```rust
use k8s_metrics::v1beta1 as metricsv1;
use kube::api;

async fn pod_metrics(client: &kube::Client, namespace: &str) -> kube::Result<Vec<metricsv1::PodMetrics>> {
    let lp = api::ListParams::default();
    api::Api::<metricsv1::PodMetrics>::namespaced(client.clone(), namespace)
        .list(&lp)
        .await
        .map(|list| list.items)
}
```
