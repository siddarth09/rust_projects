fn fibonacci(term:i32){
    let mut n = term; 
    let mut first = 0 ;
    let mut second = 1;
    

    for i in 0..n{
        if i<=1{
            println!("{}",i)
        }
        else{
            let next = first + second;
            println!("{}",next);
            first = second;
            second=next;
        }

        
    }

    println!("\n");

    
}

mod test;
fn main(){

    test::test();
    
    let mut num_terms=String::new();
    println!("Enter the number of terms you want to see in the Fibonacci sequence:");
    std::io::stdin().read_line(&mut num_terms).expect("Enter just one term");
    let num_terms:i32=num_terms.trim().parse().expect("LOL");

    fibonacci(num_terms);
}