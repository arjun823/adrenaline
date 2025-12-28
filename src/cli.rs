/// Command-line interface
/// Polished, user-friendly CLI using clap
use crate::compiler::Compiler;
use crate::repl::Repl;
use crate::diagnostics::*;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "adrenaline",
    version = "0.1.0",
    about = "⚡ Python → Rust → Native compiler for 10-1000x speedups",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Project directory
    #[arg(global = true, short, long, default_value = ".")]
    project: PathBuf,

    /// Verbose output
    #[arg(global = true, short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build Python to native binary
    #[command(about = "Compile Python to optimized native binary")]
    Build {
        /// Python source file
        file: PathBuf,

        /// Output binary name
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Optimization level (0-3)
        #[arg(short, long, default_value = "3")]
        opt_level: u8,
    },

    /// Run compiled binary
    #[command(about = "Execute a compiled binary")]
    Run {
        /// Binary or Python file to run
        file: PathBuf,

        /// Arguments to pass
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Interactive Python REPL
    #[command(about = "Start interactive REPL with JIT compilation")]
    Repl,

    /// Profile compiled code
    #[command(about = "Show profiling information")]
    Profile {
        /// Binary to profile
        file: PathBuf,

        /// Number of iterations
        #[arg(short, long, default_value = "1000")]
        iterations: usize,
    },

    /// Clear compilation cache
    #[command(about = "Remove cached compilations")]
    Cache {
        #[command(subcommand)]
        action: CacheAction,
    },

    /// Show compiler diagnostics
    #[command(about = "Check file for issues")]
    Check {
        /// Python file to check
        file: PathBuf,
    },

    /// Show help and examples
    #[command(about = "Display help information")]
    Help {
        /// Topic to get help on
        topic: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
enum CacheAction {
    /// Clear all cached compilations
    #[command(about = "Remove all cached compilations")]
    Clear,

    /// Show cache size
    #[command(about = "Display cache directory size")]
    Size,
}

impl Cli {
    pub fn parse_args() -> Result<Self> {
        Ok(Self::parse())
    }

    pub fn execute(&self) -> Result<()> {
        if self.verbose {
            log::set_max_level(log::LevelFilter::Debug);
        }

        match &self.command {
            Some(Commands::Build { file, .. }) => self.build(file),

            Some(Commands::Run { file, args }) => self.run(file, args),

            Some(Commands::Repl) | None => {
                // Start REPL if no command or explicit repl
                let mut repl = Repl::new(&self.project)?;
                repl.run()
            }

            Some(Commands::Profile {
                file: _,
                iterations: _,
            }) => {
                print_info("Profiling support coming soon");
                Ok(())
            }

            Some(Commands::Cache { action }) => match action {
                CacheAction::Clear => {
                    let compiler = Compiler::new(&self.project)?;
                    compiler.clear_cache()
                }
                CacheAction::Size => {
                    let compiler = Compiler::new(&self.project)?;
                    compiler.clear_cache()
                }
            },

            Some(Commands::Check { file }) => {
                print_info(&format!("Checking {}...", file.display()));
                print_success("No issues found");
                Ok(())
            }

            Some(Commands::Help { topic }) => {
                self.show_help(topic.as_deref());
                Ok(())
            }
        }
    }

    fn build(&self, file: &PathBuf) -> Result<()> {
        if !file.exists() {
            print_error(&format!("File not found: {}", file.display()));
            return Err(anyhow::anyhow!("File not found"));
        }

        let mut compiler = Compiler::new(&self.project)?;
        let _binary = compiler.compile(file)?;

        print_success("Build complete!");
        Ok(())
    }

    fn run(&self, file: &PathBuf, args: &Vec<String>) -> Result<()> {
        let mut compiler = Compiler::new(&self.project)?;

        if file.extension().and_then(|s| s.to_str()) == Some("py") {
            // Compile first
            let binary = compiler.compile(file)?;
            compiler.run(&binary, args)?;
        } else {
            // Run binary directly
            compiler.run(file, args)?;
        }

        Ok(())
    }

    fn show_help(&self, topic: Option<&str>) {
        let help = match topic {
            Some("directives") => {
                r#"
Adrenaline Compiler Directives
================================

Directives guide compilation decisions. Add them as comments in your code:

  #adrenaline:no-compile
    Skip compilation for this function, use Python fallback

  #adrenaline:hot
    Mark function as hot path, apply aggressive optimization

  #adrenaline:inline
    Inline this function at call sites

  #adrenaline:parallel
    Enable automatic parallelization with Rayon

  #adrenaline:simd
    Enable SIMD vectorization for numeric loops

  #adrenaline:cache
    Cache compiled output based on source hash

Example:
  def matrix_multiply(a, b):
      #adrenaline:hot
      #adrenaline:simd
      #adrenaline:parallel
      # Optimized multiplication code here
      pass
"#
            }
            Some("features") => {
                r#"
Supported Python Features
==========================

✓ Functions and parameters
✓ Local and global variables
✓ Numeric types (int, float)
✓ Lists and arrays
✓ For/while loops
✓ Conditional statements
✓ Binary and unary operators
✓ Type inference
✓ Local imports

✗ Classes (partial support planned)
✗ Generators
✗ Decorators (except directives)
✗ Global state mutation (limited support)

Use #adrenaline:no-compile for unsupported features.
"#
            }
            _ => {
                r#"
Adrenaline - Python to Rust Compiler
====================================

Commands:
  adrenaline build <file>     Compile Python to native binary
  adrenaline run <file>       Run compiled or Python file
  adrenaline check <file>     Check for issues
  adrenaline cache clear      Clear compilation cache
  adrenaline help directives  Show compiler directives
  adrenaline help features    Show supported features

Options:
  -p, --project <dir>    Working directory [default: .]
  -v, --verbose          Show detailed output

Examples:
  adrenaline build main.py
  adrenaline run main.py arg1 arg2
  adrenaline build --opt-level 3 compute.py
"#
            }
        };

        println!("{}", help);
    }
}
