use std::{net::SocketAddr, time::Duration};

use tonic::transport::{self, Server};

use crate::metrics;

pub async fn serve(addr: SocketAddr) -> Result<(), transport::Error> {
    Server::builder()
        .timeout(Duration::from_secs(60))
        .trace_fn(|_req| tracing::info_span!("grpc.request"))
        .add_service(metrics::service())
        .serve(addr)
        .await
}
