use std::time::Duration;

use opentelemetry_proto::tonic::{
    collector::metrics::v1::{
        metrics_service_client::MetricsServiceClient, ExportMetricsServiceRequest,
    },
    metrics::v1::ResourceMetrics,
};

#[tokio::test]
async fn metrics() {
    let addr = "127.0.0.1:4387".parse().unwrap();

    tokio::spawn(telemetryd::grpc::serve(addr));

    let mut attempt = 0;
    let mut client = loop {
        attempt += 1;
        match MetricsServiceClient::connect("http://127.0.0.1:4387").await {
            Ok(client) => break client,
            Err(_) => tokio::time::sleep(Duration::from_secs(1)).await,
        }
        if attempt >= 5 {
            panic!("Failed to connect server");
        }
    };

    let resource_metrics = ResourceMetrics {
        resource: None,
        scope_metrics: vec![],
        schema_url: "https://opentelemetry.io/schemas/1.21.0".into(),
    };

    let request = tonic::Request::new(ExportMetricsServiceRequest {
        resource_metrics: vec![resource_metrics],
    });

    let _response = client.export(request).await.unwrap();
}
