// IMPORTS

use std::io;
//use game_engine::*;
use termion::{clear, cursor};

use anyhow;

mod client;
mod connection;
mod field;
mod game_engine;
mod graphics;
mod reserve;
mod server;
mod tetromino;

// FUNCTIONS

fn main() -> anyhow::Result<()> {
    return main_menu();
}

fn main_menu() -> anyhow::Result<()> {
    let mut choice: u32 = 0;
    while choice > 3 || choice < 1 {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        println!("TETRIS");
        println!("1 - SINGLEPLAYER");
        println!("2 - MULTIPLAYER");
        println!("3 - QUIT");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        choice = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }

    match choice {
        1 => return singleplayer_menu(),
        2 => return multiplayer_menu(),
        _ => return Ok(println!("exiting...")),
    }
}

fn singleplayer_menu() -> anyhow::Result<()> {
    let mut choice: u32 = 0;
    while choice > 2 || choice < 1 {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        println!("SINGLEPLAYER");
        println!("1 - PLAY");
        println!("2 - BACK");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)?;
        choice = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
    match choice {
        1 => game_engine::single_player(),
        _ => main_menu()?,
    }

    Ok(())
}

fn multiplayer_menu() -> anyhow::Result<()> {
    let mut choice: u32 = 0;
    while choice > 4 || choice < 1 {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        println!("MULTIPLAYER");
        println!("1 - LOCAL");
        println!("2 - HOST ONLINE");
        println!("3 - JOIN ONLINE");
        println!("4 - BACK");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        choice = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
    match choice {
        1 => game_engine::multi_player_local(),
        2 => game_engine::multi_player_online_host(),
        3 => game_engine::multi_player_online_join(),
        _ => main_menu()?,
    }

    Ok(())
}
