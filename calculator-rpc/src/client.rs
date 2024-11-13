use calculator::calculator_client::CalculatorClient;
use calculator::ComputeRequest;
use tokio::io::{self, AsyncBufReadExt, BufReader};

pub mod calculator {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CalculatorClient::connect("http://[::1]:50051").await?;
    let stdin = BufReader::new(io::stdin());

    let mut lines = stdin.lines();
    while let Some(expression) = lines.next_line().await? {
        let request = tonic::Request::new(ComputeRequest { expression });

        let result = client.compute(request).await?;
        let res = &result.get_ref().result;
        println!("Server returned result: {res}");
    }

    Ok(())
}
