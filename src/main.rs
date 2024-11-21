

// DEBUG

use std::env;

#[allow(dead_code)]

// ENDDEBUG
use uses::{constants, importer};
mod evaluate;

mod uses;


fn main() {





    let path: &str = "/home/billdix/Documents/chess_stuff/board.csv";

    let import = importer::read_csv(path).expect("could not load.");
    for row in import{for piece in row{println!("{}",piece);}}

    let mut board: [[u32; 8]; 8] = [[0; 8]; 8];

    for i in 0..7 {
        board[i].fill(0);
    }
    
    // DEBUG STUFF

    env::set_var("RUST_BACKTRACE", "1");
    
    board[0][1] = constants::BPAWN;
    board[1][1] = constants::BROOK;

    board[5][2] = constants::WQUEEN;
    board[2][2] = constants::WPAWN;



    //update::update_board(&mut board);


    print!("\n{}\n", evaluate::evaluate_board(&import));
}

