use tonic::{transport::Server, Request, Response, Status};

use calculator::calculator_server::{Calculator, CalculatorServer};
use calculator::{ComputeRequest, ComputeResult, Error};
use std::str::FromStr;

pub mod calculator {
    tonic::include_proto!("calculator");
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
        Err(Error::Parse)
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
                _ => return Err(Error::Parse),
            }
        }

        if operator.is_some() {
            return Err(Error::Parse);
        }

        a.ok_or(Error::Parse).map(|n| n.to_string())
    }
}

#[tonic::async_trait]
impl Calculator for MyCalculator {
    async fn compute(
        &self,
        request: Request<ComputeRequest>,
    ) -> Result<Response<ComputeResult>, Status> {
        println!("Got a compute request: {request:?}");

        let result = Self::evaluate(&request.into_inner().expression);

        let reply = match result {
            Ok(result) => ComputeResult {
                result: Some(result),
                error: None,
            },
            Err(error) => ComputeResult {
                result: None,
                error: Some(error.into()),
            },
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
