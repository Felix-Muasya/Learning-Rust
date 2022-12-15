fn main() 
{

    let _string = "String literal"; // immutable string

    let mut string_1 = String::from("mutable ");

    string_1.push_str("string literal"); // append to string_1


    let s1 = String::from("Another string");

    let _s2 = s1.clone(); // deep copy of the data

    println!("This is a  {string_1}, but this {_string} is immutable. \n");



    ////////////////////////////////////////////////////////
    // Functions and Ownership //
    ////////////////////////////////////////////////////////

    let string_2 = String::from("Phonk music is nice");

    takes_ownership(string_2); // to demonstrate ownership, 
                               //can't call takes_ownership() b4 declaring string_2

    let integer = 32;

    makes_copy(integer);


    /////////////////////////////////////////////////////
    // Return values and scope //
    /////////////////////////////////////////////////////

    let _a_rand_string = String::from("A random string");

    

    let _the_string = String::from("the string in question");


    //////////////////////////////////////////////////
    // returning ownership of parameters //
    ////////////////////////////////////////////////
    let our_string = String::from("Our String");
    let (our_string_1, len) = calculate_length(our_string);

    println!("The length of '{}' is {}", our_string_1, len );

}


fn takes_ownership(a_string: String)
{
    println!("{}", a_string);

}


fn makes_copy(an_integer: i32)
{
    println!("{}", an_integer);
}


// the function below will give its return value to the function that calls it
fn _gives_ownership() -> String
{
    let a_rand_string = String::from("Yours now");

    a_rand_string
    
}


// the function below takes a string and returns one
fn _takes_ownership_and_give_it_back(the_string: String) -> String
{
    the_string
    
}


// return ownership of parameters
fn calculate_length(s: String) -> (String, usize)
{
    let length = s.len(); // return the length of our string

    (s, length)
}
