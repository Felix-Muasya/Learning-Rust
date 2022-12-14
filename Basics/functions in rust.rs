fn main() 
{
    println!("Hello, world!");

    another_function(5, 'h');

    let x = plus_one(7);
    println!("{x}");
}

fn another_function(value: i32, unit_label: char) 
{
    println!("Another function, that prints out {value} and {unit_label}");
}

fn plus_one(x: i32) -> i32{
    x + 1 // no semicolon
}
