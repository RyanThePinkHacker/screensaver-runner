use clap::Parser;
use device_query::{DeviceState, DeviceQuery};
use std::{process::{Command, Stdio}, thread};

#[derive(Parser, Debug)]
struct Args {
    #[clap(num_args = 1..)]
    command: Vec<String>,
}

fn screensaver_runner(args: Args) {
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

fn main() {
    let args = Args::parse();
    let screensaver_handle = thread::spawn(move || screensaver_runner(args));

    let device_state = DeviceState::new();
    let inital_mouse_position = device_state.get_mouse().coords;
    loop {
        // Exit if screensaver is done running
        if screensaver_handle.is_finished() {
            break;
        }

        let mouse = device_state.get_mouse();
        if inital_mouse_position != mouse.coords {
            break;
        }
    }
}
