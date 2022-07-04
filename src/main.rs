use std::io::{self, Write};

mod blackjack;

fn main() {
    let text = "Welcome to Blackjack! Choose one of the options below:\nq: quit game, p: play";
    println!("{}", text);

    loop {
        let mut choice = String::new();
        print!(">> ");
        io::stdout().flush()
            .expect("Failed to flush to console");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
    
        // Not working because read_line adds a new line to the string
        match choice.as_str() {
            "q" => return,
            "p" => break,
            d => println!("The choice, '{}', isn't an option. Pleaase choose one of the options below:\nq: quit game, p: play", d)
        }
    }

    let deck = blackjack::generate_deck();
    println!("{}", deck.len());
}

