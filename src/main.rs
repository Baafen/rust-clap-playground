use clap::Parser;
use std::io::{
        self,
        Write
};

mod cmd_path;
mod cli;

use cmd_path::CmdPath;
use cli::Command;

fn build_cmd<'a>(line: &'a [String], path: &'a CmdPath) -> Vec<&'a str> {
        let mut output: Vec<&str> = Vec::new();

        if line.is_empty() {
            return output
        }

        output.push(line[0].as_str());

        // Include path when relevant
        match line[0].as_str() {
            "set" => output.extend(path.get_path()),
            "edit" => {
                if line.len() > 1 {
                    output.extend(path.get_path());
                }
            },
            _ => (),
        }

        output.extend(line.iter().skip(1).map(String::as_str));

        output
}

fn main() {
    let mut cmd_path = CmdPath::new();

    loop {
        match cmd_path.path.len() {
            0 => print!("> "),
            _ => print!("{cmd_path} > "),
        }

        io::stdout().flush().expect("Error flushing stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading line");
        let input_vec = shlex::split(input.trim()).expect("Error splitting command");

        let command = match Command::try_parse_from(build_cmd(&input_vec, &cmd_path)) {
            Ok(command) => command,
            Err(error) => {
                println!("{error}");
                continue;
            }
        };

        match command {
            Command::Set(set_command) => println!("set {set_command:?}"),
            // todo: handle path switch to support ..
            Command::Edit { path } => cmd_path.path = path,
            Command::Exit => break,
        }
    }
}
