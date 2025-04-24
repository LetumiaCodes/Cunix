use std::{process::Command, thread, time::Duration};
use std::io::{self, Write};

fn main() {
    let message = "Welcome to Cunix. Use Command Help for a list of commands";
    let width = message.len() + 4;

    if Command::new("clear").status().is_err() {
        eprintln!("Warning: Failed to clear screen");
    }

    println!("Booting into Cunix");
    thread::sleep(Duration::new(5, 0));

    if Command::new("clear").status().is_err() {
        eprintln!("Warning: Failed to clear screen");
    }

    println!("Successfully booted into Cunix");
    thread::sleep(Duration::new(2, 0));

    if Command::new("clear").status().is_err() {
        eprintln!("Warning: Failed to clear screen");
    }

    println!("{}", "+".repeat(width));
    println!("| {} |", message);
    println!("{}", "+".repeat(width));

    loop {
        print!("User# ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes = io::stdin().read_line(&mut input).unwrap();

        if bytes == 0 {
            println!("EOF or no input detected. Exiting...");
            break;
        }

        let command = input.trim().to_lowercase();
        println!();

        match command.as_str() {
            "sysi" => println!("Cunix v0.1.0"),
            "clear" => {
                if Command::new("clear").status().is_err() {
                    eprintln!("Warning: Failed to clear screen");
                }
            }
            "shutdown" => {
                println!("Shutting down...");
                break;
            }
            "help" => {
                println!("Available commands:");
                println!("  sysi     - Show system info");
                println!("  clear    - Clear the screen");
                println!("  shutdown - Exit Cunix");
                println!("  cd       - Change directory");
                println!("  sf       - List all files and folders in dirrectory");
                println!("  ./       - Execute .rt application");
            }
            _ => println!("Unknown command: {}", command),
        }

        println!();
    }
}
