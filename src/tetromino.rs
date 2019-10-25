// IMPORTS

use rand::Rng;

use crate::field::*;
use crate::reserve::*;

// CONSTANTS

pub const TETROMINO_LENGTH : usize = 4;

pub const ZERO : [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH] = [[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]];
pub const CYAN : [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH] = [[0,0,1,0],[0,0,1,0],[0,0,1,0],[0,0,1,0]];
pub const YELLOW : [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH] = [[0,0,0,0],[0,2,2,0],[0,2,2,0],[0,0,0,0]];
pub const GREEN : [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH] = [[0,0,0,0],[0,3,3,0],[3,3,0,0],[0,0,0,0]];
pub const RED : [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH]= [[0,0,0,0],[0,4,4,0],[0,0,4,4],[0,0,0,0]];
pub const ORANGE : [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH] = [[0,0,0,0],[0,5,0,0],[0,5,0,0],[0,5,5,0]];
pub const BLUE : [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH] = [[0,0,0,0],[0,0,6,0],[0,0,6,0],[0,6,6,0]];
pub const MAJENTA : [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH] = [[0,0,0,0],[0,7,0,0],[7,7,7,0],[0,0,0,0]];

pub const LENGTH_OF_NEXT_LIST : usize = 3;

//static mut GO_DOWN_TIME : i64 = 1000;

// STRUCTURE DEFINITION

//pub struct Tetromino<'a> {
pub struct Tetromino {
	x : isize,
	y : isize,
	form : [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH], //matrice 4 * 4
	//field : &'a mut Field,
	field : Field,
	reserve : Reserve,
	next_list : [[[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH]; LENGTH_OF_NEXT_LIST],
}

//impl<'a> Tetromino<'a> {
impl Tetromino {

	// CONSTRUCTOR

	//pub fn build_tetromino(field : &mut Field) -> Tetromino {
	pub fn build_tetromino() -> Tetromino {
		let mut next_list = [ZERO; LENGTH_OF_NEXT_LIST];
		for i in 0..LENGTH_OF_NEXT_LIST {
			next_list[i] = Tetromino::init_form();
		}
		Tetromino {
			x : 0,
			y : (DEPTH / 2 - 2) as isize,
			form : Tetromino::init_form(),
			//field,
			field : Field::build_field(),
			reserve : Reserve::build_reserve(),
			next_list : next_list,
		}
	}

	// GETTERS

	/*pub fn get_x(&self) -> isize {
		self.x
	}*/

	/*pub fn get_y(&self) -> isize {
		self.y
	}*/

	pub fn get_form(&self) -> [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH] {
		self.form
	}

	/*pub fn is_moving(&self) -> bool {
		self.moving
	}*/

	//pub fn get_field(&mut self) -> &mut Field {
	pub fn get_field(&mut self) -> &mut Field {
		&mut (self.field)
	}

	/*pub fn get_reserve(&self) -> Reserve {
		self.reserve
	}*/

	pub fn get_reserve_form(&self) -> [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH] {
		self.reserve.get_form()
	}

