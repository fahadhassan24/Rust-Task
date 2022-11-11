use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "lines.txt";

    let mut output = File::create(path)?;
    let mut bank:     [u32; 8] = [20;8];
    for lop in 0..bank.len() {
        write!(output, "{}\n",bank[lop].to_string())?;
    }
    

    let input = File::open(path)?;
    
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        // println!("{}", line?);
        let data = line?;
        let data = data.parse::<u32>().unwrap();
        println!("{}",data);
    }

    Ok(())
}