use k8s_metrics::v1beta1 as metricsv1;
use kube::api;

async fn pod_metrics(
    client: &kube::Client,
    namespace: &str,
) -> kube::Result<Vec<metricsv1::PodMetrics>> {
    let lp = api::ListParams::default();
    api::Api::<metricsv1::PodMetrics>::namespaced(client.clone(), namespace)
        .list(&lp)
        .await
        .map(|list| list.items)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = kube::Client::try_default().await?;
    pod_metrics(&client, "default")
        .await?
        .into_iter()
        .for_each(|pm| println!("{pm:?}"));

    Ok(())
}
