use std::{io::Write, net::TcpStream};

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(address: &str) -> Connection {
        Connection {
            stream: TcpStream::connect(address).expect("Could not connect to the receiver!"),
        }
    }

    pub fn send_command(mut self, cmd: &str) {
        self.stream.write(format!("{}\r", cmd).as_bytes());
        // println!("{}", cmd);
    }
}
