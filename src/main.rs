mod ast_types;
mod cache;
mod cli;
mod codegen;
mod compiler;
mod diagnostics;
mod directives;
mod ir;
mod optimizer;
mod parser;
mod profiler;
mod repl;
mod runtime;
mod type_inference;

use anyhow::Result;
use cli::Cli;

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse_args()?;
    cli.execute()
}
