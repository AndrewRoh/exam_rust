pub mod pb {
    include!("../../../gen/src/proto/streaming.rs");
}
use pb::{echo_client::EchoClient, EchoRequest};

//use futures::stream::Stream;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "http://172.20.41.70:50053";
    //let address = "http://127.0.0.1:50053";
    let mut client = EchoClient::connect(address).await?;

    streaming_echo(&mut client, 5).await;

    Ok(())
}

async fn streaming_echo(client: &mut EchoClient<tonic::transport::Channel>, num: usize) {
    let stream = client.
        server_streaming_echo(EchoRequest {
            message: "Andrew".into(),
        })
        .await
        .unwrap()
        .into_inner();

    let mut stream = stream.take(num);
    while let Some(item) = stream.next().await {
        println!("\t received: {}", item.unwrap().message);
    }
}