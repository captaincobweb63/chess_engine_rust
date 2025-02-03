use std::f32::INFINITY;

use crate::uses::debug_tools;

use super::{constants::*, evaluate::*, find_moves::*};

pub fn minimax(position: Board, depth: u32, mut alpha: f32, mut beta: f32, threshold: f32, minmax: bool, verbose: bool) -> f32
{
    if verbose{println!("Starting minimax - Depth = {}:  ", depth);}
    if depth == 0
    {
        let score: f32 = evaluate_board(&position, verbose);
        if verbose{println!("reached depth: score ={}",score);}
        return score;
    }

    if minmax
    {
        let mut max_eval: f32 = -INFINITY;

        let next_moves = find_moves(&position, true, verbose);

        if next_moves.len() == 0 {return 0f32;}

        for p in next_moves
        {
            if verbose{println!("Searching from (White to move) : \n{}\n--------------------",debug_tools::format_board(p));}
            let eval: f32 = minimax(p, depth-1, alpha, beta, threshold,false, verbose);
            if verbose{println!("Finished with score : {} \n --------------------",eval);}
            max_eval = f32::max(eval, max_eval);
            alpha = f32::max(alpha, eval);
            if beta <= alpha + threshold{
                if verbose{println!("Alpha Pruned \n{}",debug_tools::format_board(p));}
                break;
            }
            
        }
        return max_eval;
    }
    else {

        let mut min_eval: f32 = INFINITY;

        let next_moves = find_moves(&position, false,verbose);

        if next_moves.len() == 0 {return 0f32;}

        for p in next_moves
        {
            if verbose{println!("Searching from (Black to move) : \n{}\n--------------------",debug_tools::format_board(p));}
            let eval: f32 = minimax(p, depth -1, alpha, beta,threshold, true,verbose);
            if verbose{println!("Finished with score : {} \n --------------------",eval);}
            min_eval = f32::min(min_eval, eval);
            beta = f32::min(beta, eval);
            
            if beta >= alpha + threshold
            {
                if verbose{println!("Alpha Pruned \n{}",debug_tools::format_board(p));}
                break;
            }
        }

        
        return min_eval;

    }

}