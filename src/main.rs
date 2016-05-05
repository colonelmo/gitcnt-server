extern crate iron;
extern crate redis;
extern crate rustc_serialize;


use iron::prelude::*;
use iron::status;

use redis::Commands;

use rustc_serialize::json;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::process::Command;

#[derive(RustcEncodable, RustcDecodable)]
struct CountRequest{
    repo: String,
}

#[derive(RustcEncodable, RustcDecodable)]
struct CountResponse{
    body: String,
}

fn counter(request: &mut Request)-> IronResult<Response>{
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let request :CountRequest = json::decode(&payload).unwrap();
    println!("payloda : {}", payload); 
   
    let counts = String::from_utf8(
        Command::new("gitcnt")
        .arg("--repo")
        .arg(request.repo.to_string())
        .output()
        .unwrap()
        .stdout
        )
        .unwrap();
    println!(":  {}", counts);
    let resp = CountResponse{body : counts.to_string()};
    let payload = json::encode(&resp).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

fn main() {
    println!("start");
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = client.get_connection().unwrap();

    Iron::new(counter).http("localhost:80").unwrap();
    println!("serving on 80");
}
