mod analyze;
mod compare;
mod list;

pub use analyze::AnalyzeCommand;
pub use compare::CompareCommand;
pub use list::ListCommand;

use anyhow::Result;

pub trait SubCommand {
    fn run(self) -> Result<()>;
}
