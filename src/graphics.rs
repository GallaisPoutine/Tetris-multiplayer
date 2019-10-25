// IMPORTS

//use termion::*;
//use std::io;

use crate::tetromino::*;
use crate::field::*;
//use crate::reserve::*;

use termion::{clear, cursor, terminal_size};

// CONSTANTS

//const BLACK_FOREGROUND : &str = "\x1b[30m";
//const WHITE_FOREGROUND : &str = "\x1b[37m";
//const RED_FOREGROUND : &str = "\x1b[31m";

const WHITE_BACKGROUND : &str = "\x1b[107m";
const CYAN_BACKGROUND : &str = "\x1b[46m";
const YELLOW_BACKGROUND : &str = "\x1b[103m";
const GREEN_BACKGROUND : &str = "\x1b[42m";
const RED_BACKGROUND : &str = "\x1b[41m";
const ORANGE_BACKGROUND : &str = "\x1b[43m";
const BLUE_BACKGROUND : &str = "\x1b[44m";
const MAJENTA_BACKGROUND : &str = "\x1b[45m";
const GRAY_BACKGROUND : &str = "\x1b[100m";

const BRIGHT_RED_BACKGROUND : &str = "\x1b[101m";

const RESET : &str = "\x1b[0m";

const DOUBLE_SPACE : &str = "  ";

// STRUCTURE DEFINITION

// CONSTRUCTOR

// GETTERS

// SETTERS

// PUBLIC FUNCTIONS

pub fn show_single_player_game(tetromino : &mut Tetromino){
	print!("{}{}", clear::All, cursor::Goto(1,1));
	let (width, _height) = terminal_size().unwrap();
	//println!("WIDTH : {}, HEIGHT : {}", x, y);
	show_single_player_grid(tetromino, width);
	show_single_player_next_list(tetromino, width);
	show_single_player_reserve(tetromino, width);
	//show_game_info();
}

 pub fn show_multi_player_game(tetromino1 : &mut Tetromino, tetromino2 : &mut Tetromino){
	print!("{}{}", clear::All, cursor::Goto(1,1));
	let (width, _height) = terminal_size().unwrap();
	show_multi_player_grid(tetromino1, tetromino2, width);
	show_multi_player_next_list(tetromino1, tetromino2, width);
	show_multi_player_reserve(tetromino1, tetromino2, width);
	//show_game_info()
}

// PRIVATE FUNCTIONS

fn select_color (color : u8){
	match color {
		0 => print!(""),
		1 => print!("{}", CYAN_BACKGROUND),
		2 => print!("{}", YELLOW_BACKGROUND),
		3 => print!("{}", GREEN_BACKGROUND),
		4 => print!("{}", RED_BACKGROUND),
		5 => print!("{}", ORANGE_BACKGROUND),
		6 => print!("{}", BLUE_BACKGROUND),
		7 => print!("{}", MAJENTA_BACKGROUND),
		8 => print!("{}", GRAY_BACKGROUND),
		_ => print!("{}", WHITE_BACKGROUND),
	}
}

fn show_single_player_grid(tetromino : &mut Tetromino, width : u16){
	for _i in 0..(width as usize - (DEPTH + 2)*2)/2 {
		print!(" ");
	}
	for _i in 0..DEPTH + 2  {
		print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
	}
	println!("\r");
	for i in 0..LENGTH {
		for _j in 0..(width as usize - (DEPTH + 2)*2)/2 {
			print!(" ");
		}
		for j in 0..DEPTH {
			if j == 0 {
				print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			}
			if i < 4 {
				print!("{}", BRIGHT_RED_BACKGROUND);
			}
			let tcolor = tetromino.is_here(i, j);
			let color = if tcolor == 0 {
				tetromino.get_field().get_grid()[i][j]
			} else {
				tcolor
			};
			select_color(color);
			print!("{}{}", DOUBLE_SPACE, RESET);
			if j == DEPTH -1 {
				print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			}
		}
		println!("\r");
	}
	for _i in 0..(width as usize - (DEPTH + 2)*2)/2 {
		print!(" ");
	}
	for _i in 0..DEPTH + 2 {
		print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
	}
	println!("\r");
	println!("");
}

