use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tonic::{transport::Server, Request, Response, Status};

use trustlog_lib::opentelemetry::proto::{
    collector::logs::v1::{
        logs_service_server::{LogsService, LogsServiceServer},
        ExportLogsServiceRequest, ExportLogsServiceResponse,
    },
    logs::v1::ResourceLogs,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listen_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9300);
    let grpc_service = LogsServiceServer::new(Service { ack: false });

    println!("server listening on {}", listen_addr);

    Server::builder()
        .add_service(grpc_service)
        .serve(listen_addr)
        .await?;

    Ok(())
}

#[derive(Clone)]
pub struct Service {
    pub ack: bool,
}

#[tonic::async_trait]
impl LogsService for Service {
    async fn export(
        &self,
        request: Request<ExportLogsServiceRequest>,
    ) -> Result<Response<ExportLogsServiceResponse>, Status> {
        let logs: Vec<ResourceLogs> = request.into_inner().resource_logs;

        let count = logs.len();
        println!("got {} logs", count);

        Ok(Response::new(ExportLogsServiceResponse {
            partial_success: None,
        }))
    }
}
