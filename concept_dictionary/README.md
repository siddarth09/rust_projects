# BASIC NOTES For RUST

1. Variable and mutability 

Every variable in rust programming is immutable meaning they cannot be changed,but with keyword "mut" we can make them mutable. This offers easy adapatability just like python, and also gives you security and reusability of the variables in the program. 


2. Constants

constants are variables that are declared with the keyword "const". 
>Note : constants are written in all caps and with underscores betweem them 
example : const THRUST_TO_LIFT=9.8*45;


3. Shadowing 

Shadowing is a concept of rust programming, where variables can be changes and used in any part of the program. 

example : 
let x = 6;
let x = "   ";


in the first line x is a integer constant but in the second statement it becomes a string constant. Like python the variables are not given any datatype but can be used as any datatype in the program. If they have to have a constant datatype ":" can be used

example :

let x:f64=8.0;

here we are telling x is a float 64 datatype. 



4. Data Types in Rust

| Length   | Signed | Unsigned |
|----------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

         
incase you do not have an idea on what data type to use, you can always stick with 32 bit format and later increase it to 64 of 128. 

>Note: The default float type is f64 but its the same speed as f32 with more precision.


### COMPOUND DATATYPES 

rust has two compund datatypes :
a.** Tuple **: A fixed length collection of values that can be of different types.

Syntax: let tup: (type1, type2, type3, ...) = (value1,val2,val3......);


example: let tup:(i32,f64,bool)=(500,8.6,true)


b. **Array**: a homogenous collection of data which is of same data type. Arrays in rust have a fixed length. 

Syntax: let arr: [type; length] = [value1, value2, value3]

example : let arr:[i32;5]=[1,2,3,4,5]

### Functions 

functions come with default value in rust, the main() function is the main entry point for all the programs. 

snakecase conventional style for function and variable names is used. 

syntax : 

func <function name> (Params){
    <expressions or statements>
}

for function with return values. 

func <function name> (Params) -> <return type>
    <expressions/statements>
    <return>

    