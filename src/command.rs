mod add;
mod list;

use core::fmt;

pub use add::AddCommand;
pub use list::ListCommand;
pub use add::AddCommandArgs;
pub use list::ListCommandArgs;

pub enum CommandType {
    ADD,
    LIST,
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandType::ADD => write!(f, "ADD"),
            CommandType::LIST => write!(f, "LIST"),
            _ => write!(f, "UNKNOWN COMMAND"),
        }
    }
}

pub trait Command {
    fn execute(&mut self) -> bool;
    fn undo(&mut self) -> bool;
}
