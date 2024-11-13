use calculator::{calculator_client::CalculatorClient, ComputeRequest};
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

        let result = client.compute(request).await;
        match result {
            Ok(response) => {
                println!("{}", response.into_inner().result);
            }
            Err(status) => {
                eprintln!("{}: {}", status.code(), status.message());
            }
        }
    }

    Ok(())
}
