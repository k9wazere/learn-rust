use std::io; // chapter 2, rust guessing game

fn chapter_2_guess() {
    println!("Guess the number, fool!");
    println!("Type it here: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("You have failed me for the last time, player.");

    println!("You gave me {}", guess);
}

fn main() {
    chapter_2_guess();
}