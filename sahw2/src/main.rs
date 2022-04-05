//!
//! A little toy
//!

#![deny(warnings)]
#![deny(missing_docs)]

use sahw2::mcli;
use std::{
    io::{Read, Result},
    net::{SocketAddr, TcpListener},
};

fn main() -> Result<()> {
    let params = mcli::parse_args().expect("Invalid params");
    let socket = SocketAddr::new(params.ia, params.port);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listen on {}", port);
    let (mut stream, addr) = listener.accept()?;
    println!("We got a new comer {:?}", addr);
    println!("{:?} is sending data:", addr);
    let mut input = String::new();
    let _ = stream.read_to_string(&mut input)?;
    println!("{}", input);
    Ok(())
}
