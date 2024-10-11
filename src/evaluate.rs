
use std:: cmp::min;

use crate::uses::constants::{*};

pub fn evaluate_board(boardstate: &[[u32; 8]; 8]) -> f32
{
    let mut score: f32 = 0.;
    let width: u32 = boardstate[0].len() as u32;
    let height: u32 = boardstate.len() as u32;

    for row_i in 0..height  
    {
        for col_i in 0..width
        {
            let code: u32;
            let pos: [u32; 2];
            let color: bool;
            code = boardstate[row_i as usize][col_i as usize];
            color = code <= 6;
            pos = [row_i, col_i];

            score += evaluate_one_piece(color, code%7 , pos);
        }   
    }
    return score;

}


fn evaluate_one_piece(color: bool, code: u32, pos:[u32;2]) -> f32
{
    let mut score: f32 = SCORETABLE[code as usize];
    score += score_piece_position(code,pos);

    if !color
    {
        score = -score;
    }
    return score;
}

fn score_piece_position(code: u32,pos:[u32;2]) -> f32 
{
    

    // codes from the obsidian table - check it yeah
    match code {
        1 => score_pawn_pos(pos),    // Pawn
        3 => score_knight_pos(pos),  // Knight
        4 => score_bishop_pos(pos),  // Bishop
        5 => score_rook_pos(pos),    // Rook
        6 => score_queen_pos(pos),   // Queen
        7 => score_king_pos(pos),    // King
        _ => 0.0,  // Empty so no score ig
    }

    

}

fn score_king_pos(position: [u32;2]) -> f32{

    let rank: u32 = position[0]+1;
    let file: u32 = position[1]+1;

    //Corner
    if rank == 1  && (file == 1 && file == 8)
    {
        return 0.5
    }
    //Too deep ;)
    if rank >=6
    {
        -0.2
    }
    else{0.0}
}

fn score_queen_pos(position: [u32;2])->f32{

    let rank: u32 = position[0]+1;
    let file: u32 = position[1]+1;

    //Central
    if (rank == 4 || rank == 5) && (file == 5 || file == 5)
    {
        0.6
    }
    else{0.0}
}

fn score_rook_pos(position: [u32;2]) -> f32{

    let file: u32 = position[1]+1;

    // Open Files
    if file == 1 || file == 8
    {
        0.5
    }
    else{0.0}

}

fn score_bishop_pos(position: [u32;2]) -> f32{

    let rank: u32 = position[0]+1;
    let file: u32 = position[1]+1;
    
    //Long diagonals
    let d_r: f32 = min(rank,8-rank) as f32;
    let d_f: f32 = min(file,8-file) as f32;
    let d: f32 = 0.5*f32::sqrt((d_r * d_r) + (d_f * d_f));
    let d_length_weight: f32 = f32::min(d_r/d_f,d_f/d_r);
    let diaganol_bonus:f32 = (d_length_weight*d)/TWOROOTTWO;

    -0.2 + 0.7*diaganol_bonus
}

fn score_knight_pos(position: [u32;2]) -> f32{

    let rank: u32 = position[0]+1;
    let file: u32 = position[1]+1;
    let pos_num: u32 = 8*rank+file;

    //Center
    if  (rank==4||rank==5)&&(file==4||file==5)
    {
        0.7
    }
    //Somewhat Central
    else if pos_num>=16 && pos_num<=47
    {
        0.3
    }
    //Knights on edges are bad
    else if file== 1 || file == 8 || rank == 1 || rank == 8
    {
        -0.5
    }

    else{-0.3}
}

fn score_pawn_pos(position: [u32;2]) -> f32 {
    let rank: u32 = position[0]+1;
    let file: u32 = position[1]+1;

    //Deep - Queen threat
    if rank >= 5
    {
        0.3
    } 
    //Central
    else if  (rank==4||rank==5)&&(file==4||file==5)
    {
        0.5
    }
    else{0.0}
}