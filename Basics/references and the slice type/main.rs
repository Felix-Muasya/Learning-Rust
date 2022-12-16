// references in rust

fn main() {
    let mut string1 = String::from("this is a string");

    let references1 = &string1;
    let references2 = &string1;
    println!("{}, {}", references1, references2);

    let len = calculate_length(&string1); // reference using &
                                     // no need to return ownership since we never had any
                                     // references are immutable by defaault.
                                     // add mut to make a value mutable

    println!("the length of '{}' is {}", string1, len);


    add_to_string( &mut string1);
    println!("{string1}");

    

    



}


fn calculate_length(s: &String) -> usize { 

    s.len() // s is a reference to a string

}

fn add_to_string(s: &mut String ) 
{
    s.push_str(" and it's mutable");

}