fn main() 
{
    let x = 30; // immutable varibale
    let y = 17; // immutable variable

    println!("{}", x*y); // string literal 

    const SAVINGS: u32 = 1000; // constant 

    println!("{}", SAVINGS); // will print out an error if I don't use it

// shadowing allows you to create a new varible with an existing name
    let x = "six"; 
    println!("the value of x is {}", x);

// scalar datatypes represent a single value    
    // they include integers, floating point numbers, boolean and characters

    let _a = 98_222; // a is unused decimal, prefix with '_' to avoid errors
    let _b = 0xff; // hex
    let _c = 0o77; // Octal
    let _d = 0b1111_0000; // binary
    let _e = b'A'; // Byte
    let _f = 255;


// compound datatypes represent a group of values
    // include tuples, 
    let tup = ("Learning rust is tricky ", 100_000); /* (&str, i32)
    we can get values out of tuples by destructuring or dot notation */
    
    let (_string, _integer) =  tup ; // destructuring
    let _number = tup.1; // dot notation

    let _sql_error_code = [152, 168, 119]; // array
    let _syntax_error = _sql_error_code[1]; // element of array

    let _byte = [0;8];
 

     // functions
    let _sum = _my_function(11, 13);
    println!("the sum is {}", _sum);
     

} 


fn _my_function(neet: i32, meme:i32) -> i32 // specifying a return type
{
    println!("Another function that show the values of neet as {}", neet);
    println!("Another function that show the values of meme as {}", meme);

    neet + meme 
} 
