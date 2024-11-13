use calculator::{
    calculator_server::{Calculator, CalculatorServer},
    ComputeRequest, ComputeResult,
};
use std::str::FromStr;
use thiserror::Error;
use tonic::{transport::Server, Request, Response, Status};

pub mod calculator {
    tonic::include_proto!("calculator");
}

#[derive(Error, Debug)]
enum Error {
    #[error("invalid token")]
    InvalidToken,
    #[error("invalid expression")]
    InvalidExpression,
}

#[derive(Clone, Copy)]
#[non_exhaustive]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

enum Token {
    Number(i64),
    Operator(Op),
}

impl FromStr for Token {
    type Err = Error;
    fn from_str(tok: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = i64::from_str(tok) {
            return Ok(Token::Number(n));
        }
        match tok {
            "+" => return Ok(Token::Operator(Op::Add)),
            "-" => return Ok(Token::Operator(Op::Sub)),
            "*" => return Ok(Token::Operator(Op::Mul)),
            "/" => return Ok(Token::Operator(Op::Div)),
            _ => {}
        }
        Err(Error::InvalidToken)
    }
}

#[derive(Debug, Default)]
struct MyCalculator {}

impl MyCalculator {
    fn parse(expression: &str) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        for tok in expression.split_ascii_whitespace() {
            if tok.is_empty() {
                continue;
            }
            tokens.push(Token::from_str(tok)?);
        }

        Ok(tokens)
    }

    fn evaluate(expression: &str) -> Result<String, Error> {
        let tokens = Self::parse(expression)?;

        let mut a: Option<i64> = None;
        let mut operator: Option<Op> = None;
        for tok in tokens {
            match (&mut a, operator, tok) {
                (None, None, Token::Number(b)) => a = Some(b),
                (Some(_), None, Token::Operator(o)) => operator = Some(o),
                (Some(a), Some(op), Token::Number(b)) => {
                    match op {
                        Op::Add => *a += b,
                        Op::Sub => *a -= b,
                        Op::Mul => *a *= b,
                        Op::Div => *a /= b,
                    };
                    operator = None;
                }
                _ => return Err(Error::InvalidExpression),
            }
        }

        if operator.is_some() {
            return Err(Error::InvalidExpression);
        }

        a.ok_or(Error::InvalidExpression).map(|n| n.to_string())
    }
}

#[tonic::async_trait]
impl Calculator for MyCalculator {
    async fn compute(
        &self,
        request: Request<ComputeRequest>,
    ) -> Result<Response<ComputeResult>, Status> {
        println!("Got a compute request: {request:?}");

        Self::evaluate(&request.into_inner().expression)
            .map(|result| Response::new(ComputeResult { result }))
            .map_err(|error| Status::invalid_argument(error.to_string()))
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
