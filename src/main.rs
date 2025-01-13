

// DEBUG

use std::{collections::HashMap, env, fs::File};

use csv::ReaderBuilder;
#[allow(dead_code)]

// ENDDEBUG
use uses::constants::{*};
use uses::{evaluate, importer};


mod uses;


fn main() {





    let path: &str = "/home/billdix/Documents/chess_stuff/board.csv";

    let b: Board = importer::read_board_csv(path).expect("Error importing");

    for row in b{for piece in row{println!("{}",piece);}}

    let mut board: Board = [[0; 8]; 8];

    for i in 0..7 {
        board[i].fill(0);
    }
    
    // DEBUG STUFF

    env::set_var("RUST_BACKTRACE", "1");
    
    board[0][1] = BPAWN;
    board[1][1] = BROOK;

    board[5][2] = WQUEEN;
    board[2][2] = WPAWN;


    let mut pawnhash: HashMap<u32, f32>;


    //update::update_board(&mut board);


    print!("\n{}\n", evaluate::evaluate_board(&b));


}

