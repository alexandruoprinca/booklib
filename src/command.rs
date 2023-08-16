mod add;
mod list;

use core::fmt;

pub use add::*;
pub use list::*;

pub enum CommandType {
    ADD,
    LIST,
}

pub trait Command {
    fn execute(&mut self) -> bool;
    fn undo(&mut self) -> bool;
}
