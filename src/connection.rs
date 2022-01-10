use die_exit::*;

use std::io::{prelude::*, BufReader};
use std::io::{Error, Write};
use std::net::{SocketAddrV4, TcpStream};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::spawn;
use std::time::Duration;

use crate::modules::parse_response;

pub fn connect(address: &str, fake: bool) -> Box<dyn SocketConnection> {
    if fake {
        Box::new(FakeConnection {})
    } else {
        Box::new(RealConnection::new(address))
    }
}

pub struct RealConnection {
    stream: Arc<TcpStream>,
}

impl RealConnection {
    pub fn new(ip: &str) -> RealConnection {
        RealConnection {
            stream: Arc::new(
                TcpStream::connect_timeout(
                    &SocketAddrV4::from_str(ip).die("Bad IP address").into(),
                    Duration::from_secs(1), // TODO: add cli option for this
                )
                .die("Could not connect to reciever"),
            ),
        }
    }
}

pub struct FakeConnection {}

pub trait SocketConnection {
    fn send(&mut self, command: &str) -> Result<usize, Error>;
    fn start_listen(&mut self);
}

impl SocketConnection for RealConnection {
    fn send(&mut self, command: &str) -> Result<usize, Error> {
        self.stream
            .deref()
            .write(format!("{}\r\n", command).as_bytes())
    }

    fn start_listen(&mut self) {
        let stream = self.stream.clone();
        spawn(move || loop {
            let mut data = Vec::new();
            BufReader::new(stream.deref())
                .read_until(b'\r', &mut data)
                .die("Could not listen");
            data.pop(); // Last char is '\r'
            let response = String::from_utf8_lossy(&data).to_string();
            println!("{}", parse_response(&response).unwrap_or(response))
        });
    }
}

impl SocketConnection for FakeConnection {
    fn send(&mut self, command: &str) -> Result<usize, Error> {
        println!("Send command: {}", command);
        Ok(command.len().into())
    }

    fn start_listen(&mut self) {
        println!("Starting listening process");
    }
}
