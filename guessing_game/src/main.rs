use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please Input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read");
    println!("You guessed: {}", guess);
}
