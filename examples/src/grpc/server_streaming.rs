pub mod pb {
    include!("../../../gen/src/proto/streaming.rs");
}

use std::{time::Duration, pin::Pin};
use pb::{echo_server::Echo, EchoRequest, EchoResponse};
use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status, Streaming};

type EchoResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send>>;

#[derive(Debug, Default)]
pub struct EchoServer {}

#[tonic::async_trait]
impl Echo for EchoServer {
    async fn unary_echo(&self, _: tonic::Request<EchoRequest>) -> EchoResult<EchoResponse> {
        Err(Status::unimplemented("not implemented"))
    }

    type ServerStreamingEchoStream = ResponseStream;

    async fn server_streaming_echo(&self, req: Request<EchoRequest>) -> EchoResult<Self::ServerStreamingEchoStream> {
        println!("EchoServer::server_streaming_echo");
        println!("\tclient connected from: {:?} {}", req.remote_addr(), req.get_ref().message);

        let repeat = std::iter::repeat(EchoResponse {
            message: req.into_inner().message,
        });
        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_micros(200)));

        let (tx,rx) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {

                    }
                    Err(_item) => {
                        break;
                    }
                }
            }
            println!("\tclient disconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output_stream) as Self::ServerStreamingEchoStream))
    }
    
    async fn client_streaming_echo(&self, request: Request<Streaming<EchoRequest>>) -> Result<Response<EchoResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }
    
    type BidirectionalStreamingEchoStream = ResponseStream;

    async fn bidirectional_streaming_echo(&self, request: Request<Streaming<EchoRequest>>) -> Result<Response<Self::BidirectionalStreamingEchoStream>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50053".parse()?;//.to_socket_addrs().unwrap().next().unwrap();
    println!("Listening {:?}", &addr);

    let server = EchoServer{};

    Server::builder()
        .add_service(pb::echo_server::EchoServer::new(server))
        .serve(addr)
        .await
        .unwrap();

    Ok(())
}
