use std::io;
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener, Shutdown};
use std::str::from_utf8;



pub struct Connection {
    stream : TcpStream,
}

impl Connection {

    pub fn new(is_server : bool, ip : String) -> Connection {
        if is_server {
            Connection {
                stream : Connection::init_socket_serveur(),
            }
        } else {
            Connection {
                stream : Connection::init_socket_client(ip),
            }
        }
    }

    // PRIVATE FUNCTIONS USED FOR CONSTRUCTOR

    fn init_socket_serveur() -> TcpStream {
        let listener = TcpListener::bind("0.0.0.0:12345").unwrap();
        println!("Server listening on port 12345");

        match listener.accept() {
            Ok((_socket, addr)) => {
                println!("New connection caught: {:?}", addr);
                _socket
            }
            Err(e) => panic!("Error accepting client: {:?}", e),
        }
    }

    fn init_socket_client(mut ip : String) -> TcpStream {
        ip.push_str(":12345");
        match TcpStream::connect(ip) {
            Ok(stream) => {
                println!("Successfully connected to server in port 12345");
                stream
            },
            Err(e) => {
                panic!("Failed to connect: {}", e);
            }
        }
    }

    // PUBLIC FUNCTIONS

    pub fn get_stream(&self) -> &TcpStream {
        &self.stream
    }

    pub fn close_socket(&mut self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
        println!("Terminated.");
    }

    pub fn write<'a>(&mut self, msg: &'a str) {
        self.stream.write(msg.as_bytes()).unwrap();
        println!("Sent message: {}\r\n", msg);
    }

    pub fn read(&mut self) -> String {
        let mut buf = [0; 2];
        self.stream.read_exact(&mut buf).unwrap();
        let buf_str = from_utf8(&buf).unwrap().to_owned().to_string();
        println!("Message received: {}\r\n", buf_str);
        buf_str
    }

    // Useless for now

    // pub fn handle_client(&mut self) {
    //     let buf = [0 as u8; 50];
    //     while match self.read().len() {
    //         0 => {
    //             // print stream input
    //             println!("{}", from_utf8(&buf).unwrap());

    //             true
    //         },
    //         _ => {
    //             println!("An error occurred, terminating connection with {}", self.stream.peer_addr().unwrap());
    //             self.close_socket();
    //             false
    //         }
    //     } {}
    // }

    pub fn try_clone(&self) -> io::Result<Connection> {
        let stream = self.stream.try_clone()?;
        Ok(Connection {stream: stream})
    }
}