
pub type Board = [[u32; 8]; 8];
pub type Moves = Vec<Board>;

pub const TWOROOTTWO: f32 = 2.82842712475;
pub const SCORETABLE:[f32; 7] = [0.0,0.5,2.5,1.5,1.7,4.5,100.0];

pub const WPAWN : u32 = 1;
pub const WROOK : u32 = 2;
pub const WKNIGHT : u32 = 3;
pub const WBISHOP : u32 = 4;
pub const WQUEEN : u32 = 5;
pub const WKING : u32 = 6;
pub const BPAWN :u32 = 7;
pub const BROOK :u32 = 8;
pub const BKNIGHT : u32 = 9;
pub const BBISHOP : u32 = 10;
pub const BQUEEN : u32 = 11;
pub const BKING : u32 = 12;
pub const EMPTY : u32 = 0;

pub const PIECENUM: u32 = 12;
pub const SIZE:u32 = 8;
pub const SQUARES: u32 = SIZE*SIZE;