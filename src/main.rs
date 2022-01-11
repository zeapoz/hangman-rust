use std::io::{self, BufReader, BufRead};
use std::fs::File;
use rand::seq::SliceRandom;

fn main() {
    // let secret_word = String::from("rust");
    let secret_word = choose_word();
    let mut guessed = Vec::new();

    println!("\n-----------------------");
    println!("| Welcome to Hangman! |");
    println!("-----------------------\n");

    println!("The secret word is: {}", secret_word);

    loop {
        // Check if player has won
        if check_win(&secret_word, &guessed) {
            println!("Congratulations! The secret word was: {}", secret_word);
            break;
        }

        // Print the obfuscated secret word
        print_revealed(&secret_word, &guessed);

        // Get user input, in case of error try again
        println!("Please enter a valid guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        let guess: char = match guess.trim().parse() {
            Ok(c) => c,
            Err(_) => continue,
        };
        
        // Check if character already has been guessed
        if guessed.contains(&guess) {
            println!("Already guessed {}!\n", &guess);
            continue;
        }
        guessed.push(guess);

        // Generate guessed characters string
        let mut guessed_str = String::new();
        for g in &guessed {
            guessed_str.push(*g);
        }
        println!("You have guessed: {}\n", guessed_str);
    }
}

fn read_words() -> Vec<String> {
    let file = File::open("./src/words.txt").unwrap();
    let reader = BufReader::new(file);
    let mut words = Vec::new();
    for line in reader.lines() {
        let word = line
            .unwrap()
            .trim()
            .to_string();
        words.push(word);
    }
    words
}

fn choose_word() -> String {
    let words = read_words();
    let mut rng = rand::thread_rng();
    let chosen = words.choose(&mut rng).unwrap();
    chosen.to_string()
}

fn check_win(secret_word: &String, guessed: &Vec<char>) -> bool {
    for c in secret_word.chars() {
        if !guessed.contains(&c) {
            return false
        }
    } 
    true
}

fn print_revealed(secret_word: &String, guessed: &Vec<char>) {
    let mut revealed = String::new();
    for c in secret_word.chars() {
        if guessed.contains(&c) {
            revealed.push(c);
            revealed += " ";
        } else {
            revealed += "_ ";
        }
    }
    println!("{}", revealed);
}
