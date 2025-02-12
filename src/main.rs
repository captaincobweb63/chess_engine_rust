

// DEBUG

use std::env;


// ENDDEBUG
use uses::constants::{*};
use uses::io::{headless_format, headless_read};
use uses::{debug_tools::{self, time_it}, find_moves, importer, search::search_control};


mod uses;


fn main() {

    let args: Vec<String> = env::args().collect();

    let mut verbose = false;
    let mut headless = false;
    let mut hashing = false;

    let mut depth = 6;
    let mut weight = 0f32;
    let mut to_move = true;
    let mut threads = 8;
    let mut threshold = 10.0;
    let mut boardstring = "".to_string();

    println!("{}", headless_read(importer::read_board_csv(&"/home/billdix/Documents/chess_stuff/board.csv").expect("Error importing")));

    for arg in args
    {
        if arg.contains("-v") {verbose = true;}
        else if arg.contains("-hash"){hashing = true;}
        else if arg.contains("depth") 
        {
            let (_, value) = arg.split_once('=').expect("no argument given");
            depth = value.parse().unwrap();
        }
        else if arg.contains("-h")
        {
            headless = true;
        }
        else if arg.contains("weight")
        {
            let (_, value) = arg.split_once('=').expect("no argument given");
            weight = value.parse().unwrap();
        }
        else if arg.contains("to_move") 
        {
            let (_, value) = arg.split_once('=').expect("no argument given");
            to_move = value.to_ascii_lowercase() == "white";
        }
        else if arg.contains("threads") 
        {
            let (_, value) = arg.split_once('=').expect("no argument given");
            threads = value.parse().unwrap();
        }
        else if arg.contains("threshold") 
        {
            let (_, value) = arg.split_once('=').expect("no argument given");
            threshold = value.parse().unwrap();
        }
        else if arg.contains("core_threads") 
        {
            let (_, value) = arg.split_once('=').expect("no argument given");
            threads = value.parse().unwrap();

            let cores = num_cpus::get();

            threads = threads * cores as i32;
        }

        else if arg.contains("board=") 
        {
            let (_, value) = arg.split_once('=').expect("no argument given");
            boardstring = value.to_string();
        }
    }


    if headless && boardstring.len() != 64 {panic!("No Position Given!\\A Stringified 64-char board must be parsed when running with '-h'");}

    if hashing {debug_tools::make_zhashes("zhash.csv").expect("error in gen");}


    let path: &str = "/home/billdix/Documents/chess_stuff/board.csv";

    let board: Board = if headless {headless_format(boardstring)}else{importer::read_board_csv(path).expect("Error importing")};

    

    if !headless{println!("{}\n",debug_tools::format_board(board));}

    find_moves::find_moves(&board, true, verbose);

    
    // DEBUG STUFF

    env::set_var("RUST_BACKTRACE", "1");
    




    let best_move = if headless{search_control(board,depth,weight,threshold,to_move,threads,verbose)} 
                                                else{time_it(||search_control(board, depth, weight, threshold, to_move, threads, verbose))};

    if best_move.is_none()
    {
        panic!("No valid moves made");
    }

    let valid_best = best_move.unwrap();

    let o_board = if headless{headless_read(valid_best.0)}else{debug_tools::format_board(valid_best.0)};
    if !headless{print!("\n{}\n{}\n", o_board, valid_best.1);}




}

