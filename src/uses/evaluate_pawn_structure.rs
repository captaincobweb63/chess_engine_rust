
mod constants;

pub fn evaluate_pawn_structure(boardstate: &[[u32; 8]; 8]) -> f32
{

}

fn parse(boardstate: &[[u32; 8]; 8]) -> [[u32; 8]; 8]
{
    for i in 0..7{
        for j in 0..7{
            if board[i][j] == constants::WPAWN
            {
                board[i][j] = 1
            }
            else if board[i][j] == constants::BPAWN{board[i][j] = 2}
            else{board[i][j] = 0}
        }
    }
}

fn evaluate_pawn(boardstate: &[[u32; 8]; 8], row: u32, col:u32, color: u32) -> f32
{
    score = 0f;
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

    let direction = color;

    for r in ((direcxtion)+row)..((8*direction))
}