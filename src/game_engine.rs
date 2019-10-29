// IMPORTS

extern crate timer;
extern crate chrono;

use std::io::*;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use termion::{event::Key, raw::IntoRawMode, input::TermRead};

use crate::tetromino::*;
use crate::field::*;
use crate::graphics::*;
use crate::server::*;
use crate::client::*;


pub fn single_player() {
	let go_down_time : i64 = 1000;
	let _stdout = stdout().into_raw_mode().unwrap();

	let timer = timer::Timer::new();
	let tetromino = Arc::new(Mutex::new(Tetromino::build_tetromino()));

	show_single_player_game(&mut tetromino.lock().unwrap());

	let guard = {
		let t = tetromino.clone();
		timer.schedule_repeating(chrono::Duration::milliseconds(go_down_time), move || {
			t.lock().unwrap().down_blocking();
			show_single_player_game(&mut t.lock().unwrap());
		})
	};

	let mut quit = false;
	while tetromino.lock().unwrap().get_field().is_full() == false && quit == false{
		for c in stdin().keys() {
			match c.unwrap() {
				Key::Char('z')	=> tetromino.lock().unwrap().straight_down_blocking(),
				Key::Char('q')	=> tetromino.lock().unwrap().left(),
				Key::Char('s')	=> tetromino.lock().unwrap().down(),
				Key::Char('d')	=> tetromino.lock().unwrap().right(),
				Key::Char('a')	=> tetromino.lock().unwrap().left_rot(),
				Key::Char('e')	=> tetromino.lock().unwrap().right_rot(),
				Key::Char('r')	=> tetromino.lock().unwrap().switch(),
				Key::Esc		=> quit = true,
				_				=> println!(""),
			}
			show_single_player_game(&mut tetromino.lock().unwrap());
			break;
		}
	}
	drop(guard);
}

pub fn multi_player_local() {
	let mut number_of_lines1 : u32 = 0;
	let mut number_of_lines2 : u32 = 0;
	let go_down_time : i64 = 1000;
	let _stdout = stdout().into_raw_mode().unwrap();

	let timer = timer::Timer::new();
	let tetromino1 = Arc::new(Mutex::new(Tetromino::build_tetromino()));
	let tetromino2 = Arc::new(Mutex::new(Tetromino::build_tetromino()));

	show_multi_player_game(&mut tetromino1.lock().unwrap(), &mut tetromino2.lock().unwrap());

	let guard1 = {
		let t1 = tetromino1.clone();
		let t2 = tetromino2.clone();
		timer.schedule_repeating(chrono::Duration::milliseconds(go_down_time), move || {
			t1.lock().unwrap().down_blocking();
			t2.lock().unwrap().down_blocking();
			show_multi_player_game(&mut t1.lock().unwrap(), &mut t2.lock().unwrap());
		})
	};

	let mut quit = false;
	while tetromino1.lock().unwrap().get_field().is_full() == false && tetromino2.lock().unwrap().get_field().is_full() == false && quit == false{
		for c in stdin().keys() {
			match c.unwrap() {
				Key::Char('z')	=> tetromino1.lock().unwrap().straight_down_blocking(),
				Key::Char('q')	=> tetromino1.lock().unwrap().left(),
				Key::Char('s')	=> tetromino1.lock().unwrap().down(),
				Key::Char('d')	=> tetromino1.lock().unwrap().right(),
				Key::Char('a')	=> tetromino1.lock().unwrap().left_rot(),
				Key::Char('e')	=> tetromino1.lock().unwrap().right_rot(),
				Key::Char('r')	=> tetromino1.lock().unwrap().switch(),

				Key::Up			=> tetromino2.lock().unwrap().straight_down_blocking(),
				Key::Left		=> tetromino2.lock().unwrap().left(),
				Key::Down		=> tetromino2.lock().unwrap().down(),
				Key::Right		=> tetromino2.lock().unwrap().right(),
				Key::Char('i')	=> tetromino2.lock().unwrap().left_rot(),
				Key::Char('o')	=> tetromino2.lock().unwrap().right_rot(),
				Key::Char('p')	=> tetromino2.lock().unwrap().switch(),

				Key::Esc		=> quit = true,
				_				=> println!(""),
			}
			verify_destroyed_lines(tetromino1.lock().unwrap().get_field(), tetromino2.lock().unwrap().get_field(), &mut number_of_lines1);
			verify_destroyed_lines(tetromino2.lock().unwrap().get_field(), tetromino1.lock().unwrap().get_field(), &mut number_of_lines2);
			show_multi_player_game(&mut tetromino1.lock().unwrap(), &mut tetromino2.lock().unwrap());
			break;
		}
	}
	drop(guard1);
}

