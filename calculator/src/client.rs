use calculator::ComputeRequest;
use calculator::{calculator_client::CalculatorClient, ComputeResult, Error};
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
        match result.into_inner() {
            ComputeResult {
                error: None,
                result: Some(res),
            } => {
                println!("{res}");
            }
            ComputeResult {
                error: Some(error),
                result,
            } => {
                eprintln!(
                    "Server returned error: {}",
                    if let Ok(error) = Error::try_from(error) {
                        format!("{error:?}")
                    } else {
                        format!("{error}")
                    }
                );
                if let Some(res) = &result {
                    eprintln!("... with result: {res}");
                }
            }
            result => {
                eprintln!("Server returned invalid response {result:?}");
            }
        }
    }

    Ok(())
}
