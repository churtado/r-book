// bring this lib into scope
use std::io;
// Rng is a trait. It defines methods that random number generators implement
// Rng has to be in scope to use the generators
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // println! is a macro
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Input guess");

        // let creates variables
        // mut means mutable
        // equality in rust means binding, thus bind guess to whatever value is on the right
        // the right hand side is a function that returns a new instance of a string type
        // :: means that the function is associated to the type, thus an associated function of the String type
        // new creates something new and empty
        let mut guess = String::new();

        // calling the stdin() function from the io module
        // function returns instance of std::io:Stdin, a handle to standard in
        // read_line calls the read_line method on the handle to get input
        // read_line appends user input into a string without overwriting
        // read_line's argument needs to be mutable to change the the string's content
        // & means it's a reference to the memory location. Ref's are immutable by default
        // read_line appends to a string, but it also returns a value of type io::Result
        // Result is an enumeration with 2 variants: Ok and Err.
        // The result type has methods attached, one is expect. If err calls expect then you get back the OS error
        // if it's Ok type, in this case, you get back the number of bytes the user input. This return varies depending on the caller
        // without expect the compiler issues a warning
        io::stdin().read_line(&mut guess).expect("Failed to read");

        // convert the string guess into a number
        // we create a variable named guess that shadows the previous value
        // this is called shadowing, converting types without having to create a new variable with a different name
        // the parse method tries to parse to any type of number, but we force it to u32 with the type annotation
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {} represents the value to be printed
        println!("you guessed: {}", guess);

        // Ordering is an enum, below are its variants
        // a match has arms. This match has 3 arms
        // the arm that runs is the one that matches based on the cmp function
        // like pattern matching in Erlang and Scala before it
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
