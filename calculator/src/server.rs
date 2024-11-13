use tonic::{transport::Server, Request, Response, Status};

use calculator::calculator_server::{Calculator, CalculatorServer};
use calculator::{ComputeRequest, ComputeResult};

pub mod calculator {
    tonic::include_proto!("calculator");
}

#[derive(Debug, Default)]
struct MyCalculator {}

#[tonic::async_trait]
impl Calculator for MyCalculator {
    async fn compute(
        &self,
        request: Request<ComputeRequest>,
    ) -> Result<Response<ComputeResult>, Status> {
        println!("Got a compute request: {request:?}");

        let reply = ComputeResult {
            result: "42".into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let calculator = MyCalculator::default();

    Server::builder()
        .add_service(CalculatorServer::new(calculator))
        .serve(addr)
        .await?;

    Ok(())
}
