use rand::Rng;
use std::io;
fn main() {
    println!("Roll me {}", "daddy");
    println!("Type roll to roll the dice");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let num = rand::thread_rng().gen_range(1..7);
    println!("You rolled: {}", num)
}
