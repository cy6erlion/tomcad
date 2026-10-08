use std::env;
use tomcad::start;

fn main() {
    let cmd_arg = 1;
    let input_arg = 2;

    if let Some(command) = &env::args().nth(cmd_arg) {
        match command.as_ref() {
            "start" => {
                if let Some(endpoints) = &env::args().nth(input_arg) {
                    match start(endpoints) {
                        Ok(_) => std::process::exit(0),
                        Err(e) => panic!("{e}"),
                    }
                } else {
                    panic!("Invalid endpoints file path")
                }
            }
            _ => panic!("Invalid command"),
        }
    } else {
        panic!("Invalid command")
    }
}
