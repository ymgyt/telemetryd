use opentelemetry_proto::tonic::collector::metrics::v1::{
    metrics_service_server::{MetricsService as ProtoMetricsService, MetricsServiceServer},
    ExportMetricsServiceRequest, ExportMetricsServiceResponse,
};

#[derive(Default)]
pub struct MetricsService {}

#[tonic::async_trait]
impl ProtoMetricsService for MetricsService {
    async fn export(
        &self,
        request: tonic::Request<ExportMetricsServiceRequest>,
    ) -> Result<tonic::Response<ExportMetricsServiceResponse>, tonic::Status> {
        tracing::info!("{request:#?}");

        let response = tonic::Response::new(ExportMetricsServiceResponse {
            partial_success: None,
        });

        Ok(response)
    }
}

pub fn service() -> MetricsServiceServer<MetricsService> {
    MetricsServiceServer::new(MetricsService::default())
}
