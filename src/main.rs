#[macro_use]
extern crate log;
extern crate pleco;
extern crate rand;
extern crate simple_logging;

mod command;

use command::{Command, CommandIt};
use std::io::{self, Write};
use pleco::Board;
use rand::Rng;

struct Engine {
    chessboard: Board,
}

impl Engine {
    fn new() -> Engine {
        Engine {
            chessboard: Board::start_pos(),
        }
    }

    fn answer_to(&mut self, command: Command) -> std::string::String {
        match command {
            Command::Quit => std::string::String::new(),
            Command::Unknown => std::string::String::new(),
            Command::Position { fen, moves } => {
                if let Some(_fenstring) = fen {
                    panic!("fen not yet implementend")
                } else {
                    self.chessboard = Board::start_pos();
                }
                for mv in moves {
                    self.chessboard.apply_uci_move(&mv);
                }
                String::new()
            }
            Command::Go => {
                let moves = self.chessboard.generate_moves();

                if let Some(mv) = rand::thread_rng().choose(&moves) {
                    self.chessboard.apply_move(*mv);
                    std::string::String::from("bestmove ") + &mv.stringify() + "\n"
                } else {
                    std::string::String::new()
                }
            }
            Command::Uci => std::string::String::from(
                "id name Luci\nid author Gunnar Klaemke, Markus Klein\nuciok\n",
            ),
            Command::IsReady => std::string::String::from("readyok\n"),
        }
    }
}

fn main() {
    simple_logging::log_to_file("luci.log", log::LevelFilter::Debug).unwrap();
    info!("Start");

    let mut engine = Engine::new();
    let input = io::stdin();
    let commands = CommandIt::new(input.lock());

    for cmd in commands.take_while(|c| *c != Command::Quit) {
        debug!("{:?}", cmd);

        io::stdout()
            .write(engine.answer_to(cmd).as_bytes())
            .unwrap();
        io::stdout().flush().unwrap();
    }
}
