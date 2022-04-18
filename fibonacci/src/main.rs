use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = input
                        .trim()
                        .parse()
                        .expect("Please type a number!");

    let result = fibonacci(input);

    println!("The {}th fibonacci number is {}", input, result);
}

fn fibonacci(number: i32) -> i32 {
    if number == 0 {
        0
    } else if number == 1 {
        1     
    } else {
        fibonacci(number-1) + fibonacci(number-2)
    }
}