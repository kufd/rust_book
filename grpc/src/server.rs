use tonic::{transport::Server, Request, Response, Status};

use try_grpc::try_grpc_server::TryGrpc;
use try_grpc::try_grpc_server::TryGrpcServer;
use try_grpc::Request as TryRequest;
use try_grpc::Response as TryResponse;

pub mod try_grpc {
    tonic::include_proto!("try_grpc");
}

#[derive(Debug, Default)]
pub struct TryGrpcService {}

#[tonic::async_trait]
impl TryGrpc for TryGrpcService {
    async fn make_request(
        &self,
        request: Request<TryRequest>,
    ) -> Result<Response<TryResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = TryResponse {
            data: format!("try grpc response").into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = TryGrpcService::default();

    Server::builder()
        .add_service(TryGrpcServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
