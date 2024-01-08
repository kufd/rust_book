use try_grpc::try_grpc_client::TryGrpcClient;
use try_grpc::Request as TryRequest;

pub mod try_grpc {
    tonic::include_proto!("try_grpc");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TryGrpcClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(TryRequest {
        data: "try grpc request".to_owned(),
    });

    let response = client.make_request(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
