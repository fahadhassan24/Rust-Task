use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error,ErrorKind};

fn main() -> Result<(), Error> {
    let mut bank:     [u32; 8] = [20;8];

    // start of Program

    let file = "bank.txt";
    let read_file = File::open(file);
    let mut read = true;

    let file_data = match read_file {
        Ok(f_data) => {
            read = true;
            f_data
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file) {
                Ok(fc) => {
                    read = false;
                    fc
                },
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };


    if read{
        let buffered = BufReader::new(file_data);
        let mut lop=0;

        for line in buffered.lines() {
            let data = line?;
            let data = data.parse::<u32>().unwrap();
            bank[lop] = data;
            lop=lop+1;
        }
    }

    //end of program
    // ************* Writing bank data to file
    let mut output = File::create(file)?;
    println!("");
    println!("");
    println!("Writing Cash Box fill level to File");
    for lop in 0..bank.len() {
        write!(output, "{}\n",bank[lop].to_string())?;
    }
    // ************* Writing bank data to file

    Ok(())
}
