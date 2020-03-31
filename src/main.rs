use actix_web::{web, error, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};

mod parser;
mod expressiontree;

use parser::parse;

use std::env;
use std::io::{self,BufRead};

use serde::Serialize;

#[derive(Serialize)]
struct Roll{
    outcome: expressiontree::DiceExpression,
    result: Vec<i64>,
    size: usize,
    trivial: bool,
}

async fn roll(req: HttpRequest) -> impl Responder {
    let expression = req.match_info().get("roll").unwrap_or("d6");
    let parsed_expression = parse(expression);
    match  parsed_expression{
        Ok(expression) => {
            if expression.size() <= 2001 {
                    let outcome = expression.outcome();
                    let roll = Roll{
                        outcome: outcome.clone(),
                        result: outcome.roll(),
                        size: outcome.size(),
                        trivial: outcome.trivial(),
                    };
                    Ok(HttpResponse::Ok().json(roll))
                } else {
                    Err(error::ErrorBadRequest("Expression too large"))
                }
        },
        Err(_) => Err(error::ErrorBadRequest("Malformed Request"))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    if env::args().any(|s| s=="cmd") {
        println!("CMD mode");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(result) = parse(&line.unwrap()){
                let roll = result.outcome();
                let serialized = serde_json::to_string(&roll).unwrap();
                println!("{} = {:?} , size: {}",serialized,roll.roll(),roll.size());
            }
        }
        Ok(())
    }
    else {
        HttpServer::new(|| App::new()
        .route(r"/roll/{roll}", web::get().to(roll)))
            .bind("127.0.0.1:6810")?
            .run()
            .await
    }
}
