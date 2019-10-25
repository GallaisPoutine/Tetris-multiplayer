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

// CONSTANTS

// STRUCTURE DEFINITION

// CONSTRUCTOR

// GETTERS

// SETTERS

// PUBLIC FUNCTIONS

/*pub fn single_player2(){

	//let mut field : Field = Field::build_field();
	//let mut tetromino : Tetromino = Tetromino::build_tetromino(&mut field);
	//let mut tetromino = Mutex::new(Tetromino::build_tetromino(&mut field));

	let mut stdout = stdout().into_raw_mode().unwrap();
	let timer = timer::Timer::new();
	let mut tetromino = Arc::new(Mutex::new(Tetromino::build_tetromino()));
	//tetromino = mutex.clone();
	let mut go_down_time : i64 = 1000;

	//show_single_player_game(&mut tetromino);
	show_single_player_game(&mut tetromino.lock().unwrap());

	//start_single_player_timer(&timer, t, go_down_time);
	//let t = tetromino.clone();

	let guard = {
		let t = tetromino.clone();
		timer.schedule_repeating(chrono::Duration::milliseconds(go_down_time), move || {
			t.lock().unwrap().down_blocking();
			show_single_player_game(&mut t.lock().unwrap());
		});
	};

	//thread::sleep(std::time::Duration::new(5, 0));

	// Get the standard input stream.
	//let stdin = stdin();
	// Get the standard output stream and go to raw mode.
	
	//stdout.flush().unwrap();
	//let t = tetromino.clone();

	let mut quit : bool = false;
	while tetromino.lock().unwrap().get_field().is_full() == false && quit == false{

		for c in stdin().keys() {
			match c.unwrap() {
				//Key::Char('z')	=> tetromino.up(),
				/*Key::Char('z')	=> tetromino.straight_down_blocking(),
				Key::Char('q')	=> tetromino.left(),
				Key::Char('s')	=> tetromino.down(),
				Key::Char('d')	=> tetromino.right(),
				Key::Char('a')	=> tetromino.left_rot(),
				Key::Char('e')	=> tetromino.right_rot(),
				Key::Char('r')	=> tetromino.switch(),*/
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
			//show_single_player_game(&tetromino, &tetromino.get_field());
			//show_single_player_game(&mut tetromino);
			show_single_player_game(&mut tetromino.lock().unwrap());
			break;
		}
		// Flush again.
		//stdout.flush().unwrap();
	}

	drop(guard);
	// Show the cursor again before we exit.
	//write!(stdout, "{}", termion::cursor::Show).unwrap();
}*/

