#[macro_use]
extern crate log;
extern crate simple_logging;

mod command;

use command::{Command, CommandIt};
use std::io::{self, Write};

fn main() {
    simple_logging::log_to_file("luci.log", log::LevelFilter::Debug).unwrap();
    info!("Start");

    let commands = io::stdin();
    let commands = CommandIt::new(commands.lock());

    for cmd in commands.take_while(|&c| c != Command::Quit) {
        debug!("{:?}", cmd);

        io::stdout().write(cmd.answer().as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }
}
