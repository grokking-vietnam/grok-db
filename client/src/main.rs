use std::io::stdin;
use std::io::{prelude::*, BufReader, BufWriter};
use std::net::TcpStream;

const ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    loop {
        let stream = TcpStream::connect(ADDRESS).expect("Could not connect to server");

        let mut request = String::new();
        stdin()
            .read_line(&mut request)
            .expect("Failed to read line");

        let mut writer = BufWriter::new(&stream);
        writer
            .write_all(request.as_bytes())
            .expect("Failed to write to stream");
        writer.flush().expect("Failed to flush stream");

        let mut buf_reader = BufReader::new(&stream);
        let mut response = String::new();
        buf_reader
            .read_to_string(&mut response)
            .expect("Failed to read from stream");
        println!("Server response: {}", response);
    }
}