fn show_multi_player_grid(tetromino1 : &mut Tetromino, tetromino2 : &mut Tetromino, _width : u16){
	for _i in 0..DEPTH + 2  {
		print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
	}
	print!("{}", DOUBLE_SPACE);
	for _i in 0..DEPTH + 2  {
		print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
	}
	println!("\r");
	for i in 0..LENGTH {
		for j in 0..DEPTH {
			if j == 0 {
				print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			}
			if i < 4 {
				print!("{}", BRIGHT_RED_BACKGROUND);
			}
			let tcolor = tetromino1.is_here(i, j);
			let color = if tcolor == 0 {
				tetromino1.get_field().get_grid()[i][j]
			} else {
				tcolor
			};
			select_color(color);
			print!("{}{}", DOUBLE_SPACE, RESET);
			if j == DEPTH -1 {
				print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			}
		}
		print!("{}", DOUBLE_SPACE);
		for j in 0..DEPTH {
			if j == 0 {
				print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			}
			if i < 4 {
				print!("{}", BRIGHT_RED_BACKGROUND);
			}
			let tcolor = tetromino2.is_here(i, j);
			let color = if tcolor == 0 {
				tetromino2.get_field().get_grid()[i][j]
			} else {
				tcolor
			};
			select_color(color);
			print!("{}{}", DOUBLE_SPACE, RESET);
			if j == DEPTH -1 {
				print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			}
		}
		println!("\r");
	}
	for _i in 0..DEPTH + 2 {
		print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
	}
	print!("{}", DOUBLE_SPACE);
	for _i in 0..DEPTH + 2 {
		print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
	}
	println!("\r");
	println!("");
}

fn show_single_player_reserve(tetromino : &mut Tetromino, width : u16) {
	for _i in 0..(width as usize - (TETROMINO_LENGTH + 2)*2)/2 {
		print!(" ");
	}
	for _i in 0..TETROMINO_LENGTH + 2  {
		print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
	}
	println!("\r");
	for i in 0..TETROMINO_LENGTH {
		for _j in 0..(width as usize - (TETROMINO_LENGTH + 2)*2)/2 {
			print!(" ");
		}
		for j in 0..TETROMINO_LENGTH {
			if j == 0 {
				print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			}
			select_color(tetromino.get_reserve_form()[i][j]);
			print!("{}{}", DOUBLE_SPACE, RESET);
			if j == TETROMINO_LENGTH -1 {
				print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			}
		}
		println!("\r");
	}
	for _i in 0..(width as usize - (TETROMINO_LENGTH + 2)*2)/2 {
		print!(" ");
	}
	for _i in 0..TETROMINO_LENGTH + 2  {
		print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
	}
	println!("\r");
	println!("");
}

fn show_multi_player_reserve(tetromino1 : &mut Tetromino, tetromino2 : &mut Tetromino, _width : u16){
	for _i in 0..TETROMINO_LENGTH + 2  {
		print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
	}
	print!("{}", DOUBLE_SPACE);
	for _i in 0..TETROMINO_LENGTH + 2  {
		print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
	}
	println!("\r");
	for i in 0..TETROMINO_LENGTH {
		for j in 0..TETROMINO_LENGTH {
			if j == 0 {
				print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			}
			select_color(tetromino1.get_reserve_form()[i][j]);
			print!("{}{}", DOUBLE_SPACE, RESET);
			if j == TETROMINO_LENGTH -1 {
				print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			}
		}
		print!("{}", DOUBLE_SPACE);
		for j in 0..TETROMINO_LENGTH {
			if j == 0 {
				print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			}
			select_color(tetromino2.get_reserve_form()[i][j]);
			print!("{}{}", DOUBLE_SPACE, RESET);
			if j == TETROMINO_LENGTH -1 {
				print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			}
		}
		println!("\r");
	}
	for _i in 0..TETROMINO_LENGTH + 2  {
		print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
	}
	print!("{}", DOUBLE_SPACE);
	for _i in 0..TETROMINO_LENGTH + 2  {
		print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
	}
	println!("\r");
	println!("");
}

