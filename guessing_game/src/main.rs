use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    let secret_no=rand::thread_rng().gen_range(1..=10);
    loop{

    
        println!("guess the number!");
        println!("ENTER YOUR GUESS");

        let mut guess=String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read");

        let guess :u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        println!("Your guess\t{} ",guess);

        match guess.cmp(&secret_no){
            Ordering::Less=>println!("Too small"),
            Ordering::Greater=>println!("Too big!"),
            Ordering::Equal=>{
                println!("You win!");
            break;
            }
        }
    }

}