	pub fn get_next_list(&self) -> [[[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH]; LENGTH_OF_NEXT_LIST] {
		self.next_list
	}

	// SETTERS

	/*pub fn set_x(&mut self, x : isize) {
		self.x = x;
	}*/

	/*pub fn set_y(&mut self, y : isize) {
		self.y = y;
	}*/

	pub fn set_form(&mut self, form : [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH]) {
		self.form = form;
	}

	/*pub fn set_moving(&mut self, moving : bool){
		self.moving = moving;
	}*/

	// PUBLIC FUNCTIONS

	/*pub fn up(&mut self) {
		println!("UP");
		self.x -= 1;
		if self.collision_detection() == true {
			self.x += 1;
		} else {
			show_grid(self, self.field);
		}
	}*/

	pub fn down(&mut self){
		self.x += 1;
		if self.collision_detection() == true {
			self.x -= 1;
		}
	}

	pub fn left(&mut self){
		self.y -= 1;
		if self.collision_detection() == true {
			self.y += 1;
		}
	}

	pub fn right(&mut self){
		self.y += 1;
		if self.collision_detection() == true {
			self.y -= 1;
		}
	}

	pub fn down_blocking (&mut self){
		self.x += 1;
		if self.collision_detection() == true {
			self.x -= 1;
			self.put_in_grid();
			self.change();
			//self.moving = false;
		}
	}

	pub fn straight_down_blocking (&mut self){
		while self.collision_detection() == false {
			self.x += 1;
		}
		self.x -= 1;
		self.put_in_grid();
		self.change();
		//self.moving = false;
	}

	pub fn left_rot(&mut self){
		let mut tmp = [[0;TETROMINO_LENGTH]; TETROMINO_LENGTH];
		for i in 0..TETROMINO_LENGTH {
			for j in 0..TETROMINO_LENGTH {
				tmp[i][j] = self.form[j][TETROMINO_LENGTH - 1 - i];
			}
		}
		let form_save = self.form;
		self.form = tmp;
		if self.collision_detection() == true {
			self.form = form_save;
		}
	}

	pub fn right_rot(&mut self){
		let mut tmp = [[0;TETROMINO_LENGTH]; TETROMINO_LENGTH];
		for i in 0..TETROMINO_LENGTH {
			for j in 0..TETROMINO_LENGTH {
				tmp[i][j] = self.form[TETROMINO_LENGTH - 1 - j][i];
			}
		}
		let form_save = self.form;
		self.form = tmp;
		if self.collision_detection() == true {
			self.form = form_save;
		}
	}

	pub fn is_here(&self, x : usize, y : usize) -> u8 {
		let i = x as isize - self.x;
		let j = y as isize - self.y;
		if i < 0 || i >= TETROMINO_LENGTH as isize || j < 0 || j >= TETROMINO_LENGTH as isize {
			0
		} else {
			self.form[i as usize][j as usize]
		}
	}

	pub fn init_form() -> [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH] {
		let random = rand::thread_rng().gen_range(1, 8);
		let form : [[u8; TETROMINO_LENGTH]; TETROMINO_LENGTH];
		match random {
			1 => form = CYAN,
			2 => form = YELLOW,
			3 => form = GREEN,
			4 => form = RED,
			5 => form = ORANGE,
			6 => form = BLUE,
			7 => form = MAJENTA,
			_ => form = ZERO
		}
		form
	}

	pub fn change(&mut self) {
		self.reserve.set_switched(false);
		self.field.count_complete_lines(self.x as usize);
		if self.field.is_in_losszone() == true {
			// STOP TIMER
		} else {
			self.next();
		}
	}

	pub fn switch(&mut self){
		if self.reserve.is_switched() == false {
			self.reserve.set_switched(true);
			if self.reserve.is_empty() == true {
				self.reserve.set_form(self.form);
				self.next();
			} else {
				let tmp = self.get_form();
				self.set_form(self.reserve.get_form());
				self.reserve.set_form(tmp);
				self.x = 0;
				self.y = (DEPTH/2 -2) as isize;
			}
		}
	}

	pub fn next(&mut self){
		self.x = 0;
		self.y = (DEPTH/2 -2) as isize;
		self.set_form(self.next_list[0]);
		//self.set_moving(true);
		for i in 0..LENGTH_OF_NEXT_LIST -1 {
			self.next_list[i] = self.next_list[i+1];
		}
		self.next_list[LENGTH_OF_NEXT_LIST - 1] = Tetromino::init_form();
	}

	// PRIVATE FUNCTIONS

	fn collision_detection(&self) -> bool{
		for i in 0..TETROMINO_LENGTH {
			for j in 0..TETROMINO_LENGTH {
				if self.form[i][j] != 0 && (self.x + (i as isize) < 0 || self.y + (j as isize) < 0 || 
					self.x + (i as isize) >= (LENGTH as isize) || self.y + (j as isize) >= (DEPTH as isize)) {
					return true
				} else if self.form[i][j] != 0 && 
				self.field.get_grid()[(i as isize + self.x) as usize][(j as isize + self.y) as usize] != 0 {
					return true
				}
			}
		}
		false
	}

	/*fn collision_detection2(&self) -> bool{
		let grid = self.field.get_grid();
		let skip : usize = self.x.min(0) as usize;
		let nbpre : usize = if self.x < 0 {-self.x as usize} else {0};
		//let nbpreup : usize = if self.y < 0 {-self.y as usize} else {0};
		for (i, (lg, lt)) in	(0 .. nbpre).map(|_|
									&[1; DEPTH]		// inserting a wall
								).chain(
									grid.iter().skip(
										self.x as usize
									)
								).zip(
									self.form.iter()
								).enumerate() {
			//for (j, (&cg, &ct)) in (0..nbpreup).map(|_| &[1; LENGTH]).chain(lg.iter()
			.skip(self.y as usize)).zip(lt.iter()).enumerate() {
			for (j, (&cg, &ct)) in lg.iter().skip(self.y as usize).zip(lt.iter()).enumerate() {
				println!("{}, {} = {}", i, j, cg);
				if cg != 0 && ct != 0 {
					return true;
				}
			}
		}
		false
	}*/

	fn put_in_grid(&mut self){
		let mut grid = self.field.get_grid();
		for i in 0..TETROMINO_LENGTH {
			for j in 0..TETROMINO_LENGTH {
				if self.form[i][j] != 0 {
					grid[(i as isize + self.x) as usize][(j as isize + self.y) as usize] = self.form[i][j];
				}
			}
		}
		self.field.set_grid(grid);
	}

	/*fn show(&self){
		println!("COORD X : {}, Y : {}", self.x, self.y);
		for i in 0..TETROMINO_LENGTH {
			for j in 0..TETROMINO_LENGTH {
				print!("{}", self.form[i][j]);
			}
			println!();
		}
	}*/
}

