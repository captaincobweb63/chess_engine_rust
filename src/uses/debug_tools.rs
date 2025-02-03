use std::{fs::File, time::Instant};

use rand::Rng;
use std::io::{self, Write};

use super::constants::*;


pub fn format_board(board: Board)->String
{
    let mut out_board: String = "\n".to_string();
    for i in 0..8
    {
        for j in 0..8
        {
            let current = board[i][j];
            let icon;
            match current {
                WPAWN       => icon = '♙',
                WKNIGHT     => icon = '♘',
                WROOK       => icon = '♖',
                WBISHOP     => icon = '♗',
                WQUEEN      => icon = '♕',
                WKING       => icon = '♔',
                BPAWN       => icon = '🨩',
                BKNIGHT     => icon = '🨨',
                BROOK       => icon = '🨦',
                BBISHOP     => icon = '🨧',
                BQUEEN      => icon = '🨥',
                BKING       => icon = '🨤',
                _           => icon = '▢'
            }

            out_board.push(icon);
            out_board.push(' ');
        }
        out_board.push('\n');

    }
    return out_board;

}

pub fn make_zhashes(path: &str) -> io::Result<()>
{
    let mut file = File::create(path).expect("Error creating file for hashes");
    let mut rng = rand::rng();

    for _ in 0..(PIECENUM*SQUARES)
    {
        let hash: u64 = rng.random();
        writeln!(file, "{:X}", hash).expect("Error writing to hashes");
    }

    Ok(())
}

pub fn time_it<F, T>(func: F) -> T
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();
    println!("Execution time: {:.6} seconds", duration.as_secs_f64());
    result
}