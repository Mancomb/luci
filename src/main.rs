#[macro_use]
extern crate log;
extern crate simple_logging;
extern crate pleco;
extern crate rand;

use std::io::{self, Write};
use pleco::{Board};
use rand::Rng;

#[derive(Debug)]
enum Command {
    Quit,
    Unknown,
    Uci,
    Position (Board),
    Go,
    IsReady,
}

impl Command {
    fn from_str(command: &str) -> Command{
        let v: Vec<&str> = command.trim().split(' ').collect();
        match v[0] {
            "quit" => Command::Quit,
            "uci" => Command::Uci,
            "isready" => Command::IsReady,
            "go" => Command::Go,
            "position" => {
                let mut board = Board::start_pos();
                if v[1] == "fen" {
                    panic!("fen not yet implemented");
                } else {
                    if v.len() > 2 {
                        for mv in v.into_iter().skip(2) {
                            board.apply_uci_move(mv);
                        }
                    }
                }
                Command::Position(board)
            }
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
    
}

struct Engine {
    chessboard: Board
}

impl Engine {
    fn new() -> Engine 
    {
        Engine{chessboard: Board::start_pos()}
    }

    fn process_command(&mut self, command: Command) -> std::string::String{
        match command{
            Command::Quit => std::string::String::new(),
            Command::Unknown => std::string::String::new(),
            Command::Position( board) => {
                self.chessboard = board;
               std::string::String::new() },
            Command::Go => {
                let moves = self.chessboard.generate_moves();
               
               if let Some(mv) = rand::thread_rng().choose(&moves) {
                   self.chessboard.apply_move(*mv);
                    std::string::String::from("bestmove ") + &mv.stringify() + "\n"
               } else {
                   std::string::String::new()
               }
            },
            Command::Uci => std::string::String::from("id name Luci\nid author Gunnar Klaemke, Markus Klein\nuciok\n"),
            Command::IsReady => std::string::String::from("readyok\n")
        }
    }
}

fn main() {
    simple_logging::log_to_file("luci.log", log::LevelFilter::Debug).unwrap();
    info!("Start");
    let mut command = String::new();
    let mut engine = Engine::new();
    loop {
        command.clear();
        match io::stdin().read_line(&mut command){
            Ok(_size) => {
                debug!("{:}", command);
                let command = Command::from_str(&command);
              //  debug!("{:?}", command);
                if command.is_quit(){
                    break;
                }
               
                io::stdout().write(engine.process_command(command).as_bytes()).unwrap();
                io::stdout().flush().unwrap();             
            },
            Err(e) => { 
                error!("Error reading command:{}", e);
                break; 
            }
        }
    }

}
