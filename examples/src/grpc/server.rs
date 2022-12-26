#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:5001".parse()?;
    // let greeter = MyGreeter::default():
    println!("Hello Server, world!");
}
