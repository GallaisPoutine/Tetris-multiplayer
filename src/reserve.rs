// IMPORTS

use crate::tetromino::{TETROMINO_LENGTH, ZERO};

// CONSTANTS

// STRUCTURE DEFINITION

pub struct Reserve {
	form : [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH],
	switched : bool,
}

impl Reserve {

	// CONSTRUCTOR

	pub fn build_reserve() -> Reserve{
		Reserve {
			form : ZERO,
			switched : false,
		}
	}

	// GETTERS

	pub fn get_form(&self) -> [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH] {
		self.form
	}

	pub fn is_switched(&self) -> bool {
		self.switched
	}

	// SETTERS

	pub fn set_form(&mut self, form : [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH]) {
		self.form = form;
	}

	pub fn set_switched(&mut self, switched : bool){
		self.switched = switched;
	}

	// PUBLIC FUNCTIONS

	pub fn is_empty(&self) -> bool{
		for i in 0..TETROMINO_LENGTH {
			for j in 0..TETROMINO_LENGTH {
				if self.form[i][j] != 0 {
					return false
				}
			}
		}
		true
	}

	// PRIVATE FUNCTIONS

}

