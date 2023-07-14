use clap::Parser;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
struct Args {
    #[clap(num_args = 1..)]
    command: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let command = args.command
        .get(0)
        .expect("Invalid command length of 0.");
    let mut screensaver_command = Command::new(command);
    for arg in args.command.iter().skip(1) {
        screensaver_command.arg(arg);
    }
    screensaver_command.stdout(Stdio::inherit())
        .output()
        .expect("Failed to run screensaver command");
}
