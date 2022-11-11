// rand = "0.8.3"
// https://doc.rust-lang.org/book/ch03-02-data-types.html

use rand::Rng;
use std::io;

fn main() {
    println!("Task 1 : ");
    task1();
}

fn task1(){
    // let random_price : f32 = rand::thread_rng().gen_range(1..=10);
    loop {
        let euro = '\u{20AC}';
        // println!("Euro : {}",euro);
        let random_price : f32 = rand::thread_rng().gen_range(1.00..=10.0);
        let random_price :u32 = (random_price* 100.0) as u32;
        let random_price :f32 = random_price as f32/ 100.0;
        println!("Product Price : {} {}",random_price,euro);
        
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
            } else {
                println!("Thank you !");
            }
            continue
        }
    }
}
