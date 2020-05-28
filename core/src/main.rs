use tonic::{transport::Server, Request, Response, Status};

use crate::api::echo_service_server::{EchoService, EchoServiceServer};
use api::Message;

pub mod api {
    tonic::include_proto!("api");
}

#[derive(Debug, Default)]
pub struct EchoAPI {}

#[tonic::async_trait]
impl EchoService for EchoAPI {
    async fn echo(
        &self,
        request: Request<Message>,
    ) -> Result<Response<Message>, Status> {
        let message = request.into_inner();

        println!("received {:?}", message.value);

        Ok(Response::new(Message { value: "response".into() }))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:7000".parse()?;

    println!("listening on port {:?}", addr);

    Server::builder()
        .add_service(EchoServiceServer::new(EchoAPI::default()))
        .serve(addr)
        .await?;

    Ok(())
}