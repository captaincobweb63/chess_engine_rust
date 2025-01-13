use std::{f32::INFINITY, sync::{Arc, Mutex}};
use rand::Rng;

use super::{constants::*, evaluate::evaluate_board, find_moves, minimax::minimax, zobrist_hash::ZobristHashTable};

pub struct Searcher
{
    hashes: Arc<Mutex<ZobristHashTable>>,
    global_best_move: Arc<Mutex<(Board, f32)>>,
    moves_to_evaluate: Vec<Board>,
    best_move: (Board, f32),
    depth: u32, // How many moves to search.
    to_move: bool,
}

impl Searcher
{
       pub fn new(
        hashes: Arc<Mutex<ZobristHashTable>>,
        moves_to_evaluate: Vec<Board>,
        global_best_move: Arc<Mutex<(Board,f32)>>,
        depth: u32,
        to_move: bool
       ) -> Self {
            Self{
                hashes,
                moves_to_evaluate,
                best_move: ([[0;8];8],0f32),
                global_best_move,
                depth,
                to_move
            }
       }

       pub fn evaluate(&mut self, weight: f32)
       {
            let mut best_move: Board;
            let mut best_score: f32 = -INFINITY;
            let mut alpha: f32 = -INFINITY;
            let mut beta: f32 = INFINITY;

            for m in self.moves_to_evaluate.iter()
            {
                let mut score = 0f32;
                let hash_result: Option<f32>;
                {
                let hashtable = self.hashes.lock().unwrap();
                hash_result = hashtable.check_pos(m).clone();
                }

                match hash_result {
                    Some(value) =>
                    {
                        score = value;
                    }
                    None =>
                    {
                        score = minimax(*m,self.depth,alpha,beta,self.to_move);

                        {
                        let mut hashtable = self.hashes.lock().unwrap();
                        hashtable.insert(m, score);
                        }

                    }
                    
                }

                if weight != 0.0
                {
                    score += rand::thread_rng().gen_range(-weight..=weight);
                }

                if score > best_score
                {
                    best_score = best_score;
                    best_move = *m;

                    self.best_move = (best_move.clone(), best_score);

                    {
                        let mut global_best = self.global_best_move.lock().unwrap();
                        if best_score > global_best.1
                        {
                            *global_best = self.best_move.clone();
                        }
                    }
                }

            }

       }
}

pub fn search_control(board: Board, depth: u32, weight: f32, to_move: bool)
{

    //WRITE TRHEADING SHIT HERE

}