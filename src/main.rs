#[macro_use]
extern crate log;
extern crate simple_logging;

use std::io::{self, Write};

#[derive(Debug)]
enum Command {
    Quit,
    Unknown,
    Uci,
    IsReady,
}

impl Command {
    fn from_str(command: &str) -> Command{
        match command.trim(){
            "quit" => Command::Quit,
            "uci" => Command::Uci,
            "isready" => Command::IsReady,
            other => {
                warn!("Unkown Command: {}", other);
                Command::Unknown
            }
        }
    }

    fn is_quit(&self) -> bool{
        match *self {
            Command::Quit => true,
            _ => false
        }
    }

    fn answer(&self) -> &'static str{
        match *self{
            Command::Quit => "",
            Command::Unknown => "",
            Command::Uci => "id name Luci\nid author Gunnar Klaemke, Markus Klein\nuciok\n",
            Command::IsReady => "readyok\n"
        }
    }
}

fn main() {
    simple_logging::log_to_file("luci.log", log::LevelFilter::Debug).unwrap();
    info!("Start");
    let mut command = String::new();
    loop {
        command.clear();
        match io::stdin().read_line(&mut command){
            Ok(_size) => {
                let command = Command::from_str(&command);
                debug!("{:?}", command);
                if command.is_quit(){
                    break;
                }
               
                io::stdout().write(command.answer().as_bytes()).unwrap();
                io::stdout().flush().unwrap();             
            },
            Err(e) => { 
                error!("Error reading command:{}", e);
                break; 
            }
        }
    }

}
