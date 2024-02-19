use std::io;
use speakit_rust::*; 

fn main() {
    let filename = "data.txt";
    let lines = match read_file(filename) {
        Ok(lines) => {
            println!("File read successfully");
            lines
        }
        Err(err) => {
            eprintln!("Error reading file {}: {}", filename, err);
            return;
        }
    };

    let freq = count_frequency(&lines);
    let num_words_to_print = 15;

    loop {
        println!("\nType a substring or press q to quit:");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read user input");
        let user_input = user_input.trim().to_lowercase();
        if user_input == "q" {
            println!("Game over");
            break;
        }
        let matching_words = get_matching_words(&freq, &user_input);
        if !matching_words.is_empty() {
            for word in matching_words.iter().take(num_words_to_print) {
                println!("{}", word);
            }
        } else {
            println!("Substring {} not found in any word", user_input);
        }
    }
}

