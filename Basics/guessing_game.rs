// a simple program for guessing a number and returning it as an output
use std::io; // including the io functionaity from the standard library

fn main() //entry point into the program. declare a new function

    {  // no parameters and start the body of the function
        println!("Guess the number! ");

        println!("Please input your guess.");

        let mut guess = String::new(); // create a variable to store user input
                                       // making the variable mutable
                                       // string variable

        io::stdin() //
            .read_line(&mut guess) // the & shows that the argument is a reference
                                   // immutable by default 
            .expect("Failed to read line");

        println!("You guessed: {guess}"); // {} is a place holder for printing stuff
    }
