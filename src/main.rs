mod pb {
    tonic::include_proto!("test");
}

use std::time::Duration;

use pb::{
    pong::Payload,
    tester_client::TesterClient,
    tester_server::{Tester, TesterServer},
    Ping, Pong,
};
use tonic::{transport::Server, Status};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let tester_server = TesterServer::new(TesterImpl);
        Server::builder()
            .add_service(tester_server)
            .serve("127.0.0.1:9000".parse().unwrap())
            .await
            .unwrap();
    });

    tokio::time::sleep(Duration::from_secs(1)).await;

    let conn = tonic::transport::Endpoint::new("http://127.0.0.1:9000")
        .unwrap()
        .connect()
        .await
        .unwrap();

    let mut tester_client = TesterClient::new(
        ServiceBuilder::new()
            // .layer(tower_http::trace::TraceLayer::new_for_grpc()) // this layer does not work
            .service(conn),
    );

    let response = tester_client.test(Ping {}).await.unwrap();

    assert_eq!(
        response.into_inner().payload,
        Some(Payload::Raw("pong".into()))
    );
}

#[derive(Clone)]
pub struct TesterImpl;

#[async_trait::async_trait]
impl Tester for TesterImpl {
    async fn test(&self, _req: tonic::Request<Ping>) -> Result<tonic::Response<Pong>, Status> {
        Ok(tonic::Response::new(Pong {
            payload: Some(Payload::Raw("pong".into())),
        }))
    }
}
