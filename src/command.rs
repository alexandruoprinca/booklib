mod add;
mod list;

use core::fmt;

pub use add::AddCommand;
pub use add::AddCommandArgs;
pub use list::ListCommand;
pub use list::ListCommandArgs;

pub enum CommandType {
    ADD,
    LIST,
}

pub trait Command {
    fn execute(&mut self) -> bool;
    fn undo(&mut self) -> bool;
}