fn show_single_player_next_list(tetromino : &mut Tetromino, width : u16){
	for _i in 0..(width as usize - ((TETROMINO_LENGTH + 2) * LENGTH_OF_NEXT_LIST + (LENGTH_OF_NEXT_LIST - 1))*2)/2 {
		print!(" ");
	}
	for _i in 0..LENGTH_OF_NEXT_LIST {
		print!("{}", WHITE_BACKGROUND);
		for _j in 0..TETROMINO_LENGTH + 2 {
			print!("{}", DOUBLE_SPACE);
		}
		print!("{}{}", RESET, DOUBLE_SPACE);
	}
	println!("\r");
	for i in 0..TETROMINO_LENGTH {
		for _z in 0..(width as usize - ((TETROMINO_LENGTH + 2) * LENGTH_OF_NEXT_LIST + (LENGTH_OF_NEXT_LIST - 1))*2)/2 {
			print!(" ");
		}
		for z in 0..LENGTH_OF_NEXT_LIST {
			print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			for j in 0..TETROMINO_LENGTH {
				select_color(tetromino.get_next_list()[z][i][j]);
				print!("{}{}", DOUBLE_SPACE, RESET);
			}
			print!("{}{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET, DOUBLE_SPACE);
		}
		println!("\r");
	}
	for _i in 0..(width as usize - ((TETROMINO_LENGTH + 2) * LENGTH_OF_NEXT_LIST + (LENGTH_OF_NEXT_LIST - 1))*2)/2 {
		print!(" ");
	}
	for _i in 0..LENGTH_OF_NEXT_LIST {
		print!("{}", WHITE_BACKGROUND);
		for _j in 0..TETROMINO_LENGTH + 2 {
			print!("{}", DOUBLE_SPACE);
		}
		print!("{}{}", RESET, DOUBLE_SPACE);
	}
	println!("\r");
	println!("");
}

fn show_multi_player_next_list(tetromino1 : &mut Tetromino, tetromino2 : &mut Tetromino, _width : u16){
	for _i in 0..LENGTH_OF_NEXT_LIST {
		print!("{}", WHITE_BACKGROUND);
		for _j in 0..TETROMINO_LENGTH + 2 {
			print!("{}", DOUBLE_SPACE);
		}
		print!("{}{}", RESET, DOUBLE_SPACE);
	}
	print!("{}", DOUBLE_SPACE);
	for _i in 0..LENGTH_OF_NEXT_LIST {
		print!("{}", WHITE_BACKGROUND);
		for _j in 0..TETROMINO_LENGTH + 2 {
			print!("{}", DOUBLE_SPACE);
		}
		print!("{}{}", RESET, DOUBLE_SPACE);
	}
	println!("\r");

	for i in 0..TETROMINO_LENGTH {
		for z in 0..LENGTH_OF_NEXT_LIST {
			print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			for j in 0..TETROMINO_LENGTH {
				select_color(tetromino1.get_next_list()[z][i][j]);
				print!("{}{}", DOUBLE_SPACE, RESET);
			}
			print!("{}{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET, DOUBLE_SPACE);
		}
		print!("{}", DOUBLE_SPACE);
		for z in 0..LENGTH_OF_NEXT_LIST {
			print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
			for j in 0..TETROMINO_LENGTH {
				select_color(tetromino2.get_next_list()[z][i][j]);
				print!("{}{}", DOUBLE_SPACE, RESET);
			}
			print!("{}{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET, DOUBLE_SPACE);
		}
		println!("\r");
	}

	for _i in 0..LENGTH_OF_NEXT_LIST {
		print!("{}", WHITE_BACKGROUND);
		for _j in 0..TETROMINO_LENGTH + 2 {
			print!("{}", DOUBLE_SPACE);
		}
		print!("{}{}", RESET, DOUBLE_SPACE);
	}
	print!("{}", DOUBLE_SPACE);
	for _i in 0..LENGTH_OF_NEXT_LIST {
		print!("{}", WHITE_BACKGROUND);
		for _j in 0..TETROMINO_LENGTH + 2 {
			print!("{}", DOUBLE_SPACE);
		}
		print!("{}{}", RESET, DOUBLE_SPACE);
	}
	println!("\r");
	println!("");
}