use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::str; //string funkcije

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("socket.sock")?; //povezava na socket
    let protocol: String = String::from("zapri");
    stream.write_all(  protocol.as_bytes())?; //pošljem argumente + err handler -?
    let mut buff: [u8; 256] = [0; 256];
    stream.read(&mut buff).unwrap(); //preberem odgovor v buffer
    let response = str::from_utf8(&buff).unwrap(); //spremenim v string
    println!("{}", response);
    Ok(())
}