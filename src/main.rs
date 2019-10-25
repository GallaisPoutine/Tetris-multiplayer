// IMPORTS

use std::io;
//use game_engine::*;
use termion::{clear, cursor};

mod tetromino;
mod field;
mod game_engine;
mod graphics;
mod reserve;
mod server;
mod client;
mod connection;

// FUNCTIONS

fn main() {
	main_menu();
	//game_engine::single_player();
	//game_engine::multi_player_local();
	//game_engine::timer_test();
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
	if choice == 1 {
		singleplayer_menu();
	} else if choice == 2 {
		multiplayer_menu();
	} else {

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
	if choice == 1 {
		game_engine::single_player();
	} else if choice == 2 {
		main_menu();
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
	if choice == 1 {
		game_engine::multi_player_local();
	} else if choice == 2 {
		//START HOSTED GAME
		game_engine::multi_player_online_host();
	} else if choice == 3 {
		// JOIN GAME HOSTED BY ANOTHER PLAYER
		game_engine::multi_player_online_join();
	} else if choice == 4 {
		main_menu();
	} else {
		
	}
}