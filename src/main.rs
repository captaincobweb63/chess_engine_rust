

// DEBUG

#[allow(dead_code)]

// ENDDEBUG

mod evaluate;
mod update;
mod uses;


fn main() {

    let wpawn : u32 = 1;
    let wrook : u32 = 2;
    let wknight : u32 = 3;
    let wbishop : u32 = 4;
    let wqueen : u32 = 5;
    let wking : u32 = 6;
    let bpawn :u32 = 7;
    let brook :u32 = 8;
    let bknight : u32 = 9;
    let bbishop : u32 = 10;
    let bqueen : u32 = 11;
    let bking : u32 = 12;

    let mut board: [[u32; 8]; 8] = [[0; 8]; 8];

    for i in 0..7 {
        board[i].fill(0);
    }
    
    
    board[0][1] = bpawn;
    board[1][1] = brook;

    board[5][2] = wqueen;
    board[2][2] = wpawn;



    update::update_board(&mut board);

    print!("\n{}\n", evaluate::evaluate_board(&board));
}

