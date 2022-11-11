// rand = "0.8.3"
// https://doc.rust-lang.org/book/ch03-02-data-types.html

// use rand::Rng;
use std::io;

fn main() {
    println!("Task3 : ");
    task3();
}

fn task3(){
    // let random_price : f32 = rand::thread_rng().gen_range(1..=10);
    loop {
        let euro = '\u{20AC}';
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
        println!(" ");
        println!("You Product Number : {} ",product_choice);
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
                println!("Thank you ! Here is your change {} {}",change,euro);
                let currency: [f32; 8] = [2.0, 1.0, 0.5,0.2, 0.1, 0.05, 0.02,0.01];
                let change_int :u32 = change as u32;
                
                let change_dec :f32 = change - change_int as f32;
                let mut change_dec :u32 = (change_dec* 100.0) as u32;
                // let mut change_dec :f32 = change_dec as f32/ 100.0;
                // let change_int :f32 = change_int as f32;
                if change_int >0 {
                    let changer2e = change_int/2;
                    let changer1e = change_int-(changer2e*2);

                    if changer2e > 0 {
                        println!(" {}    {} * {} = {} {}",currency[0],euro,changer2e,changer2e*2,euro);
                    }
                    if changer1e > 0 {
                        println!(" {}    {} * {} = {} {}",currency[1],euro,changer1e,changer1e*1,euro);
                    }
                }
                // println!(" decimal {}  ",change_dec);
                if change_dec > 0 {
                    let changer05 = change_dec/50;
                    if changer05 > 0 {
                        println!(" {}  {} * {} = {} {}",currency[2],euro,changer05,changer05 as f32*0.5,euro);
                    }
                    change_dec = change_dec - (changer05*50);
                    let changer02 = change_dec  /20;
                    if changer02 > 0 {
                        println!(" {}  {} * {} = {} {}",currency[3],euro,changer02,changer02 as f32*0.2,euro);
                    }
                    change_dec = change_dec - (changer02*20);
                    let changer01 = change_dec  /10;
                    if changer01 > 0 {
                        println!(" {}  {} * {} = {} {}",currency[4],euro,changer01,changer01 as f32*0.1,euro);
                    }
                    change_dec = change_dec - (changer01*10);
                    let changer005 = change_dec  /5;
                    if changer005 > 0 {
                        println!(" {} {} * {} = {} {}",currency[5],euro,changer005,changer005 as f32*0.05,euro);
                    }
                    change_dec = change_dec - (changer005*5);
                    let changer002 = change_dec  /2;
                    if changer002 > 0 {
                        println!(" {} {} * {} = {} {}",currency[6],euro,changer002,changer002 as f32*0.02,euro);
                    }
                    let changer001 = change_dec - (changer002*2);
                    if changer001 > 0 {
                        println!(" {} {} * {} = {} {}",currency[7],euro,changer001,changer001 as f32*0.01,euro);
                    }

                }
                
            } else {
                println!("Thank you !");
            }
            continue
        }
    }
}
