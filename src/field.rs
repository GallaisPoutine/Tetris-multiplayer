// IMPORTS

use rand::Rng;

use crate::tetromino::*;

// CONSTANTS

pub const LENGTH : usize = 24;
pub const DEPTH : usize = 10;

// STRUCTURE DEFINITION

pub struct Field {
	grid : [[u8; DEPTH]; LENGTH],
	full : bool,
	number_of_lines : u32,
}

impl Field {

	// CONSTRUCTOR

	pub fn build_field() -> Field{
		Field {
			grid : [[0; DEPTH]; LENGTH],
			full : false,
			number_of_lines : 0,
		}
	}

	// GETTERS

	pub fn get_grid(&self) -> [[u8; DEPTH]; LENGTH] {
		self.grid
	}

	pub fn is_full(&self) -> bool {
		self.full
	}

	pub fn get_number_of_lines(&self) -> u32 {
		self.number_of_lines
	}

	// SETTERS

	pub fn set_grid(&mut self, grid : [[u8; DEPTH]; LENGTH]) {
		self.grid = grid;
	}

	// PUBLIC FUNCTIONS

	pub fn add_lines(&mut self, number_of_lines : usize){
		let random = rand::thread_rng().gen_range(0, 10);
		for _i in 0..number_of_lines {
			for j in 0..LENGTH - 1{
				self.grid[j] = self.grid[j+1];
			}
			for j in 0..DEPTH {
				if j != random {
					self.grid[LENGTH - 1][j] = 8;
				} else {
					self.grid[LENGTH - 1][j] = 0;
				}
			}
		}
	}

	pub fn is_in_losszone(&mut self) -> bool{
		let mut i : usize = 0;
		while i < 4 && !self.full {
			let mut j : usize = 0;
			while j < DEPTH && !self.full {
				if self.grid[i][j] != 0 {
					self.full = true;
				}
				j += 1;
			}
			i += 1;
		}
		self.full
	}

	pub fn count_complete_lines(&mut self, mut x : usize){
		let mut consecutive_count = 0;
		let mut i : usize = TETROMINO_LENGTH - 1 + x;
		while i >= x{
			let mut line : u8 = 0;
			for j in 0..DEPTH {
				if i < LENGTH && j < DEPTH && self.grid[i][j] != 0 {
					line += 1;
				}
			}
			if line == 10 {
				let mut j : usize = i;
				while j >= 1 {
					self.grid[j] = self.grid[j-1];
					j -= 1;
				}
				x += 1;
				i += 1;
				consecutive_count += 1;
			} else {
				self.number_of_lines += consecutive_count;
				consecutive_count = 0;
			}
			if i > 0 {
				i -= 1;
			} else {
				break;
			}		
		}
		self.number_of_lines += consecutive_count;
	}
}

