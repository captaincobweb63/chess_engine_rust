
use std::cmp::min;

use crate::uses::{constants::{*},evaluate_pawn_structure::{*},find_moves::find_moves};

pub fn evaluate_board(boardstate: &Board, verbose: bool) -> f32
{
    let mut score: f32 = 0.;
    score += (0.1)*(find_moves(boardstate, true, verbose).len() as f32 - find_moves(boardstate, false, verbose).len() as f32);
    let width: u32 = boardstate[0].len() as u32;
    let height: u32 = boardstate.len() as u32;

    for x in 0..width  
    {
        for y in 0..height
        {
            let code: u32 = boardstate[y as usize][x as usize];
            if code > 0
            {
                let mut pos: [u32; 2];
                let color: bool;

                color = code < BPAWN;
                pos = [y, x];


                // normalise board position so like black and white talking about depth as opposite sides

                if !color
                {
                    pos=[7-pos[0],7-pos[1]];
                }
                

            

                score += evaluate_one_piece(color,((code-1)%6)+1, pos);
            }
        }   
    }

    score += (evaluate_pawn_structure(boardstate))*2f32;

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
        WPAWN    => score_pawn_pos(pos)*1.0,    // Pawn
        WKNIGHT  => score_knight_pos(pos)*1.0,  // Knight
        WBISHOP  => score_bishop_pos(pos)*1.0,  // Bishop
        WROOK    => score_rook_pos(pos)*1.0,    // Rook
        WQUEEN   => score_queen_pos(pos)*1.0,   // Queen
        WKING    => score_king_pos(pos)*2.0,    // King
        _        => 0.0,// Empty so no score ig
    }

    

}

fn score_king_pos(position: [u32;2]) -> f32{

    let rank: u32 = position[0]+1;
    let file: u32 = position[1]+1;

    //Corner
    if rank == 1  && (file == 1 || file == 8)
    {
        return 0.5
    }
    //Too deep 
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