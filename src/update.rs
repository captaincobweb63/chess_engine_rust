// DEBUG

#[allow(dead_code)]

// ENDDEBUG

use crate::uses::piece::Piece;

pub fn update_board(boardstate: &mut [[u32; 8]; 8])
{
    let width: u32 = boardstate[0].len() as u32;
    let height: u32 = boardstate.len() as u32;

    for row_i in 0..height
    {
        for col_i in 0..width
        {
            //pass
        }
    }
}