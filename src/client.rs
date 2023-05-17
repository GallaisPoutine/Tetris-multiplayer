use crate::connection::*;

use std::io::*;
use std::sync::mpsc;
use std::thread;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub fn start(tx: &mpsc::Sender<String>, ip: String) {
    let co: Connection = Connection::new(false, ip);
    key_listener(&co);
    connection_listener(&co, tx);
}

fn key_listener(co: &Connection) {
    let mut reading_connection = co.try_clone().unwrap();
    thread::spawn(move || {
        let _stdout = stdout().into_raw_mode().unwrap();
        for c in stdin().keys() {
            let mut command: String = String::from("/");
            match c.unwrap() {
                Key::Char('z') => command.push('z'),
                Key::Char('q') => command.push('q'),
                Key::Char('s') => command.push('s'),
                Key::Char('d') => command.push('d'),
                Key::Char('a') => command.push('a'),
                Key::Char('e') => command.push('e'),
                Key::Char('r') => command.push('r'),
                Key::Esc => command.push('E'),
                _ => command.clear(),
            }
            reading_connection.write(&command);
            if command.eq(&String::from("/E")) {
                reading_connection.close_socket();
            }
            command.clear();
        }
    });
}

fn connection_listener(co: &Connection, tx: &mpsc::Sender<String>) {
    let mut reading_connection = co.try_clone().unwrap();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || loop {
        let msg = reading_connection.read();
        match msg.len() {
            0 => {
                println!(
                    "An error occurred, terminating connection with {}",
                    reading_connection.get_stream().peer_addr().unwrap()
                );
                reading_connection.close_socket();
            }
            2 => {
                decode(msg.as_str(), &tx1);
            }
            _ => panic!("msg.length != 2 || != 0"),
        }
    });
}

fn decode(function: &str, tx: &mpsc::Sender<String>) {
    let mut func: String = function.to_owned().to_string();
    assert_eq!(func.remove(0), '/');
    match func.as_str() {
        "D" => tx.send("D".to_owned().to_string()).unwrap(),
        _ => panic!("Character not found"),
    }
}
