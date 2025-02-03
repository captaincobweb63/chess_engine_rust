use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use super::constants::{*};
pub fn read_board_csv(path: &str) -> Result<Board,Box<dyn Error>>
{

    let mut board: Board = [[0;8];8];

    let mut dict=HashMap::new();

    dict.insert("WP", WPAWN);
    dict.insert("WB",WBISHOP);
    dict.insert("WR", WROOK);
    dict.insert("WN", WKNIGHT);
    dict.insert("WQ",WQUEEN);
    dict.insert("WK", WKING);
    dict.insert("BP",BPAWN);
    dict.insert("BB", BBISHOP);
    dict.insert("BR", BROOK);
    dict.insert("BN", BKNIGHT);
    dict.insert("BQ", BQUEEN);
    dict.insert("BK", BKING);
    dict.insert("E", EMPTY);

    let file = File::open(path).expect("Error reading csv");
    
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut j = 0_usize;    
    for  record in rdr.records()
    {
        match record
        {
            Ok(record) => 
            {
                let mut row: [u32; 8] = [0;8];

                let mut i = 0_usize;
                for piece in record.iter()
                {
                    let code = dict.get(piece).cloned().unwrap_or(0);
                    row[i]=code;
                    i+=1;
                }

                board[j] = row;

            }Err(e) =>
            {
                eprintln!("Error reading : {}",e)
            }

            
        }
        
        j+=1;

    }

    Ok(board)

}

pub fn read_zcode_board(path: &str) -> Result<[[[u64; PIECENUM as usize]; SIZE as usize]; SIZE as usize], Box<dyn Error>>
{
    let mut keys: Vec<u64> = vec![];

    
    let file = File::open(path).expect("Error reading csv");
    
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for record in rdr.records()
    {
        let value = record?;

        if let Some(hex_str) = value.get(0)
        {
            if let Ok(hex_value) = u64::from_str_radix(hex_str,16)
            {
                keys.push(hex_value);
            }else {
                eprintln!("Error parsing hex value: {}", hex_str);
            }
        }
    }

    let mut keys_arr : [[[u64; PIECENUM as usize]; SIZE as usize]; SIZE as usize] = [[[0 ;PIECENUM as usize]; SIZE as usize]; SIZE as usize];
    
    
    for i in 0..(PIECENUM) as usize
    {
        for j in 0..SIZE as usize
        {
            for k in 0..SIZE as usize
            {
                let line = (64*k)+(8*j)+i;

                let temp = keys[line];
                keys_arr[k][j][i] = temp;

            }
        }
    }
    Ok(keys_arr)
}