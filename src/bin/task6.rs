use std::io;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error,ErrorKind};

fn main() {
    println!("Task6 : ");
    task6();
}

fn task6()-> Result<(), Error>{
    // let random_price : f32 = rand::thread_rng().gen_range(1..=10);
    let mut bank:     [u32; 8] = [20;8];
    // ************* Reading data from file
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

    println!("File check");
    if read{
        println!("Read Done");
        let buffered = BufReader::new(file_data);
        let mut lop=0;

        for line in buffered.lines() {
            let data = line?;
            let data = data.parse::<u32>().unwrap();
            bank[lop] = data;
            lop=lop+1;
        }
    }

    // **************Reading data from File
    loop {
        println!("in Loop");
        let euro = '\u{20AC}';
        let currency: [f32; 8] = [2.0, 1.0, 0.5,0.2, 0.1, 0.05, 0.02,0.01];
        let mut changer: [u32; 8] = [0;8];
        // println!("Euro : {}",euro);
        
        let product = ["Coke 0.33 Ltr",
                        "Pepsi 0.33 Ltr",
                        "San Benedetto Citrus",
                        "Mulino Bianco Pan ",
                        "Ferrero Pocket Coffee",
                        "Condorelli Almond Milk ",
                        "Red Crodino Drinl",
                        "Estathe Lemon Tea ",
                        "Unico Orange snack ",
                        "Dragées Orange Peel",
                        "Dragées Hazelnuts ",
                        "85% Dark Chocolate bar"];
        let price: [f32; 12] = [2.55,2.50,2.50,3.55,3.25,5.55,1.50,2.35,1.95,9.58,9.85,6.68];
        println!("",);
        println!("",);
        println!("__________    Menu    __________",);
        println!("No.      Product                Price",);
        for lop in 0..product.len() {
            print!("{}  {} ",lop+1,product[lop]);
            let sp = 25-product[lop].len();
            for _spac in 0..sp {
                print!("-");
            }
            println!(">  {} {}",price[lop],euro);

        }
        println!(" ");
        println!("Please enter the desired product number :  ");
        println!("Please enter Product # between 1  to 12 or q to exit or s for service menu");
        let product_choice :usize   ;
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        
        if user_input.trim()=="q" || user_input.trim()=="Q" {
            println!("Thank you !");
            break;
        }

        if user_input.trim()=="s" || user_input.trim()=="S" {
            loop {
            println!("Here is your Cash Box");
            println!("");
            println!("  Index    Coin     fill level");
            for lop in 0..bank.len() {
                println!("  {}.        {} -------> {}  ",lop+1,currency[lop],bank[lop]);
            }

            println!("",);
            println!("Please enter the Index number to add / remove coins :  ");
            let cash_index :usize;
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
            
            if user_input.trim()=="q" || user_input.trim()=="Q" {
                println!("Thank you !");
                break;
            }
            cash_index = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter index from 1 to 8 or q to exit");
                    continue},
            };

            if cash_index >0 && cash_index <9 {
            
            }
            else {
                println!("Please enter value between 1 and 8 ");
                continue
            }

            println!("");
            println!("  Index    Coin     fill level");
            println!("  {}.        {} -------> {}  ",cash_index,currency[cash_index-1],bank[cash_index-1]);
            println!("",);
            println!("Please enter the value to add / remove coins   ");
            let value :i8;
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
            
            if user_input.trim()=="q" || user_input.trim()=="Q" {
                println!("Thank you !");
                break;
            }
            value = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter value from 1 to 8 or q to exit");
                    continue},
            };
            
            if value > 0{
                if bank[cash_index-1] + value as u32>0 && bank[cash_index-1] + value as u32<=50 {
                    bank[cash_index-1] = bank[cash_index-1] + value as u32;
                } else {
                    println!("");
                    println!("Sorry value out of bound");

                }
            } else {
                let val : u32 = value.abs() as u32;
                if bank[cash_index-1] - val as u32>0 && bank[cash_index-1] - val as u32<=50 {
                    bank[cash_index-1] = bank[cash_index-1] - val;
                } else {
                    println!("");
                    println!("Sorry value out of bound");
                }
            }
        }
        continue;
        }

        product_choice = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter Product # between 1  to 12 or q to exit or s for service menu");
                continue},
        };

        if product_choice >0 && product_choice < product.len() {
            
        }
        else {
            println!("Please enter Product # between 1  to 12 or q to exit");
            continue
        }

        println!(" ");
        println!("Your Product Number : {} ",product_choice);
        println!("Product :  {}  Price : {} {}",product[product_choice-1],price[product_choice-1],euro);

        let random_price : f32 = price[product_choice-1];
        let random_price :u32 = (random_price* 100.0) as u32;
        let random_price :f32 = random_price as f32/ 100.0;
        println!("",);
        println!("Your Bill : {} {}",random_price,euro);
        println!("",);
        println!("Please pay the bill :  ");
        let user_payment :f32;
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        
        if user_input.trim()=="q" || user_input.trim()=="Q" {
            println!("Thank you !");
            break;
        }
        user_payment = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter value between 0.00 {} to 999.99 {} or q to exit",euro,euro);
                continue},
        };
        println!("You Paid $ {}",user_payment);

        if user_payment >0.00 && user_payment < 999.99 {
            
        }
        else {
            println!("Please enter value between 0.00 {} to 999.99 {}",euro,euro);
            continue
        }

        if user_payment < random_price {
            println!("Please pay >= Product price {} {}",random_price,euro);
            continue
        }
        else {
            let change :u32 = ((user_payment - random_price)* 100.0) as u32;
            let change :f32 = change as f32/ 100.0;
            if change >0.00 {
                
                
                let change_int :u32 = change as u32;
                
                let change_dec :f32 = change - change_int as f32;
                let mut change_dec :u32 = (change_dec* 100.0) as u32;
                // let mut change_dec :f32 = change_dec as f32/ 100.0;
                // let change_int :f32 = change_int as f32;
                if change_int >0 {
                    changer[0] = change_int/2;
                    changer[1] = change_int-(changer[0]*2);

                    if changer[0] > 0 {
                        if bank[0]>=changer[0] {
                            println!(" {}    {} * {} = {} {}",currency[0],euro,changer[0],changer[0]*2,euro);
                        } else {
                            println!("Roll bank transaction .");
                            println!("We are out of some coins");
                            continue;
                        }
                        
                    }
                    if changer[1] > 0 {
                        if bank[1]>=changer[1] {
                            println!(" {}    {} * {} = {} {}",currency[1],euro,changer[1],changer[1]*1,euro);
                        } else {
                            println!("Roll bank transaction .");
                            println!("We are out of some coins");
                            continue;
                        }
                    }
                }
                // println!(" decimal {}  ",change_dec);
                if change_dec > 0 {
                    changer[2]= change_dec/50;
                    if changer[2] > 0 {
                        if bank[2]>=changer[2] {
                            println!(" {}  {} * {} = {} {}",currency[2],euro,changer[2],changer[2] as f32*0.5,euro);
                            
                        } else {
                            println!("Roll bank transaction .");
                            println!("We are out of some coins");
                            continue;
                        }
                        
                    }
                    change_dec = change_dec - (changer[2]*50);
                    changer[3] = change_dec  /20;
                    if changer[3] > 0 {
                        if bank[3]>=changer[3] {
                            println!(" {}  {} * {} = {} {}",currency[3],euro,changer[3],changer[3] as f32*0.2,euro);
                            
                        } else {
                            println!("Roll bank transaction .");
                            println!("We are out of some coins");
                            continue;
                        }
                        
                    }
                    change_dec = change_dec - (changer[3]*20);
                    changer[4] = change_dec  /10;
                    if changer[4] > 0 {
                        if bank[4]>=changer[4] {
                            println!(" {}  {} * {} = {} {}",currency[4],euro,changer[4],changer[4] as f32*0.1,euro);
                            
                        } else {
                            println!("Roll bank transaction .");
                            println!("We are out of some coins");
                            continue;
                        }
                        
                    }
                    change_dec = change_dec - (changer[4]*10);
                    changer[5] = change_dec  /5;
                    if changer[5] > 0 {
                        if bank[5]>=changer[5] {
                            println!(" {} {} * {} = {} {}",currency[5],euro,changer[5],changer[5] as f32*0.05,euro);
                            
                        } else {
                            println!("Roll bank transaction .");
                            println!("We are out of some coins");
                            continue;
                        }
                        
                    }
                    change_dec = change_dec - (changer[5]*5);
                    changer[6] = change_dec  /2;
                    if changer[6] > 0 {
                        if bank[6]>=changer[6] {
                            println!(" {} {} * {} = {} {}",currency[6],euro,changer[6],changer[6] as f32*0.02,euro);
                            
                        } else {
                            println!("Roll bank transaction .");
                            println!("We are out of some coins");
                            continue;
                        }
                        
                    }
                    changer[7] = change_dec - (changer[6]*2);
                    if changer[7] > 0 {
                        if bank[7]>=changer[7] {
                            println!(" {} {} * {} = {} {}",currency[7],euro,changer[7],changer[7] as f32*0.01,euro);
                            
                        } else {
                            println!("Roll bank transaction .");
                            println!("We are out of some coins");
                            continue;
                        }
                        
                    }
                    println!("Thank you ! Here is your change {} {}",change,euro);

                }
                
                for lop in 0..bank.len() {
                    bank[lop]=bank[lop]-changer[lop];        
                    // println!("{} {}  {} ",currency[lop],bank[lop],changer[lop]);
                }

            } else {
                println!("Thank you !");
            }
            continue
        }
    }

    //end of program
    let mut output = File::create(file)?;
    println!("");
    println!("");
    println!("Writing Cash Box fill level to File");
    for lop in 0..bank.len() {
        write!(output, "{}\n",bank[lop].to_string())?;
    }

    Ok(())
}
