use std::io;
// use colored::*;
#[path = "days/1/solution_a.rs"] mod solution1_a;
// #[path = "days/1/solution_b.rs"] mod solution1_b;

// HOW DO YOU MAKE THIS SHIT COLOURED AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA

fn main() {
    println!("\nKortimu's Epic Advent of Code Solver (KEAoCS)\n");

    loop {
        println!("Select the day you want to solve!\n(Do the day number followed by part A/B. For example, 4A or 7b work, but not 3, A or a5.)\nYou can quit by writing QUIT.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        // TODO: add a formatting check
        match guess.trim().to_lowercase().as_str() {
            "1a" => solution1_a::run(),
            // "1b" => solution1_b::run(),
            "quit" => std::process::exit(0),
            _ => println!("\nSorry, but that day is not covered or does not exist.\n"),
        }
    }
}