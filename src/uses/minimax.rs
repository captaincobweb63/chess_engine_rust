use std::f32::INFINITY;

use super::{constants::*, evaluate::*, find_moves::*};

pub fn minimax(position: Board, depth: u32, mut alpha: f32, mut beta: f32, minmax: bool) -> f32
{
    if depth == 0
    {
        let score: f32 = (evaluate_board(&position));
        return score;
    }

    if minmax
    {
        let mut max_eval: f32 = -INFINITY;

        for p in find_moves(&position, true)
        {
            let eval: f32 = minimax(p, depth-1, alpha, beta, false);
            max_eval = f32::max(eval, max_eval);
            alpha = f32::max(alpha, eval);
            if beta <= alpha{
                break;
            }
        }
        
        return max_eval;
    }
    else {

        let mut min_eval: f32 = INFINITY;

        for p in find_moves(&position, false)
        {
            let eval: f32 = minimax(p, depth -1, alpha, beta, true);
            min_eval = f32::min(min_eval, eval);
            beta = f32::min(beta, eval);
            if beta <= alpha
            {
                break;
            }
        }

        return min_eval;
        
    }

}