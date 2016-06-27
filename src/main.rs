extern crate iron;
extern crate redis;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;

use rustc_serialize::json;

use std::io::prelude::*;
use std::process::Command;

use std::net::{SocketAddrV4, TcpStream, UdpSocket, TcpListener, Ipv4Addr};

use std::env;

mod router;
use router::Router;

#[derive(RustcEncodable, RustcDecodable)]
struct CountRequest {
    repo: String,
}

#[derive(RustcEncodable, RustcDecodable)]
struct CountResponse {
    body: String,
}

fn counter(request: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let request: CountRequest = json::decode(&payload).unwrap();
    println!("payloda : {}", payload);

    let counts = String::from_utf8(Command::new("gitcnt")
                                       .arg("--repo")
                                       .arg(request.repo.to_string())
                                       .output()
                                       .unwrap()
                                       .stdout)
                     .unwrap();
    println!(":  {}", counts);
    let resp = CountResponse { body: counts.to_string() };
    let payload = json::encode(&resp).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

fn main() {
    println!("start");
    let mut args = env::args();
    if let Some(firstop) = args.nth(1) {
        let mut mux = router::IronMux::new();
        mux.add(router::Method::POST, "count".to_string(), counter);
        let sp :Vec<&str> = firstop.split(":").collect();
        //let ips :Vec<&str> = sp[0].split(".").collect();
        let ip4 = (sp[0], sp[1].parse::<i32>().unwrap() as u16);
        Iron::new(mux).http(ip4).unwrap();
    }

}
