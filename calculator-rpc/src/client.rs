use calculator::calculator_client::CalculatorClient;
use calculator::ComputeRequest;

pub mod calculator {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CalculatorClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(ComputeRequest {
        expression: "1 + 1".into(),
    });

    let result = client.compute(request).await?;
    let res = &result.get_ref().result;
    println!("Server returned result: {res}");

    Ok(())
}
