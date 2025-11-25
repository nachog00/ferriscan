pub mod commands;
pub mod repo;

use anyhow::Result;

pub trait SubCommand {
    fn run(self) -> Result<()>;
}
