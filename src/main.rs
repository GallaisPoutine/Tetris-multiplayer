mod tetromino;
mod field;
mod game_engine;
mod graphics;
mod reserve;

// IMPORTS

use std::io;
use termion::{clear, cursor};

// FUNCTIONS

fn main() {
	main_menu();
}

fn main_menu() {
	let mut choice : u32 = 0;
	while choice > 3 || choice < 1 {
		print!("{}{}", clear::All, cursor::Goto(1,1));
		println!("TETRIS");
		println!("1 - SINGLEPLAYER");
		println!("2 - MULTIPLAYER");
		println!("3 - QUIT");

		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read line");
		choice = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
	}
	match choice {
		1 => singleplayer_menu(),
		2 => multiplayer_menu(),
		_ => println!("exiting..."),
	}
}

fn singleplayer_menu() {
	let mut choice : u32 = 0;
	while choice > 2 || choice < 1 {
		print!("{}{}", clear::All, cursor::Goto(1,1));
		println!("SINGLEPLAYER");
		println!("1 - PLAY");
		println!("2 - BACK");

		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read line");
		choice = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
	}
	match choice {
		1 => game_engine::single_player(),
		_ => main_menu(),
	}
}

fn multiplayer_menu() {
	let mut choice : u32 = 0;
	while choice > 4 || choice < 1 {
		print!("{}{}", clear::All, cursor::Goto(1,1));
		println!("MULTIPLAYER");
		println!("1 - LOCAL");
		println!("2 - HOST ONLINE");
		println!("3 - JOIN ONLINE");
		println!("4 - BACK");

		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read line");
		choice = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
	}
	match choice {
		1 => game_engine::multi_player_local(),
		2 => game_engine::multi_player_online_host(),
		3 => game_engine::multi_player_online_join(),
		_ => main_menu(),
	}
}