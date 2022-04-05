//!
//! A little toy
//!

#![deny(warnings)]
#![deny(missing_docs)]

use sahw2::mcli;
use std::{
    io::{Read, Result, Write},
    net::{SocketAddr, TcpListener},
};

fn main() -> Result<()> {
    // parse parameters from cli
    let params = mcli::parse_args().expect("Invalid params");
    // create a new socket address from specified ipaddress and port number
    let socket = SocketAddr::new(params.ia, params.port);
    // create a TcpListener bound to provided socket
    let listener = TcpListener::bind(socket)?;
    // show used port
    println!("Listen on {}", listener.local_addr()?);
    // Wait for new client in block way
    let (mut stream, addr) = listener.accept()?;
    // print newly connected client address
    println!("We got a new comer {:?}", addr);
    let buf_len = 16;
    // allocate buffer to store received data
    let mut input = vec![0; buf_len];
    loop {
        // read data from data queue to buffer
        let size = stream.read(input.as_mut_slice())?;
        // if size is not zero, process received data, or say goodbye and exit
        if size != 0 {
            // write back all received data to client
            stream.write_all(&input[..size]).unwrap();
            // try to convert raw data to utf8 string
            match std::str::from_utf8(&input[..size]) {
                // if ok, print string to console
                Ok(data) => print!("{}", data),
                // or print raw data
                Err(_) => print!("{:?}", &input[..size]),
            }
        } else {
            // print goodbye
            println!("Goodbye, client {}!", addr);
            // break infinite loop and return
            break Ok(());
        }
    }
}
