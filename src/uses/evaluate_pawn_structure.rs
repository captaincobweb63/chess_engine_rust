use std::cmp::min;
use super::constants;

pub fn evaluate_pawn_structure(boardstate: &[[u32; 8]; 8]) -> f32
{
    let board: [[u32; 8]; 8] = parse(boardstate);
    let mut score = 0f32;
    for rank in 0..8
    {
        for file in 0..8
        {
            let color: u32 = board[rank][file];
            if color > 0{
                println!("{}",color);
                score += dbg!(evaluate_pawn(&board, rank as u32, file as u32, color));
            }
        }
    }
    score

}

fn parse(boardstate: &[[u32; 8]; 8]) -> [[u32; 8]; 8]
{
    let mut board: [[u32; 8]; 8] = *boardstate;
    for i in 0..8{
        for j in 0..8{
            if boardstate[i][j] == constants::WPAWN
            {
                board[i][j] = 1;
            }
            else if boardstate[i][j] == constants::BPAWN
            {
                board[i][j] = 2;
            }
            else
            {
                board[i][j] = 0;
            }
        }
    }
    board
}

fn evaluate_pawn(board: &[[u32; 8]; 8], row: u32, col:u32, color: u32) -> f32
{
    let mut score = 0f32;

    //if is_isolated(board, row, col, color){score -= 0.5}
    //if is_doubled(board, row, col, color){score -= 0.3}
    if is_backward(board, row, col, color){score -= 0.5}
    //if is_passed(board, row, col, color){score += 0.5}
    //if is_connected(board, row, col, color){score += 0.5}
    
    score *( -((2*color) as f32-3f32))
}



fn is_isolated(board: &[[u32; 8]; 8], _row: u32, col:u32, color: u32)-> bool
{
    //Check adjacent files for pawns
    
    let left_file:u32 = col.saturating_sub(1);
    let right_file:u32 = col +1;

    for file in [right_file,left_file]
    {
        if file<=7
        {
            if board.iter().any(|&rank| rank[file as usize] == color)
            {
                return false;
            }
        }
    }
    true

}

fn is_doubled(board: &[[u32; 8]; 8], row: u32, col:u32, color: u32)-> bool
{
    //Check if theres another pawn of the same color in this file

    (0..=7).any(|rank: usize| board[rank][col as usize] == color && rank as u32 !=  row)

}

fn is_backward(board: &[[u32; 8]; 8], row: u32, col:u32, color: u32)-> bool
{
    // Has no support and is alone in its file

    let direction: i32;
    if color == 1{direction=-1}else{direction=1}

    
    for r in ((direction)+row as i32)..(4-(4*direction))
    {
        if board[r as usize][col as usize] == color{false;}
    }
    true
}

fn is_passed(board: &[[u32; 8]; 8], row: u32, col:u32, color: u32)-> bool
{
    // Passed pawn has no opposing pawn in its path

    let o_color: u32 = 1+(color%2); 
    let direction: i32;
    if color == 1{direction=-1}else{direction=1}
    let back_col: u32 = col.saturating_sub(1);

    for r in ((direction)+row as i32)..(4-(4*direction))
    {
        if ((back_col)..(min(8,col+2))).any(| rank: u32| board[rank as usize][r as usize] == o_color){false;}
    }
    true
}

fn is_connected(board: &[[u32; 8]; 8], _row: u32, col:u32, color: u32)-> bool
{
    // Connected pawn is any friendly in the same or neighbouring row

    let adjacent_files: [u32; 2] = [col.saturating_sub(1), col +1];
    for file in adjacent_files
    {
        if file <8
        {
            if (adjacent_files[0]..min(8,col+2)).any(&| rank| board[rank as usize][col as usize] == color){true;}
        }
    }
    false
}