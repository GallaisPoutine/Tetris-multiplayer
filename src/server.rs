use crate::connection::*;

use std::str;
use std::sync::mpsc;
use std::thread;

pub struct Server {
    co : Connection,
    //tx : mpsc::Sender<String>,
}

impl Server {

    pub fn new() -> Server { //tx : mpsc::Sender<String>) -> Server {
        Server {
            co : Connection::new(true, String::new()), 
        }
    }

    pub fn start(&self, tx : mpsc::Sender<String>) {
        let mut connection = self.co.try_clone().unwrap();
    	let tx1 = mpsc::Sender::clone(&tx);

        thread::spawn(move|| 
            loop {
                let msg = connection.read();
                match msg.len() {
                    0 => {
                        println!("An error occurred, terminating connection with {}", 
                                connection.get_stream().peer_addr().unwrap());
                        connection.close_socket();
                    },
                    2 => {
                        Server::decode(msg.as_str(), &tx1);
                    },
                    _ => panic!("msg.length != 2 || != 0"),
                }
        });
    }

    fn decode (function: &str, tx: &mpsc::Sender<String>) {
        let mut func : String = function.to_owned().to_string();
        assert_eq!(func.remove(0), '/');
        match func.as_str() {
            "z" => tx.send("z".to_owned().to_string()).unwrap(),
            "q" => tx.send("q".to_owned().to_string()).unwrap(),
            "s" => tx.send("s".to_owned().to_string()).unwrap(),
            "d" => tx.send("d".to_owned().to_string()).unwrap(),
            "a" => tx.send("a".to_owned().to_string()).unwrap(),
            "e" => tx.send("e".to_owned().to_string()).unwrap(),
            "r" => tx.send("r".to_owned().to_string()).unwrap(),
            "E" => tx.send("E".to_owned().to_string()).unwrap(),
            _   => panic!("Character not found"),
        }
    }

    pub fn write(&mut self, msg: & str) {
        self.co.write(msg);
    }
}