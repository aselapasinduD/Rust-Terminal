use std::env;
use std::path::Path;
use std::io::{self};

const SHELL_COMMANDS: &[&str] = &["cd", "help"];

pub fn handle_help() {
    println!("List of available commands:");
    println!("  cd <directory path> - Change the directory.");
    println!("  exit - exit form the terminal.");
}

pub fn handle_cd(args: &[&str]) -> io::Result<()>{
    if args.is_empty(){
        eprintln!("cd: Missing Operand");
    } else {
        let new_path = Path::new(args[0]);
        if let Err(e) = env::set_current_dir(&new_path){
            eprintln!("cd: {}", e);
        }
    }
    Ok(())
}

pub fn is_command_in_shell(command: &str) -> bool {
    SHELL_COMMANDS.contains(&command)
}

pub fn handle_shell_commands(command: &str, args: &[&str]){
    match command{
        "cd" => {
            if let Err(e) = handle_cd(args) {
                eprintln!("Error cd: {}", e);
            }
        }
        "help" => handle_help(),
        _ => eprintln!("Unknown Command: {}", command)
    }
}