pub fn multi_player_online_host() {
	println!("Starting server !\n");

	let mut number_of_lines1 : u32 = 0;
	let mut number_of_lines2 : u32 = 0;
	let go_down_time : i64 = 1000;
	let _stdout = stdout().into_raw_mode().unwrap();
	let mut quit = false;

	let timer = timer::Timer::new();
	let tetromino1 = Arc::new(Mutex::new(Tetromino::build_tetromino()));
	let tetromino2 = Arc::new(Mutex::new(Tetromino::build_tetromino()));

	let (tx, rx) = mpsc::channel::<String>();

	key_listener_host(&tx);

	let mut server: Server = Server::new();
	server.start(tx);

	show_multi_player_game(&mut tetromino1.lock().unwrap(), &mut tetromino2.lock().unwrap());

	// Timer to make the tetrominos go down
	let guard1 = {
		let t1 = tetromino1.clone();
		let t2 = tetromino2.clone();
		timer.schedule_repeating(chrono::Duration::milliseconds(go_down_time), move || {
			t1.lock().unwrap().down_blocking();
			t2.lock().unwrap().down_blocking();
			show_multi_player_game(&mut t1.lock().unwrap(), &mut t2.lock().unwrap());
			// server.write("/D");
		})
	};

	while !tetromino1.lock().unwrap().get_field().is_full()
		&& !tetromino2.lock().unwrap().get_field().is_full()
		&& !quit {
		// Error thrown if receive queue empty
		let received = rx.try_recv();

		// Handling error case
		if received.is_ok() {
			match received.unwrap().as_str() {
				"z t1" => tetromino1.lock().unwrap().straight_down_blocking(),
				"q t1" => tetromino1.lock().unwrap().left(),
				"s t1" => tetromino1.lock().unwrap().down(),
				"d t1" => tetromino1.lock().unwrap().right(),
				"a t1" => tetromino1.lock().unwrap().left_rot(),
				"e t1" => tetromino1.lock().unwrap().right_rot(),
				"r t1" => tetromino1.lock().unwrap().switch(),
				"E t1" => {
					quit = true;
					// TODO Exit socket properly ?
					server.close_socket();
				},
				"z t2" => tetromino2.lock().unwrap().straight_down_blocking(),
				"q t2" => tetromino2.lock().unwrap().left(),
				"s t2" => tetromino2.lock().unwrap().down(),
				"d t2" => tetromino2.lock().unwrap().right(),
				"a t2" => tetromino2.lock().unwrap().left_rot(),
				"e t2" => tetromino2.lock().unwrap().right_rot(),
				"r t2" => tetromino2.lock().unwrap().switch(),
				"E t2" => {
					quit = true;
					// TODO Exit socket properly ?
					server.close_socket();
				},
				_   => (),
			}
		}

		verify_destroyed_lines(tetromino1.lock().unwrap().get_field(), 
								tetromino2.lock().unwrap().get_field(), 
								&mut number_of_lines1);
		verify_destroyed_lines(tetromino2.lock().unwrap().get_field(), 
								tetromino1.lock().unwrap().get_field(), 
								&mut number_of_lines2);
		show_multi_player_game(&mut tetromino1.lock().unwrap(), 
								&mut tetromino2.lock().unwrap());
	}
	drop(guard1);
}

pub fn multi_player_online_join() {
	let mut quit = false;

	// Get server IPv4 address
	let mut ip = String::new();
	print!("Please enter server IP address : ");
	let _=stdout().flush();
	stdin().read_line(&mut ip).expect("Did not enter a correct IP");
	if let Some('\n')=ip.chars().next_back() {
        ip.pop();
    }
    if let Some('\r')=ip.chars().next_back() {
        ip.pop();
    }

	// Starting queue
	let (tx, rx) = mpsc::channel::<String>();

	println!("Starting client !\n");
	start(&tx, String::from(ip));

	while !quit {
		// Error thrown if receive queue empty
		let received = rx.try_recv();

		// Handling error case
		if received.is_ok() {
			match received.unwrap().as_str() {
				"D" => (),// tetromino1.lock().unwrap().down(),
				"E" => {
					quit = true;
					// TODO Exit socket properly ?
				},
				_   => (),
			}
		}
	}

    // DELETE THIS GARBAGE !
    // loop {}
}

// PRIVATE FUNCTIONS

fn verify_destroyed_lines (field1 : &Field, field2 : &mut Field, number_of_lines : &mut u32) {
	let difference = field1.get_number_of_lines() - *number_of_lines;
	if difference > 0 && difference < 4 {
		field2.add_lines((difference - 1) as usize);
		field2.is_in_losszone();
	} else if difference == 4 {
		field2.add_lines((difference) as usize);
		field2.is_in_losszone();
	}
	*number_of_lines = field1.get_number_of_lines();
}

fn key_listener_host(tx: &mpsc::Sender<String>) {
	let tx1 = mpsc::Sender::clone(&tx);
	thread::spawn(move || 
	loop {
		for c in stdin().keys() {
			match c.unwrap() {
				Key::Char('z')	=> tx1.send("z t2".to_owned().to_string()).unwrap(),
				Key::Char('q')	=> tx1.send("q t2".to_owned().to_string()).unwrap(),
				Key::Char('s')	=> tx1.send("s t2".to_owned().to_string()).unwrap(),
				Key::Char('d')	=> tx1.send("d t2".to_owned().to_string()).unwrap(),
				Key::Char('a')	=> tx1.send("a t2".to_owned().to_string()).unwrap(),
				Key::Char('e')	=> tx1.send("e t2".to_owned().to_string()).unwrap(),
				Key::Char('r')	=> tx1.send("r t2".to_owned().to_string()).unwrap(),
				Key::Esc		=> tx1.send("E t2".to_owned().to_string()).unwrap(),
				_				=> (),
			}
			break;
		}
	});
}