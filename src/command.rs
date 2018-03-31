use std::io;

/// UCI Command send from the GUI to the chess engine
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Command {
    Quit,
    Unknown,
    Uci,
    IsReady,
}

impl Command {
    fn from_str(command: &str) -> Command {
        match command.trim() {
            "quit" => Command::Quit,
            "uci" => Command::Uci,
            "isready" => Command::IsReady,
            other => {
                warn!("Unkown Command: {}", other);
                Command::Unknown
            }
        }
    }

    pub fn answer(&self) -> &'static str {
        match *self {
            Command::Quit => "",
            Command::Unknown => "",
            Command::Uci => "id name Luci\nid author Gunnar Klaemke, Markus Klein\nuciok\n",
            Command::IsReady => "readyok\n",
        }
    }
}

/// Wraps a `BufRead` as an iterator over `Command`s
pub struct CommandIt<R> {
    buffer: String,
    read: R,
}

impl<R> CommandIt<R> {
    /// Create a new iterator of uci `Command`s from a BufRead
    pub fn new(read: R) -> Self {
        CommandIt {
            buffer: String::new(),
            read,
        }
    }
}

impl<R> Iterator for CommandIt<R>
where
    R: io::BufRead,
{
    type Item = Command;
    fn next(&mut self) -> Option<Command> {
        self.buffer.clear();
        match self.read.read_line(&mut self.buffer) {
            Ok(0) => {
                info!("Encountered EOF");
                None
            }
            Ok(_) => Some(Command::from_str(&self.buffer)),
            Err(e) => {
                error!("Error reading command: {}", e);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn command_iterator() {
        let input = b"uci\nisready\nfoo\nquit\n";
        let commands: Vec<_> = CommandIt::new(&input[..]).collect();
        use Command::*;
        let expected = vec![Uci, IsReady, Unknown, Quit];
        assert_eq!(expected, commands);
    }
}
