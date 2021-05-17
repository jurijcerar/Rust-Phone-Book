use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::env;
use std::str; //string funkcije

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("socket.sock")?; //povezava na socket
    let args: Vec<String> = env::args().collect();
    let number = &args[1];
    let protocol: String = String::from("izpis");
    let list = [protocol.as_str(), number.as_str()].join(" "); //zdruzim argumente
    stream.write_all(  list.as_bytes())?; //pošljem argumente + err handler -?
    let mut buff: [u8; 256] = [0; 256];
    stream.read(&mut buff).unwrap(); //preberem odgovor v buffer
    let response = str::from_utf8(&buff).unwrap(); //spremenim v string
    println!("{}", response);
    Ok(())
}