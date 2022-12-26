pub mod greeter {
    include!("../../../gen/src/proto/greeter.rs");
}

use greeter::greeter_client::GreeterClient;
use greeter::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://172.20.41.70:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Andrew".into(),
    });

    let response = client.say_hello_world(request).await?;

    println!("rust grpc Response={:?}", response);

    Ok(())
}
