use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};
use std::io::BufReader;

use super::{utils, crypt};

pub struct Sock {
    pub stream: TcpStream,
    pub key: String
}

impl Sock {

    pub fn listen(&mut self) -> String {
        let mut reader = BufReader::new(self.stream.try_clone().unwrap());
        let mut buf = "".to_owned();

        // Flush the buffer
        self.stream.flush().unwrap();

        // Read line
        reader.read_line(&mut buf).unwrap();

        // Remove newline
        buf = buf.replace("\n", "");

        // Return result
        return buf;
    }
    
    pub fn send(&mut self, msg: &String, nonce: &String) {
        self.stream.write(format!("{msg}|{nonce}").as_bytes()).unwrap();
    }

    pub fn close(&mut self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
    }

    pub fn manage(&mut self) {

        // Send our key
        self.stream.write(&self.key.as_bytes()).unwrap();

        // Manage the commands
        loop {
            let msg: String = self.listen();
            let values: Vec<&str> = msg.split("|").collect();

            let encrypted = values[0].to_owned();
            let sent_nonce = values[1].to_owned();

            let nonce = utils::random_nonce();

            println!("{}, {}, {}", &encrypted, &self.key, &sent_nonce);

            let decrypted =  crypt::decrypt(&encrypted, &self.key, &sent_nonce);
            
            self.send(&crypt::encrypt(&decrypted, &self.key, &nonce), &nonce);
        }
    }
    
}

pub fn connect(ip: &String, port: &String) -> TcpStream {
    TcpStream::connect(format!("{ip}:{port}")).unwrap()
}