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
    let result = parse(s);
    match result {
        Ok(roll_result) => json!({"result":roll_result}),
        Err(_) => json!({})
    }
}

fn main() {
    if env::args().any(|s| s=="--cmd") {
        println!("CMD mode");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(result) = parse(line.unwrap()){
                println!("{}",result);
            }
        }
    }
    else {
        rocket::ignite().mount("/", routes![index,roll]).launch();
    }
}
