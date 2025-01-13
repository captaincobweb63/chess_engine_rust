use std::{usize, vec};

use crate::{BROOK, WBISHOP, WKING, WKNIGHT, WROOK};

use super::constants::{Board, Moves, BPAWN, EMPTY, SIZE, WPAWN, WQUEEN};


pub fn find_moves(board: &Board, to_move: bool) -> Vec<Board>
{
    let moves : Vec<Board> = vec![];

    for rank in 0..SIZE as usize
    {
        for file in 0..SIZE as usize{

            let mut piece = board[rank][file];


            if to_move == (piece<BPAWN)
            {
                let color: bool =  to_move;//Is white?
                piece = ((piece-1)%6)+1;
                
                let pos = (rank,file);

                match piece
                {
                    WPAWN => return pawn_moves(board,pos, color),
                    WROOK => return rook_moves(board, pos, color),
                    WKNIGHT => return knight_moves(board, pos, color),
                    WBISHOP => return bishop_moves(board, pos, color),
                    WQUEEN => return queen_moves(board, pos, color),
                    WKING => return king_moves(board, pos, color),

                    _ => return vec![],

                }
            }
        }
    }
    


    moves

}





fn pawn_moves(board: &Board, pos: (usize,usize), color: bool)->Vec<Board>
{
    let mut moves:Moves = vec![];
    let forwards: usize;

    let (x,y) = pos;

    if color
    {
        forwards = pos.0+1;
    }else{
        forwards = pos.0-1;
    }

    let mut piece: u32 = board[pos.0][pos.1];
    if forwards == 0 || forwards == 7{piece+=WQUEEN-WPAWN;}

    
    if (board[forwards][pos.1-1]>=BPAWN) ^ !color
    {
        let mut nb: Board = board.clone();
        nb[forwards][pos.1-1] = piece;
        nb[pos.0][pos.1]=EMPTY;

        moves.push(nb);
    }
    if (board[forwards][pos.1+1]>=BPAWN) ^ !color
    {
        let mut nb: Board = board.clone();
        nb[forwards][pos.1+1] = piece;
        nb[pos.0][pos.1]=EMPTY;

        moves.push(nb);
    }

    if board[forwards][pos.1] != EMPTY  
    {
        let mut nb: Board = board.clone();
        nb[forwards][pos.1]=board[pos.0][pos.1];
        nb[pos.0][pos.1]=0;

        if(y == if color {1} else {6} && board[x][if color {3} else {4}] == EMPTY)
        {
            let mut nb: Board = board.clone();

            nb[x][if color {3} else {4}] = board[x][y];
            nb[x][y] = EMPTY;

            moves.push(nb);
        }
    }

    moves

}

fn rook_moves(board: &Board, pos: (usize,usize), color: bool)->Vec<Board>
{
    let piece = board[pos.0][pos.1];

    let mut moves: Moves = vec![];

    let mut directions: [Box<dyn Iterator<Item = usize>>; 4] = [
                                Box::new(pos.0+1..=SIZE as usize),
                                Box::new((0usize..pos.0).rev()), 
                                Box::new(pos.1+1..SIZE as usize), 
                                Box::new((0usize..pos.1).rev())];

    for i in 0..directions.len() {
        let mut x = pos.0;
        let mut y = pos.1;
        
        let d = &mut directions[i]; // mutable
        
        for j in d.by_ref() {
            if i < 2 {
                x = j; // Update `x` for the x move
            } else {
                y = j; // Update `y` for the y move
            }

            if board[x][y]!=0
            {
                if (board[x][y]<BPAWN) ^ color
                {
                    let mut nb: Board = board.clone();
                    nb[pos.0][pos.1]=EMPTY;
                    nb[x][y]=piece;

                    moves.push(nb); 
                    break;
                }
            }

            let mut nb: Board = board.clone();
            nb[pos.0][pos.1]=EMPTY;
            nb[x][y]=piece; 

            moves.push(nb);
        }
    }

    return moves
}

