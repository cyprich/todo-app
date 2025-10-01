use std::{
    io::{self, Write},
    vec,
};

const DONE_MARK: char = 'x';

struct Item {
    name: String,
    done: bool,
}

enum Command {
    Add(String),
    Remove(Option<usize>),
    List,
    Done(Option<usize>),
    Undone(Option<usize>),
    Help,
    Exit,
    Unknown,
}

fn print_help() {
    println!(
        "TODO app

Usage: 
    list            list items
    add <name>      add new item with given <name>
    remove <index>  remove item at <index>
    done <index>    mark item at <index> as done
    undone <index>  mark item at <index> as undone
    help            display this help screen
    exit            exit program

You can also use just first letter of the command, like 'h' for help or 'l' for list
"
    )
}

fn get_command() -> String {
    let mut user_input = String::new();

    print!("\nEnter command: ");
    io::stdout().flush().unwrap(); // prints to console immediately

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read from user...");

    user_input.trim().to_lowercase()
}

fn execute_command_by_type(command: &Command, list: &mut Vec<Item>) -> bool {
    match command {
        Command::Add(x) => list.push(Item {
            name: x.clone(),
            done: false,
        }),
        Command::Remove(x) => {
            #[allow(unused)] // disable warning for unused variable
            match *x {
                Some(val) => {
                    list.remove(val - 1);
                }
                None => print_help(),
            };
        }
        Command::List => {
            println!("TODO list: ");
            let mut n = 1;
            for i in list {
                println!(
                    "  {n}. [{}] {}",
                    if i.done { DONE_MARK } else { ' ' },
                    i.name
                );
                n += 1;
            }
        }
        Command::Done(x) => match x {
            Some(val) => {
                let i = list.get_mut(val - 1);
                match i {
                    Some(val) => val.done = true,
                    None => println!("Couldn't access item number {val}"),
                }
            }
            None => print_help(),
        },
        Command::Undone(x) => match x {
            Some(val) => {
                let i = list.get_mut(val - 1);
                match i {
                    Some(val) => val.done = false,
                    None => println!("Couldn't access item number {val}"),
                }
            }
            None => print_help(),
        },
        Command::Help => print_help(),
        Command::Exit => return true,
        Command::Unknown => {
            println!("\nI don't know this command yet!\nType \"help\" to see usage");
        }
    };

    false
}

fn execute_command_by_name(command: &String, list: &mut Vec<Item>) -> bool {
    let mut iter = command.splitn(2, ' '); // split in max 2 parts
    let command: char = iter
        .next()
        .unwrap_or_default()
        .chars()
        .next()
        .unwrap_or_default(); // fist letter of command
    let arg = iter.next().unwrap_or_default();

    // no arguments
    if command == 'l' {
        execute_command_by_type(&Command::List, list)
    } else if command == 'h' {
        execute_command_by_type(&Command::Help, list)
    } else if command == 'e' {
        execute_command_by_type(&Command::Exit, list)
    } else {
        // string argument
        if command == 'a' {
            let arg = if arg.is_empty() { "<empty>" } else { arg };
            execute_command_by_type(&Command::Add(String::from(arg)), list)
        } else {
            // usize arguments
            let arg: Option<usize> = if arg.len() > 0 {
                Some(arg.parse().unwrap_or_default())
            } else {
                None
            };

            if command == 'r' {
                execute_command_by_type(&Command::Remove(arg), list)
            } else if command == 'd' {
                execute_command_by_type(&Command::Done(arg), list)
            } else if command == 'u' {
                execute_command_by_type(&Command::Undone(arg), list)
            } else {
                // unknown command
                execute_command_by_type(&Command::Unknown, list)
            }
        }
    }
}

fn main() {
    let mut list: Vec<Item> = vec![];

    loop {
        let user_input = get_command();
        if execute_command_by_name(&user_input, &mut list) {
            break;
        }
    }
}
