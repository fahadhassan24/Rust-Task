// rand = "0.8.3"
// https://doc.rust-lang.org/book/ch03-02-data-types.html

// use rand::Rng;
use std::io;

fn main() {
    println!("Task4 : ");
    task4();
}

fn task4(){
    // let random_price : f32 = rand::thread_rng().gen_range(1..=10);
    let mut bank:     [u32; 8] = [20;8];
    loop {
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
        let product_choice :usize   ;
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        
        if user_input.trim()=="q" || user_input.trim()=="Q" {
            println!("Thank you !");
            break;
        }
        product_choice = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter Product # between 1  to 12 or q to exit");
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
                    println!("{} {}  {} ",currency[lop],bank[lop],changer[lop]);
                }

            } else {
                println!("Thank you !");
            }
            continue
        }
    }
}
