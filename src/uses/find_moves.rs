use std::{usize, vec};

use crate::{BROOK, WBISHOP, WKING, WKNIGHT, WROOK};

use super::{constants::{Board, Moves, BPAWN, EMPTY, SIZE, WPAWN, WQUEEN}, debug_tools};


pub fn find_moves(board: &Board, to_move: bool, verbose: bool) -> Vec<Board>
{

    
    //let verbose = false;
    
    let mut moves : Vec<Board> = vec![];

    for rank in 0..SIZE as usize
    {
        for file in 0..SIZE as usize{

            let mut piece = board[rank][file];


            if to_move == (piece<BPAWN)
            {
                let color: bool =  to_move;//Is white?
                piece = (((piece as i32 -1)%6)+1) as u32;
                
                let pos = (rank,file);

                let board_use: &Board = &(board.clone());

                match piece
                {
                    WPAWN => moves.append(&mut pawn_moves(board_use,pos, color)),
                    WROOK => moves.append(&mut rook_moves(board_use, pos, color)),
                    WKNIGHT => moves.append(&mut knight_moves(board_use, pos, color)),
                    WBISHOP => moves.append(&mut bishop_moves(board_use, pos, color)),
                    WQUEEN => moves.append(&mut queen_moves(board_use, pos, color)),
                    WKING => moves.append(&mut king_moves(board_use, pos, color)),

                    _ => ()
                }
                if verbose {for m in moves.clone()
                {
                    println!("Found move:\n{}",debug_tools::format_board(m))
                }}
            }
        }
    }
    


    moves

}





fn pawn_moves(board: &Board, pos: (usize,usize), color: bool)->Vec<Board>
{
    let mut moves:Moves = vec![];
    let forwards: usize;

    let (y,x) = pos;

    if color//Set the vector for forwards movement
    {
        forwards = pos.0+1;
    }else{
        forwards = pos.0.saturating_sub(1);
    }

    let mut piece: u32 = board[pos.0][pos.1];
    if forwards == 0 || forwards == 7{piece+=WQUEEN-WPAWN;} // handles promotion

    
    if pos.1.saturating_sub(1) != pos.1 && ((board[forwards][pos.1.saturating_sub(1)]>=BPAWN) ^ !color) && board[forwards][pos.1.saturating_sub(1)] != EMPTY// if isn't at the low edge & (there is)
    {
        let mut nb: Board = board.clone();

        nb[forwards][pos.1.saturating_sub(1)] = piece;
        nb[pos.0][pos.1]=EMPTY;

        moves.push(nb);
    }
    if pos.1 + 1 < 8 && ((board[forwards][pos.1+1]>=BPAWN) ^ !color) && board[forwards][pos.1+1] != EMPTY
    {
        let mut nb: Board = board.clone();
        nb[forwards][pos.1+1] = piece;
        nb[pos.0][pos.1]=EMPTY;

        moves.push(nb);
    }



    if board[forwards][pos.1] == EMPTY   
    {

        let mut nb: Board = board.clone();


        nb[forwards][pos.1]=piece;
        nb[pos.0][pos.1]=EMPTY;

        moves.push(nb);

        if y == if color {1} else {6} && board[if color {3} else {4}][x] == EMPTY
        {
            let mut nb: Board = board.clone();

            let double_forwards = if color {3} else {4};
            nb[double_forwards][x] = piece;
            nb[pos.0][pos.1] = EMPTY;

            moves.push(nb);
        }
    }



    moves

}

fn rook_moves(board: &Board, pos: (usize,usize), color: bool)->Vec<Board>
{
    let deltas = vec![(0,1),(1,0),(0,-1),(-1,0)];

    let moves = uncapped_moves(board, pos, color, deltas);

    return moves
}

fn knight_moves(board: &Board, pos: (usize,usize), color: bool)->Vec<Board>
{

    let deltas = vec![(2,1),(1,2),(-2,1),(-1,2),(2,-1),(1,-2),(-2,-1),(-1,-2)];

    let moves = capped_moves(board, pos, color, deltas);

    moves
}

