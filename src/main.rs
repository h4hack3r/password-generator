use std::io;
use rand::seq::SliceRandom;
use rand::Rng;
fn main() {
    let letters:Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let numbers:Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let symbols:Vec<char> = vec!['!', '#', '$', '%', '&', '(', ')', '*', '+'];

    println!("Welcome to the Password Generator!");
    let mut in_letters = String::new();
    println!("How many letters would you like in your password?");
    io::stdin().read_line(&mut in_letters).expect("Failed to read line");
    let nr_letters:u32 = in_letters.trim().parse().expect("Please type a number!");

    let mut in_symbols = String::new();
    println!("How many symbols in your password?");
    io::stdin().read_line(&mut in_symbols).expect("Failed to read line");
    let nr_symbols:u32 = in_symbols.trim().parse().expect("Please type a number!");


    let mut in_numbers = String::new();
    println!("How many numbers in your password?");
    io::stdin().read_line(&mut in_numbers).expect("Failed to read line");
    let nr_numbers:u32 = in_numbers.trim().parse().expect("Please type a number!");

    let mut rng = rand::thread_rng();

    let mut password = String::new();
    for _n in 0..nr_letters {
        password.push(letters[rng.gen_range(0..letters.len())]);
    }
    for _n in 0..nr_symbols{
        password.push(symbols[rng.gen_range(0..symbols.len())]);
    }
    for _n in 0..nr_numbers {
        password.push(numbers[rng.gen_range(0..numbers.len())]);
    }
    

    let vector: Vec<char> = password.chars().collect();
    let mut shuffled_vector = vector.clone();
    let mut rng = rand::thread_rng();
    shuffled_vector.shuffle(&mut rng);
    let shuffled_password: String = shuffled_vector.into_iter().collect();

    println!("Your password is:  {}",shuffled_password);
}
