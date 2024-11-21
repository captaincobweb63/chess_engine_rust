use std::{collections::HashMap, result};
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use super::constants::{self, BBISHOP, BKING, BKNIGHT, BPAWN, BQUEEN, BROOK, EMPTY, WBISHOP, WKING, WKNIGHT, WPAWN, WQUEEN, WROOK};

pub fn read_csv(path: &str) -> Result<[[u32; 8]; 8],Box<dyn Error>>
{

    let mut board: [[u32; 8]; 8] = [[0;8];8];

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
                    println!("{}",piece);
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