fn bishop_moves(board: &Board, pos: (usize,usize), color: bool)->Vec<Board>
{

    let deltas = vec![(1,1),(1,-1),(-1,1),(-1,-1)];

    let moves = uncapped_moves(board, pos, color, deltas);

    moves
}

fn queen_moves(board: &Board, pos: (usize,usize), color: bool)->Vec<Board>
{

    let deltas = vec![(1,0),(-1,0),(0,1),(0,-1),(1,1),(1,-1),(-1,1),(-1,-1)];
    
    let moves = uncapped_moves(board, pos, color, deltas);

    moves
}

fn king_moves(board: &Board, pos: (usize,usize), color: bool)->Vec<Board>
{

    let deltas = vec![(1,0),(-1,0),(0,1),(0,-1),(1,1),(1,-1),(-1,1),(-1,-1)];
    let mut moves = capped_moves(board, pos, color, deltas);

    let y = pos.0;
    let x = pos.1;
    let piece = board[y][x];

    let home_rank: usize = if color {0} else {7};
    let rook: u32 = if color {WROOK} else {BROOK};

    if y == home_rank && x == 4 // CASTLING
    {
        if board[home_rank][0]==rook && board[home_rank][1] == EMPTY && board[home_rank][2] == EMPTY && board[home_rank][3] == EMPTY
        {
            let mut nb: Board = board.clone();
            
            nb[home_rank][2] = piece;
            nb[home_rank][3] = board[home_rank][0];

            nb[home_rank][0] = EMPTY;
            nb[y][x] = EMPTY;

            moves.push(nb);

        }
        if board[home_rank][7]==rook && board[home_rank][6] == EMPTY && board[home_rank][5] == EMPTY
        {
            let mut nb: Board = board.clone();

            nb[home_rank][6] = piece;
            nb[home_rank][5] = board[home_rank][7];

            nb[home_rank][7] = EMPTY;
            nb[y][x] = EMPTY;

            moves.push(nb);
        }

    }


    return moves;
}


fn uncapped_moves(board: &Board, pos: (usize,usize), color: bool, deltas: Vec<(i32,i32)>) -> Vec<Board>
{

    let mut moves: Vec<Board> = vec![];
    let piece = board[pos.0][pos.1];

    for d in deltas
    {
        let mut safe = true;
        let mut count = 1;
        while safe 
        {
            let real_delta = (d.0*count,d.1*count);
            
            let mut nb = board.clone();

            if (real_delta.0 + pos.0 as i32) < 0 || (real_delta.1 + pos.1 as i32) < 0 
            || (real_delta.0 + pos.0 as i32) >= SIZE as i32|| (real_delta.1 + pos.1 as i32) >= SIZE as i32
            {break;}

            let ny = (real_delta.0 + pos.0 as i32) as usize;
            let nx = (real_delta.1 + pos.1 as i32) as usize;


            nb[ny][nx] = piece;
            nb[pos.0][pos.1] = EMPTY;

            if board[ny][nx] != EMPTY {safe = false;}

            if board[ny][nx] == EMPTY || ((board[ny][nx] < BPAWN) ^ color)
            {
                moves.push(nb);
            } 
            count  += 1;
        }
    }

    moves
}

fn capped_moves(board: &Board, pos: (usize,usize), color: bool, deltas: Vec<(i32,i32)>) -> Vec<Board>
{

    let mut moves: Vec<Board> = vec![];
    let piece = board[pos.0][pos.1];

    for d in deltas
    {       
        let mut nb = board.clone();

        if !((d.0 + pos.0 as i32) < 0 || (d.1 + pos.1 as i32) < 0 
        || (d.0 + pos.0 as i32) >= SIZE as i32|| (d.1 + pos.1 as i32) >= SIZE as i32)
        {

            let ny = (d.0 + pos.0 as i32) as usize;
            let nx = (d.1 + pos.1 as i32) as usize;


            nb[ny][nx] = piece;
            nb[pos.0][pos.1] = EMPTY;

            if board[ny][nx] == EMPTY || ((board[ny][nx] < BPAWN) ^ color)
            {
                moves.push(nb);
            } 
        }
    }

    moves
}
