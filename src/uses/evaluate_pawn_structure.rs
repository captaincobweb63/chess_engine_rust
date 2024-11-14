use std::cmp::{max,min};
mod constants;

pub fn evaluate_pawn_structure(boardstate: &[[u32; 8]; 8]) -> f32
{
    let board: [[u32; 8]; 8] = parse(boardstate);
    let mut score = 0f32;
    for rank in board{for file in rank
    {
        let color = board[rank][file];
        if color > 0{
            score += evaluate_pawn(board, row, col, color)
        }
    }}

}

fn parse(boardstate: &[[u32; 8]; 8]) -> [[u32; 8]; 8]
{
    for i in 0..7{
        for j in 0..7{
            if board[i][j] == constants::WPAWN
            {
                board[i][j] = 1;
            }
            else if board[i][j] == constants::BPAWN
            {
                board[i][j] = 2;
            }
            else
            {
                board[i][j] = 0;
            }
        }
    }
}

fn evaluate_pawn(boardstate: &[[u32; 8]; 8], row: u32, col:u32, color: u32) -> f32
{
    let mut score = 0f;

    if is_isolated(board, row, col, color){score -= 0.5}
    if is_doubled(board, row, col, color){score -= 0.5}
    if is_backward(board, row, col, color){score -= 0.5}
    if is_passed(board, row, col, color){score += 0.5}
    if is_connected(board, row, col, color){score += 0.5}
    
    score
}



fn is_isolated(board: &[[u32; 8]; 8], row: u32, col:u32, color: u32)-> bool
{
    //Check adjacent files for pawns
    
    let left_file = col - 1;
    let right_file = col + 1;

    for file in [right_file,left_file]
    {
        if 0<=file<=7
        {
            if board.iter().any(|&rank| rank[file] == color)
            {
                false
            }
        }
    }

}

fn is_doubled(board: &[[u32; 8]; 8], row: u32, col:u32, color: u32)-> bool
{
    //Check if theres another pawn of the same color in this file

    0..7.iter.any(&| rank| board[rank][col] == color && rank !=  row)
}

fn is_backward(board: &[[u32; 8]; 8], row: u32, col:u32, color: u32)-> bool
{
    // Has no support and is alone in its file

    let mut direction: i32 = 0;
    if color == 1{direction=-1}else{direction=1}

    for r in ((direction)+row)..(4-(4*direction))
    {
        if board[r][col] == color{false}
    }
    true
}

fn is_passed(board: &[[u32; 8]; 8], row: u32, col:u32, color: u32)-> bool
{
    // Passed pawn has no opposing pawn in its path

    o_color = 1+(color%2); 
    let mut direction: i32 = 0;
    if color == 1{direction=-1}else{direction=1}

    for r in ((direction)+row)..(4-(4*direction))
    {
        if max(0,col-1)..min(8,col+2).iter.any(&| rank| board[rank][col] == o_color){false}
    }
    true
}

fn is_connected(board: &[[u32; 8]; 8], row: u32, col:u32, color: u32)-> bool
{
    // Connected pawn is any friendly in the same or neighbouring row

    adjacent_files = [col -1, col +1];
    for file in adjacent_files
    {
        if 0<=file && file <8
        {
            if max(0,col-1)..min(8,col+2).iter.any(&| rank| board[rank][col] == o_color){true}
        }
    }
    false
}