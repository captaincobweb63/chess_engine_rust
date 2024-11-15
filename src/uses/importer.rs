use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

fn read_csv(path: &str) -> Result<(), Box<dyn Error>>
{
    let file = File::open(path)?;
    
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for result in rdr.records()
    {
        let record = result?;
        println!("{:?}",record);
    }


    Ok(())

}