/*pub fn multi_player_local2(){
	//let mut field1 : Field = Field::build_field();
	//let mut field2 : Field = Field::build_field();
	//let mut tetromino1 : Tetromino = Tetromino::build_tetromino(&mut field1);
	//let mut tetromino2 : Tetromino = Tetromino::build_tetromino(&mut field2);

	let mut tetromino1 = Arc::new(Mutex::new(Tetromino::build_tetromino()));
	let mut tetromino2 = Arc::new(Mutex::new(Tetromino::build_tetromino()));

	let mut number_of_lines1 : u32 = 0;
	let mut number_of_lines2 : u32 = 0;

	let mut quit : bool = false;

	//show_multi_player_game(&mut tetromino1, &mut tetromino2);
	show_multi_player_game(&mut tetromino1.lock().unwrap(), &mut tetromino2.lock().unwrap());

	let mut stdout = stdout().into_raw_mode().unwrap();

	stdout.flush().unwrap();
	while tetromino1.lock().unwrap().get_field().is_full() == false && tetromino2.lock().unwrap().get_field().is_full() == false && quit == false{
		for c in stdin().keys() {
			match c.unwrap() {
				/*Key::Char('z')	=> tetromino1.straight_down_blocking(),
				Key::Char('q')	=> tetromino1.left(),
				Key::Char('s')	=> tetromino1.down(),
				Key::Char('d')	=> tetromino1.right(),
				Key::Char('a')	=> tetromino1.left_rot(),
				Key::Char('e')	=> tetromino1.right_rot(),
				Key::Char('r')	=> tetromino1.switch(),

				Key::Up			=> tetromino2.straight_down_blocking(),
				Key::Left		=> tetromino2.left(),
				Key::Down		=> tetromino2.down(),
				Key::Right		=> tetromino2.right(),
				Key::Char('i')	=> tetromino2.left_rot(),
				Key::Char('o')	=> tetromino2.right_rot(),
				Key::Char('p')	=> tetromino2.switch(),*/

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
			//show_multi_player_game(&mut tetromino1, &mut tetromino2);
			show_multi_player_game(&mut tetromino1.lock().unwrap(), &mut tetromino2.lock().unwrap());
			break;
		}
		stdout.flush().unwrap();
	}
	// 
}*/

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

	// show_multi_player_game(&mut tetromino1.lock().unwrap(), &mut tetromino2.lock().unwrap());

	// let guard1 = {
	// 	let t1 = tetromino1.clone();
	// 	let t2 = tetromino2.clone();
	// 	timer.schedule_repeating(chrono::Duration::milliseconds(go_down_time), move || {
	// 		t1.lock().unwrap().down_blocking();
	// 		t2.lock().unwrap().down_blocking();
	// 		show_multi_player_game(&mut t1.lock().unwrap(), &mut t2.lock().unwrap());
	// 		// server.write("/D");
	// 	})
	// };

	let mut quit = false;
	let mut i = 0;

	while tetromino1.lock().unwrap().get_field().is_full() == false 
		&& tetromino2.lock().unwrap().get_field().is_full() == false 
		&& quit == false {
			i+= 1;
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
		// 	verify_destroyed_lines(tetromino1.lock().unwrap().get_field(), 
		// 							tetromino2.lock().unwrap().get_field(), 
		// 							&mut number_of_lines1);
		// 	verify_destroyed_lines(tetromino2.lock().unwrap().get_field(), 
		// 							tetromino1.lock().unwrap().get_field(), 
		// 							&mut number_of_lines2);
		// 	show_multi_player_game(&mut tetromino1.lock().unwrap(), 
		// 							&mut tetromino2.lock().unwrap());
		// 	break;
		// }
		println!("OK {}", i);
		match rx.recv().unwrap().as_str() {
			"z" => {
				println!("{}", i);
				tetromino1.lock().unwrap().straight_down_blocking()
				},
			"q" => {
				println!("{}", i);
				tetromino1.lock().unwrap().left()
				},
			"s" => {
				println!("{}", i);
				tetromino1.lock().unwrap().down()
				},
			"d" => {
				println!("{}", i);
				tetromino1.lock().unwrap().right()
				},
			"a" => {
				println!("{}", i);
				tetromino1.lock().unwrap().left_rot()
				},
			"e" => {
				println!("{}", i);
				tetromino1.lock().unwrap().right_rot()
				},
			"r" => {
				println!("{}", i);
				tetromino1.lock().unwrap().switch()
				},
			"E" => {
				println!("{}", i);
				quit = true
				},
			_   => println!(""),
		}
	}
	// drop(guard1);
	// while match rx.recv().unwrap(){
	// 	"z".to_owned().to_string() => tetromino1.lock().unwrap().straight_down_blocking(),
	// 	"q".to_owned().to_string() => tetromino1.lock().unwrap().left(),
	// 	"s".to_owned().to_string() => tetromino1.lock().unwrap().down(),
	// 	"d".to_owned().to_string() => tetromino1.lock().unwrap().right(),
	// 	"a".to_owned().to_string() => tetromino1.lock().unwrap().left_rot(),
	// 	"e".to_owned().to_string() => tetromino1.lock().unwrap().right_rot(),
	// 	"r".to_owned().to_string() => tetromino1.lock().unwrap().switch(),
	// 	"E".to_owned().to_string() => quit = true,
	// 	_   => println!(""),
	// }{};
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