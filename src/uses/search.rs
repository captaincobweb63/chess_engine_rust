use std::{f32::{INFINITY, NEG_INFINITY}, sync::{Arc, Mutex}, thread};
use rand::Rng;

use crate::uses::debug_tools;

use super::{constants::*, find_moves::find_moves, minimax::minimax, zobrist_hash::ZobristHashTable};

pub struct Searcher
{
    hashes: Arc<Mutex<ZobristHashTable>>,
    global_best_move: Arc<Mutex<(Board, f32)>>,
    moves_to_evaluate: Vec<Board>,
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
                global_best_move,
                depth,
                to_move
            }
       }

       pub fn evaluate(&mut self, weight: f32, threshold: f32, colour: bool, verbose: bool)
       {
            let mut best_move: Board = self.moves_to_evaluate[0];
            let mut best_score: f32 = if colour{-INFINITY} else{INFINITY};
            let alpha: f32 = -INFINITY;
            let beta: f32 = INFINITY;

            for m in self.moves_to_evaluate.iter()
            {
                let mut score ;
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
                        score = minimax(*m,self.depth,alpha,beta,threshold,!self.to_move, verbose);

                        if verbose{println!("score of :{} for move:\n{}",score,debug_tools::format_board(*m));}

                        {
                        let mut hashtable = self.hashes.lock().unwrap();
                        hashtable.insert(m, score);
                        }

                    }
                    
                }

                if weight != 0.0
                {
                    score += rand::rng().random_range(-weight..=weight);
                }


                if verbose{println!("{}>{}? vs {} = {}", score, best_score, colour,(score > best_score) ^ !colour);}

                if (score > best_score) ^ !colour
                {
                    best_score = score;
                    best_move = *m;
                }

            }

            let comb_best = (best_move.clone(), best_score);

            {
                let mut global_best = self.global_best_move.lock().unwrap();
                if verbose{println!("gbest: {}\n lbest {}",global_best.1,best_score);}
                if (best_score > global_best.1) ^ !colour
                {
                    if verbose {println!("attempting to change gbest");}
                    *global_best = comb_best.clone();
                }
            }

       }
}

pub fn search_control(board: Board, depth: u32, weight: f32, threshold: f32, to_move: bool, mut threads: i32, verbose: bool) -> (Board, f32)
{
    let mut handles:Vec<thread::JoinHandle<()>>  = vec![];
    let global_best_move:Arc<Mutex<(Board, f32)>> = Arc::new(Mutex::new((board.clone(), if to_move {NEG_INFINITY} else {INFINITY})));
    let hashes: Arc<Mutex<ZobristHashTable>> = Arc::new(Mutex::new(ZobristHashTable::new()));   

    let mut possible_moves: Vec<Board> = find_moves(&board, to_move, verbose);

    if true && threads > possible_moves.len() as i32{println!("Thread count throttled to :{}",possible_moves.len());}
    threads = if possible_moves.len() as i32 > threads {threads} else{possible_moves.len() as i32};

    let moves_per_thread: i32  = (possible_moves.len() as i32)/threads;

    for _ in 1..threads
    {
        let mut thread_moves: Vec<Board> = vec![];
        for _ in 0..moves_per_thread
        {
            thread_moves.push(possible_moves.pop().expect("error"));
        }

        let mut instance: Searcher = Searcher::new(
            Arc::clone(&hashes), 
            thread_moves,
            Arc::clone(&global_best_move), 
            depth, to_move);
        
        let handle = thread::spawn(move ||
        {
            instance.evaluate(weight, threshold, to_move, verbose);
        });
        handles.push(handle);
    }
    let mut instance: Searcher = Searcher::new(
        Arc::clone(&hashes), 
        possible_moves,
        Arc::clone(&global_best_move), 
        depth, to_move);
    
    let handle = thread::spawn(move ||
    {
        instance.evaluate(weight, threshold, to_move, verbose);
    });
    handles.push(handle);

    for handle in handles
    {
        handle.join().unwrap();
    }

    let final_move =  global_best_move.lock().unwrap();
    return *final_move;


}