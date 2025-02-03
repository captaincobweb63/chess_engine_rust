

// DEBUG

use std::env;

#[allow(dead_code)]

// ENDDEBUG
use uses::constants::{*};
use uses::{debug_tools::{self, time_it}, find_moves, importer, search::search_control};


mod uses;


fn main() {

    let args: Vec<String> = env::args().collect();

    let verbose = args.last().is_some_and(|a| a == "-v");


    debug_tools::make_zhashes("zhash.csv").expect("error in gen");


    let path: &str = "/home/billdix/Documents/chess_stuff/board.csv";

    let board: Board = importer::read_board_csv(path).expect("Error importing");

println!("{}\n",debug_tools::format_board(board));

    find_moves::find_moves(&board, true, verbose);

    
    // DEBUG STUFF

    env::set_var("RUST_BACKTRACE", "1");
    

    let depth = 6;
    let weight = 0f32;
    let to_move = true;
    let threads = 8;
    let threshold = 10.0;


    let best_move = time_it(||search_control(board, depth, weight, threshold, to_move, threads, verbose));

    let o_board = debug_tools::format_board(best_move.0);
    print!("\n{}\n{}\n", o_board, best_move.1);




}

