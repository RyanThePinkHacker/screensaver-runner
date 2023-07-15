use clap::Parser;
use device_query::{DeviceState, DeviceQuery};
use shlex::split;
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    command: String,
}

fn main() {
    let args = Args::parse();
    let command_split = split(&args.command)
        .expect("Failed to parse command");
    let command = command_split.get(0)
        .expect("Invalid command length of 0.");
    let mut screensaver_command = Command::new(command);
    for arg in command_split.iter().skip(1) {
        screensaver_command.arg(arg);
    }
    let mut screensaver_child = screensaver_command.spawn()
            .expect("Failed to run screensaver command.");

    let device_state = DeviceState::new();
    let inital_mouse_position = device_state.get_mouse().coords;
    loop {
        let mouse = device_state.get_mouse();
        if inital_mouse_position != mouse.coords {
            break;
        }
    }
    screensaver_child
        .kill()
        .expect("Failed to stop screensaver process");
}
