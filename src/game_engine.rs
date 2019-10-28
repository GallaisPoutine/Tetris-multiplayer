// IMPORTS

extern crate timer;
extern crate chrono;

//use timer::Timer;
//use std::thread;

use termion::{event::Key, raw::IntoRawMode, input::TermRead};
use std::io::*;
use std::sync::{Arc, Mutex, mpsc};

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
	// let _stdout = stdout().into_raw_mode().unwrap();

	let timer = timer::Timer::new();
	let tetromino1 = Arc::new(Mutex::new(Tetromino::build_tetromino()));
	let tetromino2 = Arc::new(Mutex::new(Tetromino::build_tetromino()));

	let (tx, rx) = mpsc::channel::<String>();


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

	let mut quit = false;

	while tetromino1.lock().unwrap().get_field().is_full() == false 
		&& tetromino2.lock().unwrap().get_field().is_full() == false 
		&& quit == false {
		// Error thrown if receive queue empty
		let received = rx.try_recv();

		// Handling error case
		if received.is_ok() {
			match received.unwrap().as_str() {
				"z" => tetromino1.lock().unwrap().straight_down_blocking(),
				"q" => tetromino1.lock().unwrap().left(),
				"s" => tetromino1.lock().unwrap().down(),
				"d" => tetromino1.lock().unwrap().right(),
				"a" => tetromino1.lock().unwrap().left_rot(),
				"e" => tetromino1.lock().unwrap().right_rot(),
				"r" => tetromino1.lock().unwrap().switch(),
				"E" => quit = true,
				_   => println!(""),
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
		
		// for c in stdin().keys() {
		// 	match c.unwrap() {
		// 		Key::Char('z')	=> tetromino1.lock().unwrap().straight_down_blocking(),
		// 		Key::Char('q')	=> tetromino1.lock().unwrap().left(),
		// 		Key::Char('s')	=> tetromino1.lock().unwrap().down(),
		// 		Key::Char('d')	=> tetromino1.lock().unwrap().right(),
		// 		Key::Char('a')	=> tetromino1.lock().unwrap().left_rot(),
		// 		Key::Char('e')	=> tetromino1.lock().unwrap().right_rot(),
		// 		Key::Char('r')	=> tetromino1.lock().unwrap().switch(),

		// 		Key::Esc		=> quit = true,
		// 		_				=> println!(""),
		// 	}
		// 	break;
		// }
	}
	drop(guard1);
}

pub fn multi_player_online_join() {
	println!("Starting client !\n");
	let (tx, rx) = mpsc::channel::<String>();

	start(&tx);

    // DELETE THIS GARBAGE !
    loop {}
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