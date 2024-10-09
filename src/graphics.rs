// IMPORTS

use termion::{clear, cursor, terminal_size};

use crate::field::*;
use crate::tetromino::*;

// CONSTANTS

const WHITE_BACKGROUND: &str = "\x1b[107m";
const CYAN_BACKGROUND: &str = "\x1b[46m";
const YELLOW_BACKGROUND: &str = "\x1b[103m";
const GREEN_BACKGROUND: &str = "\x1b[42m";
const RED_BACKGROUND: &str = "\x1b[41m";
const ORANGE_BACKGROUND: &str = "\x1b[43m";
const BLUE_BACKGROUND: &str = "\x1b[44m";
const MAJENTA_BACKGROUND: &str = "\x1b[45m";
const GRAY_BACKGROUND: &str = "\x1b[100m";
const BRIGHT_RED_BACKGROUND: &str = "\x1b[101m";

const RESET: &str = "\x1b[0m";
const DOUBLE_SPACE: &str = "  ";

const SINGLE_PLAYER_GRID_MIDDLE: usize = (DEPTH + 2) * 2;
const MULTI_PLAYER_GRID_MIDDLE: usize = (DEPTH + 2) * 4;
const SINGLE_PLAYER_RESERVE_MIDDLE: usize = (TETROMINO_LENGTH + 2) * 2;
const MULTI_PLAYER_RESERVE_MIDDLE: usize = (TETROMINO_LENGTH + 2) * 4;
const SINGLE_PLAYER_NEXT_MIDDLE: usize =
    ((TETROMINO_LENGTH + 2) * LENGTH_OF_NEXT_LIST + (LENGTH_OF_NEXT_LIST - 1)) * 2;
const MULTI_PLAYER_NEXT_MIDDLE: usize =
    ((TETROMINO_LENGTH + 2) * LENGTH_OF_NEXT_LIST + (LENGTH_OF_NEXT_LIST - 1)) * 4;

const SHOWN_DEPTH: usize = DEPTH + 2;
const SHOWN_TETROMINO_LENGTH: usize = TETROMINO_LENGTH + 2;

// PUBLIC FUNCTIONS

pub fn show_single_player_game(tetromino: &mut Tetromino) {
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    let (width, _height) = terminal_size().unwrap();
    show_single_player_grid(tetromino, width);
    show_single_player_next_list(tetromino, width);
    show_single_player_reserve(tetromino, width);
}

pub fn show_multi_player_game(tetromino1: &mut Tetromino, tetromino2: &mut Tetromino) {
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    let (width, _height) = terminal_size().unwrap();
    show_multi_player_grid(tetromino1, tetromino2, width);
    show_multi_player_next_list(tetromino1, tetromino2, width);
    show_multi_player_reserve(tetromino1, tetromino2, width);
}

// PRIVATE FUNCTIONS

