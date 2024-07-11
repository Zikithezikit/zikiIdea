use std::fs;
use std::fs::File;
use std::io::{stdin, stdout, Write};

fn main() {
    println!("> Enter the idea: ");
    stdout().flush().unwrap(); // Flush the output buffer to display the prompt immediately

    let mut the_idea = String::new();
    stdin().read_line(&mut the_idea).unwrap();
    println!("{}", the_idea.trim_end());

    // Create a new file (or overwrite existing one)
    let mut file = File::create("output.txt");

    fs::write("output.txt", the_idea.trim_end());

    println!("Data written to output.txt successfully!");
}
