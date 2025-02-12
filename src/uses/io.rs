use super::constants::Board;

pub fn headless_format(mut board_string: String) -> Board {
    let refs = "xabcdefghijkl";
    let mut board: Board = [[0; 8]; 8];

    for i in 0..8 {
        for j in 0..8 {
            let current = board_string.pop().unwrap();
            board[j][i] = refs.chars().position(|c| c == current).unwrap() as u32;
        }
    }

    board
}

pub fn headless_read(board: Board) -> String
{
    let refs = "xabcdefghijkl";
    let mut board_string: String = "".to_string();

    for i in 0..8{ for j in 0..8{
        board_string.push(refs.as_bytes()[board[j][i] as usize] as char);
    }}

    board_string 
}
