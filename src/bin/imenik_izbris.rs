use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::env;
use std::str; //string funkcije

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("socket.sock")?;
    let args: Vec<String> = env::args().collect();
    let number = &args[1];
    let protocol: String = String::from("izbris");
    let list = [protocol.as_str(), number.as_str()].join(" ");
    stream.write_all(  list.as_bytes())?;
    let mut buff: [u8; 256] = [0; 256];
    stream.read(&mut buff).unwrap();
    let response = str::from_utf8(&buff).unwrap();
    println!("{}", response);
    Ok(())
}
