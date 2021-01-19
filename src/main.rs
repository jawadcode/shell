use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;
use std::str::SplitWhitespace;
use whoami;

fn main() {
    println!("This is a shell written in rust");
    loop {
        // Prompt and flush stdout in preparation for stdin
        let username = whoami::username();
        let hostname = whoami::hostname();
        let dir = env::current_dir().unwrap();
        print!("{}@{} {}\n$ ", username, hostname, dir.display());
        stdout().flush().expect("Failed to flush stdout :(");

        // Take in user input
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input :(");

        // Split input into command and args
        let mut input = input.trim().split_whitespace(); // Shadows String with SplitWhitespace Iterator
        let cmd = input.next().unwrap();
        match cmd {
            "cd" => cd(input),
            "exit" => break,
            cmd => run_cmd(cmd, input),
        }
    }
}

fn cd(mut args: SplitWhitespace) {
    // The next arg should be the path, if it does not exist then use "/" in it's place
    let target = args.next().unwrap_or("/");
    // Convert &str to Path and then set the current directory to that path (and check for errors)
    let path = Path::new(target);
    if let Err(error) = env::set_current_dir(path) {
        eprintln!("{}", error);
    } else {
        println!("Changed directory to '{}'", target);
    }
}

fn run_cmd(cmd: &str, args: SplitWhitespace) {
    // Spawn command as child-process of the shell
    let child = Command::new(cmd).args(args).spawn();
    // Error checking for child
    match child {
        Ok(mut child) => {
            if let Err(error) = child.wait() {
                eprintln!("{}", error);
            }
        }
        Err(error) => eprintln!("{}", error),
    }
}
