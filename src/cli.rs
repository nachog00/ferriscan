pub mod args;
pub mod commands;

use anyhow::Result;

pub trait SubCommand {
    fn run(self) -> Result<()>;
}
