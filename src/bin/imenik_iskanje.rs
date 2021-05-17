use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::str; //string funkcije
use std::io;
use std::{thread, time}; //niti

fn main() -> std::io::Result<()> {
    let protocol: String = String::from("iskanje"); //protocol

    let mut buff: [u8; 256] = [0; 256];
    let mut input = String::new();

    loop {
        println!("Vnesi iskani niz!: ");
        let mut stream = UnixStream::connect("socket.sock")?;
        match io::stdin().read_line(&mut input) { //vnosi v standardni vhod
            Ok(_) => {
                if input !="" { //preverim za ctrl-d
                    /*let ten_millis = time::Duration::from_millis(6000); //za testiranje večih niti
                    thread::sleep(ten_millis);*/ 
                    let list = [protocol.as_str(), input.as_str()].join(" "); 
                    stream.write_all(  list.as_bytes())?;
                    stream.read(&mut buff).unwrap();
                    let response = str::from_utf8(&buff).unwrap();
                    println!("{}", response);
                    input = String::from("");
                }
                else {
                    break;
                }
            },
            Err(_) => break,
        }
    }

    Ok(())
}