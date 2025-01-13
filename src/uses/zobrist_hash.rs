use std::{collections::HashMap, ops::BitXor};


use super::{constants::{Board, PIECENUM, SIZE}, importer};

pub struct ZobristHashTable
{
    keys: [[[u64; PIECENUM as usize]; SIZE as usize]; SIZE as usize],//List of keys indexed by rank, file, piece code
    table: HashMap<u64, f32>
}

impl ZobristHashTable
{
    fn new() -> ZobristHashTable
    {
        let path = "path";

        let z = ZobristHashTable{table: HashMap::new(), keys: importer::read_zcode_board(path).expect("Error importing keys")};

        z
    }

    pub fn check_pos(& self,boardstate: &Board) -> Option<f32>
    {
        let h: u64 = self.hash(boardstate);
        
        self.table.get(&h).copied()
    }

    pub fn insert(& mut self, boardstate: &Board, score: f32)
    {
        let h: u64 = self.hash(boardstate);

        if !self.table.contains_key(&h)
        {
            self.table.insert(h, score);
        }
    }

    fn hash(& self, boardstate: &Board) -> u64
    {
        let mut hash: u64 = 0;

        for i in 0..(SIZE as usize)
        {for j in 0..(SIZE as usize){
            let p: usize = boardstate[i][j] as usize;
            let key = self.keys[i][j][p];
            hash = hash.bitxor(key);
        }}

        return hash;

    }
}