

// DEBUG

use uses::constants;

#[allow(dead_code)]

// ENDDEBUG

mod evaluate;

mod uses;


fn main() {



    let mut board: [[u32; 8]; 8] = [[0; 8]; 8];

    for i in 0..7 {
        board[i].fill(0);
    }
    
    
    board[0][1] = constants::BPAWN;
    board[1][1] = constants::BROOK;

    board[5][2] = constants::WQUEEN;
    board[2][2] = constants::WPAWN;



    //update::update_board(&mut board);

    print!("\n{}\n", evaluate::evaluate_board(&board));
}

