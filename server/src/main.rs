use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};

const ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!(
                    "Received a connection! - {}:{}",
                    stream.peer_addr().unwrap().ip(),
                    stream.peer_addr().unwrap().port()
                );
                handle_client(stream);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    drop(listener)
}

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut request = String::new();
    reader.read_line(&mut request).expect("could not read");
    println!("Request: {}", request);

    let mut writer = BufWriter::new(&stream);
    let response = "OK pong\n";
    writer
        .write_all(response.as_bytes())
        .expect("could not write");
    writer.flush().expect("could not flush");
}