fn knight_moves(board: &Board, pos: (usize,usize), color: bool)->Vec<Board>
{
    let mut moves: Moves = vec![];

    let piece = board[pos.0][pos.1];

    let ipos = (pos.0 as i32, pos.1 as i32);

    let possible = [(ipos.0+2,ipos.1+1),(ipos.0+2,ipos.1-1),
                                        (ipos.0+1,ipos.1+2),(ipos.0+1,ipos.1-2),
                                        (ipos.0-1,ipos.1+2),(ipos.0-1,ipos.1-2),
                                        (ipos.0-2,ipos.1+1),(ipos.0-2,ipos.1-1)];

    for npos in possible
    {
        if (npos.0.max(npos.1)<8 && npos.0.min(npos.1)>=0) && ((board[npos.0 as usize][npos.1 as usize]>=BPAWN) ^ color)
        {
            let mut nb = board.clone();
            nb[npos.0 as usize][npos.1 as usize] = piece;
            nb[pos.0][pos.1] = EMPTY;

            moves.push(nb);
        }
    }
    return moves;
}

fn bishop_moves(board: &Board, pos: (usize,usize), color: bool)->Vec<Board>
{
    let mut moves: Moves = vec![];

    let i_pos = (pos.0 as i32, pos.1 as i32);

    for i in 1..8
    {
        let directions = [(i_pos.0+i,i_pos.1+i), (i_pos.0+i, i_pos.1-i), (i_pos.0-i, i_pos.1+i), (i_pos.0-i, i_pos.1-i)];
        let mut flags = [false,false,false,false];


        for j in 0..4
        {
            if !flags[j]
            {
                let d = directions[j];
                if (8 > d.0 && d.0 >= 0) && (8 > d.1 && d.0 >=0) // is the move on the board
                {
                    let mut nb = board.clone();

                    flags[j] = board[d.0 as usize][d.1 as usize] != EMPTY; // is moving to an occupied square

                    nb[d.0 as usize][d.1 as usize] = board[pos.0][pos.1];
                    nb[pos.0][pos.1]=EMPTY;

                    if (board[d.0 as usize][d.1 as usize]==EMPTY) || ((board[d.0 as usize][d.1 as usize] < BPAWN) ^ color) // is moving to an empty or capturable square
                    {
                        moves.push(nb);
                    }
                }
                else
                {
                    flags[j] = true;
                }
            }
        }


        
    }

    return moves;
}

fn queen_moves(board: &Board, pos: (usize,usize), color: bool)->Vec<Board>
{
    let mut moves: Moves = vec![];

    moves.append(&mut rook_moves(board, pos, color));
    moves.append(&mut bishop_moves(board, pos, color));

    return moves;
}

fn king_moves(board: &Board, pos: (usize,usize), color: bool)->Vec<Board>
{
    let mut moves: Moves = vec![];
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    let (x, y) = pos;

    for (dx, dy) in directions.iter() 
    {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && nx < 8 && ny >= 0 && ny < 8
        {
            let nx = nx as usize;
            let ny = ny as usize;

            if(board[nx][ny] == EMPTY)||((board[nx][ny]<BPAWN)^ color)
            {
                let mut nb: Board = board.clone();

                nb[nx][ny] = board[x][y];
                nb[x][y] = EMPTY;

                moves.push(nb);
            }
        }
    }

    let home_rank: usize = if color {0} else {8};
    let rook: u32 = if color {WROOK} else {BROOK};

    if y==home_rank && x == 4
    {
        if board[0][home_rank]==rook && board[1][home_rank] == EMPTY && board[2][home_rank] == EMPTY && board[3][home_rank] == EMPTY
        {
            let mut nb: Board = board.clone();
            
            nb[2][home_rank] = board[x][y];
            nb[3][home_rank] = board[0][home_rank];

            nb[0][home_rank] = EMPTY;
            nb[x][y] = EMPTY;

            moves.push(nb);

        }
        if board[7][home_rank]==rook && board[6][home_rank] == EMPTY && board[5][home_rank] == EMPTY
        {
            let mut nb: Board = board.clone();

            nb[6][home_rank] = board[x][y];
            nb[5][home_rank] = board[7][home_rank];

            nb[7][home_rank] = EMPTY;
            nb[x][y] = EMPTY;

            moves.push(nb);
        }

    }


    return moves;

}