fn select_color(color: u8) {
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

fn show_single_player_grid(tetromino: &mut Tetromino, width: u16) {
    print_space((width as usize - SINGLE_PLAYER_GRID_MIDDLE) / 2);
    print_wall(SHOWN_DEPTH);
    println!("\r");
    for i in 0..LENGTH {
        print_space((width as usize - SINGLE_PLAYER_GRID_MIDDLE) / 2);
        print_grid_line(i, tetromino);
        println!("\r");
    }
    print_space((width as usize - SINGLE_PLAYER_GRID_MIDDLE) / 2);
    print_wall(SHOWN_DEPTH);
    println!("\r\n");
}

fn show_multi_player_grid(tetromino1: &mut Tetromino, tetromino2: &mut Tetromino, width: u16) {
    print_space((width as usize - MULTI_PLAYER_GRID_MIDDLE) / 4);
    print_wall(SHOWN_DEPTH);
    print_space((width as usize - MULTI_PLAYER_GRID_MIDDLE) / 2);
    print_wall(SHOWN_DEPTH);
    println!("\r");
    for i in 0..LENGTH {
        print_space((width as usize - MULTI_PLAYER_GRID_MIDDLE) / 4);
        print_grid_line(i, tetromino1);
        print_space((width as usize - MULTI_PLAYER_GRID_MIDDLE) / 2);
        print_grid_line(i, tetromino2);
        println!("\r");
    }
    print_space((width as usize - MULTI_PLAYER_GRID_MIDDLE) / 4);
    print_wall(SHOWN_DEPTH);
    print_space((width as usize - MULTI_PLAYER_GRID_MIDDLE) / 2);
    print_wall(SHOWN_DEPTH);
    println!("\r\n");
}

fn show_single_player_reserve(tetromino: &mut Tetromino, width: u16) {
    print_space((width as usize - SINGLE_PLAYER_RESERVE_MIDDLE) / 2);
    print_wall(SHOWN_TETROMINO_LENGTH);
    println!("\r");
    for i in 0..TETROMINO_LENGTH {
        print_space((width as usize - SINGLE_PLAYER_RESERVE_MIDDLE) / 2);
        print_reserve_line(i, tetromino);
        println!("\r");
    }
    print_space((width as usize - SINGLE_PLAYER_RESERVE_MIDDLE) / 2);
    print_wall(SHOWN_TETROMINO_LENGTH);
    println!("\r\n");
}

fn show_multi_player_reserve(tetromino1: &mut Tetromino, tetromino2: &mut Tetromino, width: u16) {
    print_space((width as usize - MULTI_PLAYER_RESERVE_MIDDLE) / 4);
    print_wall(SHOWN_TETROMINO_LENGTH);
    print_space((width as usize - MULTI_PLAYER_RESERVE_MIDDLE) / 2);
    print_wall(SHOWN_TETROMINO_LENGTH);
    println!("\r");
    for i in 0..TETROMINO_LENGTH {
        print_space((width as usize - MULTI_PLAYER_RESERVE_MIDDLE) / 4);
        print_reserve_line(i, tetromino1);
        print_space((width as usize - MULTI_PLAYER_RESERVE_MIDDLE) / 2);
        print_reserve_line(i, tetromino2);
        println!("\r");
    }
    print_space((width as usize - MULTI_PLAYER_RESERVE_MIDDLE) / 4);
    print_wall(SHOWN_TETROMINO_LENGTH);
    print_space((width as usize - MULTI_PLAYER_RESERVE_MIDDLE) / 2);
    print_wall(SHOWN_TETROMINO_LENGTH);
    println!("\r\n");
}

fn show_single_player_next_list(tetromino: &mut Tetromino, width: u16) {
    print_space((width as usize - SINGLE_PLAYER_NEXT_MIDDLE) / 2);
    print_next_list_wall();
    println!("\r");
    for i in 0..TETROMINO_LENGTH {
        print_space((width as usize - SINGLE_PLAYER_NEXT_MIDDLE) / 2);
        print_next_list_line(i, tetromino);
        println!("\r");
    }
    print_space((width as usize - SINGLE_PLAYER_NEXT_MIDDLE) / 2);
    print_next_list_wall();
    println!("\r\n");
}

fn show_multi_player_next_list(tetromino1: &mut Tetromino, tetromino2: &mut Tetromino, width: u16) {
    print_space((width as usize - MULTI_PLAYER_NEXT_MIDDLE) / 4);
    print_next_list_wall();
    print_space((width as usize - MULTI_PLAYER_NEXT_MIDDLE) / 2);
    print_next_list_wall();
    println!("\r");
    for i in 0..TETROMINO_LENGTH {
        print_space((width as usize - MULTI_PLAYER_NEXT_MIDDLE) / 4);
        print_next_list_line(i, tetromino1);
        print_space((width as usize - MULTI_PLAYER_NEXT_MIDDLE) / 2);
        print_next_list_line(i, tetromino2);
        println!("\r");
    }
    print_space((width as usize - MULTI_PLAYER_NEXT_MIDDLE) / 4);
    print_next_list_wall();
    print_space((width as usize - MULTI_PLAYER_NEXT_MIDDLE) / 2);
    print_next_list_wall();
    println!("\r\n");
}

fn print_space(x: usize) {
    for _i in 0..x {
        print!(" ");
    }
}

fn print_wall(x: usize) {
    for _i in 0..x {
        print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
    }
}

fn print_next_list_wall() {
    for i in 0..LENGTH_OF_NEXT_LIST {
        print_wall(SHOWN_TETROMINO_LENGTH);
        if i != LENGTH_OF_NEXT_LIST - 1 {
            print!("{}", DOUBLE_SPACE);
        }
    }
}

fn print_next_list_line(x: usize, tetromino: &mut Tetromino) {
    for z in 0..LENGTH_OF_NEXT_LIST {
        print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
        for j in 0..TETROMINO_LENGTH {
            select_color(tetromino.get_next_list()[z][x][j]);
            print!("{}{}", DOUBLE_SPACE, RESET);
        }
        if z != LENGTH_OF_NEXT_LIST - 1 {
            print!(
                "{}{}{}{}",
                WHITE_BACKGROUND, DOUBLE_SPACE, RESET, DOUBLE_SPACE
            );
        } else {
            print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
        }
    }
}

fn print_reserve_line(x: usize, tetromino: &mut Tetromino) {
    for j in 0..TETROMINO_LENGTH {
        if j == 0 {
            print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
        }
        select_color(tetromino.get_reserve_form()[x][j]);
        print!("{}{}", DOUBLE_SPACE, RESET);
        if j == TETROMINO_LENGTH - 1 {
            print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
        }
    }
}

fn print_grid_line(x: usize, tetromino: &mut Tetromino) {
    for j in 0..DEPTH {
        if j == 0 {
            print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
        }
        if x < 4 {
            print!("{}", BRIGHT_RED_BACKGROUND);
        }
        let tcolor = tetromino.is_here(x, j);
        let color = if tcolor == 0 {
            tetromino.get_field().get_grid()[x][j]
        } else {
            tcolor
        };
        select_color(color);
        print!("{}{}", DOUBLE_SPACE, RESET);
        if j == DEPTH - 1 {
            print!("{}{}{}", WHITE_BACKGROUND, DOUBLE_SPACE, RESET);
        }
    }
}
