pub mod greeter {
    include!("../../../gen/src/proto/greeter.rs");
}

use greeter::greeter_client::GreeterClient;
use greeter::HelloRequest;
use rand::{Rng};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let address = "http://172.20.41.70:50051";
    let address = "http://127.0.0.1:50051";
    let mut client = GreeterClient::connect(address).await?;

    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let request = tonic::Request::new(HelloRequest {
            name: "Andrew".into(),
        });

        let response = client.say_hello_world(request).await?;

        let sec_20 = tokio::time::Duration::from_secs(rng.gen_range(170..200));

        println!("rust grpc Response={:?}, Sleep={:?}", response, sec_20);

        tokio::time::sleep(sec_20).await;
    }

    Ok(())
}
