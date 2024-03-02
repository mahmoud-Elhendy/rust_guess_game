use std::io;
use rand::Rng;

fn main() {
    //generate num
    let mut tg = rand::thread_rng();
    let num: u8 = tg.gen_range(1..=100);
    let mut guess = String::new();

    loop {
        println!("Guess what:");
        io::stdin().read_line(&mut guess).expect("hhhhhhh");
        let guess_num: u8 = match guess.trim().parse::<u8>(){
            Ok(n) => n,
            Err(_) => {
                println!("enter a valid number");
                guess.clear();
                continue;
            },
        };
        
        guess.clear();
        if guess_num > num{
            println!("high");
        }
        else if guess_num < num {
            println!("low");
        }
        else {
            println!("Winner Winner");
            break;
        }
    }    
        
}
