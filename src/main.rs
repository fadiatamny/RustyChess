mod models;
use models::*;
mod game;

fn main() {

    let r = request::Method::try_from("test");
    match r {
        Ok(m) => println!("This is cool {:?}", m),
        Err(e) => println!("{}", errors::APIError::internal_server_error(e)), 
    }
    println!("Hello, world!");
}
