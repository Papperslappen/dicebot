#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
use rocket_contrib::json::JsonValue;

mod parser;
use parser::parse;

use std::str;
use std::env;
use std::io::{self,BufRead};


#[get("/")]
fn index() -> &'static str {
    ""
}

#[get("/roll/<s>")]
fn roll(s:String) -> JsonValue {
    let parsed_expression = parse(s);
    match  parsed_expression{
        Ok(expression) => json!({"result":expression.eval(),"trivial":expression.trivial()}),
        Err(_) => json!({})
    }
}

fn main() {
    if env::args().any(|s| s=="cmd") {
        println!("CMD mode");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(result) = parse(line.unwrap()){
                println!("{} bounds: {:?}",result.eval(),result.bounds());
            }
        }
    }
    else {
        rocket::ignite().mount("/", routes![index,roll]).launch();
    }
}
