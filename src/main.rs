#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod parser;
use parser::parse;
use std::str;
#[get("/")]
fn index() -> &'static str {
    ""
}

#[get("/roll/<s>")]
fn roll(s:String) -> Option<String> {
    let result = parse(s);
    match result {
        Ok(expression) => Some(expression.to_string()),
        Err(_) => None
    }
    // if let Ok(result) = p.parse(ss.as_bytes()) {
    //     Some(result.eval().to_string())
    // }else {
    //     None
    // }
}


fn main() {
    rocket::ignite().mount("/", routes![index,roll]).launch();


    // let expression = parser::dice_parser();
    // if let Ok(e) = expression.parse(b"d10 - d10"){
    //     println!("Expression {:?}",e);
    //     println!("Result {:?}",e.eval());
    // }
}
