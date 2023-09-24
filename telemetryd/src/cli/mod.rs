use std::net::{IpAddr, SocketAddr};

use clap::Parser;
use tracing::{error, info};

use crate::{config, grpc};

#[derive(Parser)]
#[clap(version)]
pub struct Telemetryd {
    /// Bind ip address
    #[clap(long, default_value = "0.0.0.0")]
    ip: IpAddr,
    /// Bind port
    #[clap(long, default_value_t = config::DEFAULT_GRPC_PORT)]
    port: u16,
}

pub fn parse() -> Telemetryd {
    Telemetryd::parse()
}

impl Telemetryd {
    pub async fn run(self) {
        let Self { ip, port } = self;

        let addr = SocketAddr::new(ip, port);

        info!(%addr, "Serve grpc server...");

        if let Err(err) = grpc::serve(addr).await {
            error!("{err:?}");
            std::process::exit(1);
        }
    }
}
