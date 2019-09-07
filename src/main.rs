#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
use rocket_contrib::json::JsonValue;

mod parser;
use parser::parse;

use std::str;


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
    rocket::ignite().mount("/", routes![index,roll]).launch();
}
