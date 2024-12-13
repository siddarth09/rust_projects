use std::io;
use rand::Rng;

fn main(){
    // let  x = get_value();
    // let  sum = 4+10; 
    // println!("Sum is: {}", sum);
    // const P_I:f64=3.14; 

    // let differ= 9.44 - P_I-x;
    // println!("Difference:{}",differ);

    // //division

    // let quotient = x / P_I;
    // let turnacated = -5/3; 
    // println!("Quotient:{}",quotient);
    // println!("Turnacated:{}",turnacated);


    // let tup:(i32,f64,bool)=(500,5.6,false);
    // println!("Tup:({},{},{})",tup.0,tup.1,tup.2);
    temperature_converter(true)
    

}


fn get_value()->f64{

    let mut value = String::new();

    println!("Enter a value");
    std::io::stdin().read_line(&mut value).expect("no input");
    let value:f64 = value.trim().parse().expect("Please type floating point");
    return value;
}

fn temperature_converter(celcius:bool){

    let x:f64 = get_value();
    if celcius == true{
        let fahrenheit = (x*9.0/5.0)+32.0;
        println!("Fahrenheit:{}",fahrenheit);
    }
    else {
        let fahrenheit = (x-32.0)*5.0/9.0;
        println!("Celcius:{}",fahrenheit);
    }

}
// fn guess_game()->bool{

//     let x = get_value();

//     let secret_no=rand::thread_rng().gen_range(1..=10);

//     if x == secret_no{
//         println!("You guessed it!");
//         return true;   
//     }
//     else if x>secret_no{
//         println!("Too big!");
//         return false;
//     }
//     else{
//         println!("Too small!");
//         return false;
//     }
    

// }

