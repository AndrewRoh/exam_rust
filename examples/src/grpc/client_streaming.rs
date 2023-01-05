pub mod pb {
    include!("../../../gen/src/proto/streaming.rs");
}

use std::time::Duration;
use pb::{echo_client::EchoClient, EchoRequest};
use tokio_stream::{Stream, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let address = "http://172.20.41.70:50053";
    let address = "http://127.0.0.1:50053";
    let mut client = EchoClient::connect(address).await?;

    println!("Start Streaming echo:");
    streaming_echo(&mut client, 4).await;
    println!("End Streaming echo");

    // tokio::time::sleep(Duration::from_secs(1)).await;
    //
    // println!("\r\nBidirectional stream echo:");
    // bidirectional_streaming_echo(&mut client, 17).await;
    // println!("End Bidirectional Stream echo");
    //
    // tokio::time::sleep(Duration::from_secs(1)).await;
    //
    // println!("\r\nBidirectional stream echo:");
    // bidirectional_streaming_echo_throttle(&mut client, Duration::from_secs(2)).await;
    // println!("End Bidirectional Stream echo");

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

fn echo_requests_iter() -> impl Stream<Item = EchoRequest> {
    tokio_stream::iter(1..usize::MAX).map(|i|EchoRequest {
        message: format!("msg {:02}", i)
    })
}

async fn bidirectional_streaming_echo(client: &mut EchoClient<tonic::transport::Channel>, num: usize) {
    let in_stream = echo_requests_iter().take(num);

    let response = client.bidirectional_streaming_echo(in_stream)
        .await.unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        println!("\treceived message: `{}`", received.message);
    }
}

async fn bidirectional_streaming_echo_throttle(client: &mut EchoClient<tonic::transport::Channel>, dur: Duration) {
    let in_stream = echo_requests_iter().throttle(dur);

    let response = client.bidirectional_streaming_echo(in_stream)
        .await.unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        println!("\treceived message: `{}`", received.message);
    }
}