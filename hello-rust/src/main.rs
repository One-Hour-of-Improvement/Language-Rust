#![allow(dead_code)]
use ferris_says::say;
use std::io::{stdout, BufWriter};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[derive(Debug)]
#[derive(Clone)]
struct Address{
    street: String,
    city: String,
    state: String,
    zip: String,
}

// impl Into<String> for Address {
//     fn into(self) -> String {
//         format!("{} {}, {}, {}", self.street, self.city, self.state, self.zip)
//     }
// }

impl From<Address> for String {
    fn from(address: Address) -> String {
        format!("{} {}, {}, {}", address.street, address.city, address.state, address.zip)
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    address: Vec<Address>,
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "data": "pong"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(ping)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn is_score_pass(score: u8) -> bool {
    let threshold:u8 = 75;
    if score >= threshold {
        return true;
    }
    return false;
}

fn inspect_number(number: i32) -> () {
    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }
}