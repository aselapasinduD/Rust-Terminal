extern crate winapi;

use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;
use winapi::um::consoleapi::AllocConsole;
use std::path::Path;

mod shell_commands;

fn main() {
    unsafe {
        AllocConsole();
    }
    loop{
        match env::current_dir() {
            Ok(path) => print!("{}", path.display()),
            Err(e) => eprintln!("Error Getting Current Directory: {}", e)
        }
        print!(" >");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let mut parts = input.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };
        let args: Vec<&str> = parts.collect();

        if command == "cd" {
            if args.is_empty() {
                eprintln!("cd: missing operand");
            } else {
                let new_path = Path::new(args[0]);
                if let Err(e) = env::set_current_dir(&new_path){
                    println!("cd: {}", e);
                }
            }
            continue;
        }

        let output = Command::new(command).args(&args).stdout(Stdio::piped()).output();

        match output {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
            }
        }
    }
}
