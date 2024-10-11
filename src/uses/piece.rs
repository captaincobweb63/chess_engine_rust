

#[allow(dead_code)]
#[derive(Clone)]
pub struct Piece
{
    pub code:u32,
    pub name: String,
    pub white: bool,
    pub score: u32,
    moves: Vec<[i16;2]>,
    pub position: [u32;2],

}
impl Piece
{
    pub fn return_moves(&self) -> Vec<[u32;2]>
    {
        let mut movelist : Vec<[u32;2]> = Vec::new();

        movelist.push([1,1]);
        // Add logic for checking moves

        return movelist;
    }

    pub fn new() -> Piece
    {
        Self { 
            code: 0,
            name: "empty".to_string(),
            white: true,
            score: 0,
            moves: Vec::new(),
            position: [0,0]
        }
    }
}