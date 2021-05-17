use lazy_static::lazy_static;
use std::io;
use std::os::unix::net::{UnixStream,UnixListener}; //unix vrata
use std::thread; //niti
use std::io::prelude::*; //ne delajo prav unix vrata brez tega
use std::str; //string funkcije
use std::fs; //za izbris datoteke
use std::path::Path; //if stavek za brisanje socktea
use std::sync::Mutex;
use std::thread::JoinHandle;


struct Contact { //struktura
    name: String,
    surname: String,
    number: String,
}

lazy_static! {
    static ref contacts : Mutex < Vec <Contact> > = Mutex::new(vec![]); //globalno deklariram vektor 
    static ref FIN : Mutex <bool> = Mutex::new(false);
}

fn vpis( name: String, surname: String, number: String) -> String {

    if name.chars().count() > 101 || number.chars().count() > 31 || surname.chars().count() > 101 { //preverimo da podatki niso preveliki
        println!("vpis neuspeh");
        return "vpis neuspeh".to_owned();
    }

    for contact in contacts.lock().unwrap().iter() { //skozi celi vektor

        if number == contact.number { //preverimo če je telefonska že v imeniku
            println!("vpis neuspeh");
            return "vpis neuspeh".to_owned();
        }
    }

    contacts.lock().unwrap().push(Contact{ name: name, surname: surname, number: number }); //če ni jo dodamo
    println!("vpis uspeh");
    return "vpis uspeh".to_owned();
}

fn izbris( number: String) -> String {

    let mut con = contacts.lock().unwrap(); //vektor tukopiram v novo spremenljivko, zato, ker ga ne smem večrakt lockat

    let mut i=0;

    for contact in con.iter() { 

        if number == contact.number { //če se nahaj v imeniku izbrišemo tisti index

            con.remove(i); //tu bi ga drugič moral če ne bi bilo v spremenljivki
            println!("izbris uspeh");
            return "izbris uspeh".to_owned();
        }

        i=i+1; //če ni povečamo index da vemo katerega izbrisati

    }
    println!("izbris neuspeh");
    return "izbris neuspeh".to_owned();
}

fn izpis( number: String) -> String {

    for contact in contacts.lock().unwrap().iter() {

        if number == contact.number { //če najdemo telefonsko spremeni kontakt v string in ga vrnem
            
            let con = ["izpis", contact.name.as_str(), contact.surname.as_str(), contact.number.as_str()].join(" "); // vse skupaj združim v eno spremenljivko
            println!("{}",con);
            return con;
        }

    }
    println!("izpis neuspeh");
    return "izpis neuspeh".to_owned();
}

fn iskanje(data: String) -> String {

    let con = contacts.lock().unwrap();
    let msg = data.trim_matches(char::from('\n')); //odstranim \n ki nastane zaradi console inputa
    let mut list: String = String::from("iskanje");
    let mut i =0;

    for contact in con.iter() {

        if contact.name.contains(msg) || contact.surname.contains(msg)  { //če se input nahaja v imenu ali priimku izpišem vse podatke
            let con = [contact.name.as_str(), contact.surname.as_str(), contact.number.as_str()].join(" ");
            list = format!("{}\n{}", list, con);
            i=i+1;
        }

    }
    println!("iskanje Stevilo zadetkov: {}",i);
    return list;
}

fn zapri() -> String{
    println!("zapri uspeh");
    *(FIN.lock().unwrap())=true;
    return "zapri uspeh".to_owned();
}

fn handle_client(mut stream: UnixStream) -> std::io::Result<()> {
    let mut buff: [u8; 256] = [0; 256]; //init bufferja
    stream.read(&mut buff).unwrap(); //beremo kaj nam je poslal imenik
    let msg = str::from_utf8(&buff).unwrap(); //spremenimo v string
    let msg = msg.trim_matches(char::from(0));
    let mut args  = Vec::new(); //init vektorja argumentov
    let mut j=0; //dodatni index za lažje določanja konca in začetka besede

    if msg != ""{
        for (i, c) in msg.chars().enumerate() { //znak c in indeks i
            if c == ' ' {
                let arg = msg[j..i].to_owned(); //argument dobi tako da ve njegov začetek in konec
                args.push(arg);
                j=i+1; 
            }
        }
        args.push(msg[j..(msg.chars().count())].to_owned()); //zadnji arg bo od j do konca

        let mut response: String = String::from("");

        match args[0].as_str() { // case stavek za protokole
            "vpis" => response = vpis( args[1].to_string(), args[2].to_string(), args[3].to_string()),
            "izbris" => response = izbris(args[1].to_string()),
            "izpis" => response = izpis(args[1].to_string()),
            "zapri" => response = zapri(),
            "iskanje" => response = iskanje( args[1].to_string()),
            _ => println!("Neznana komanda"),
        }
        stream.write_all(  response.as_bytes())?; //pošljem ukaz in rezultat
    }
    Ok(())

}

fn main() -> std::io::Result<()>{

    if Path::new("socket.sock").exists(){
        fs::remove_file("socket.sock")?; //izbrišem socket če že obstaja
    }

    let mut threads : Vec <JoinHandle<std::result::Result<(), std::io::Error>> > = Vec::new(); //vektor, ki shranjuje niti
    let listener = UnixListener::bind("socket.sock").unwrap(); //naredim sokcet za komunikacijo
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    for stream in listener.incoming() { //poslušam za odjemalce
        let data = FIN.lock().unwrap();
        if !*data { //z globalno spremenljivko preverimo če je program zaključen
                match stream {
                    Ok(stream) => {
                        let t = thread::spawn(|| handle_client(stream)); //če je odjemalec ok grem v funkcijo handle_client, uporaba niti
                        threads.push(t); //vnesem nit v vektor
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => panic!("Prišlo do IO napake: {}", e),
                }
            }
        else{
            break;
        }
    }
    for t in threads{ //zapiranje vseh niti
        t.join().unwrap()?;
    }
    Ok(())
}
