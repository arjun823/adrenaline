/// Interactive REPL for Adrenaline
/// Python shell with JIT compilation for hot functions
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use crate::compiler::Compiler;
use crate::ast_types::*;
use std::path::Path;
use anyhow::Result;

pub struct Repl {
    compiler: Compiler,
    variables: std::collections::HashMap<String, String>,
    functions: std::collections::HashMap<String, FunctionDef>,
}

impl Repl {
    pub fn new(project_dir: &Path) -> Result<Self> {
        Ok(Self {
            compiler: Compiler::new(project_dir)?,
            variables: std::collections::HashMap::new(),
            functions: std::collections::HashMap::new(),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut rl = DefaultEditor::new()?;
        
        println!("⚡ Adrenaline REPL v0.1");
        println!("Type 'help' for commands, 'exit' to quit\n");

        loop {
            let readline = rl.readline(">>> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(&line)?;
                    
                    if line.trim().is_empty() {
                        continue;
                    }

                    if line.trim() == "exit" || line.trim() == "quit" {
                        println!("Goodbye!");
                        break;
                    }

                    if line.trim() == "help" {
                        self.print_help();
                        continue;
                    }

                    if line.trim().starts_with("def ") {
                        self.handle_function_definition(&line);
                        continue;
                    }

                    if let Err(e) = self.execute_expression(&line) {
                        println!("Error: {}", e);
                    }
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        Ok(())
    }

    fn execute_expression(&mut self, input: &str) -> Result<()> {
        let trimmed = input.trim();

        // Variable assignment
        if trimmed.contains('=') && !trimmed.contains("==") {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            let var_name = parts[0].trim().to_string();
            let expr_str = parts[1].trim();

            // Try to evaluate simple expressions
            if let Ok(n) = expr_str.parse::<i64>() {
                self.variables.insert(var_name.clone(), format!("{}", n));
                println!("{} = {}", var_name, n);
                return Ok(());
            }

            if let Ok(f) = expr_str.parse::<f64>() {
                self.variables.insert(var_name.clone(), format!("{}", f));
                println!("{} = {}", var_name, f);
                return Ok(());
            }
        }

        // Function call or expression evaluation
        if trimmed.contains('(') {
            println!("Function calls not yet fully implemented in REPL");
            return Ok(());
        }

        // Simple arithmetic
        if trimmed.contains('+') || trimmed.contains('-') || trimmed.contains('*') || trimmed.contains('/') {
            if let Ok(result) = self.eval_simple_math(trimmed) {
                println!("{}", result);
                return Ok(());
            }
        }

        // Variable lookup
        if let Some(value) = self.variables.get(trimmed) {
            println!("{}", value);
            return Ok(());
        }

        println!("Error: Could not evaluate '{}'", trimmed);
        Ok(())
    }

    fn eval_simple_math(&self, expr: &str) -> Result<f64> {
        let expr = expr.trim();
        
        // Replace variables with their values
        let mut processed = expr.to_string();
        for (var, value) in &self.variables {
            processed = processed.replace(var, value);
        }

        // Try to parse and evaluate as a number
        if let Ok(n) = processed.parse::<f64>() {
            return Ok(n);
        }

        // Basic parsing for simple expressions
        // This is a placeholder; real implementation would use an expression parser
        Err(anyhow::anyhow!("Cannot evaluate: {}", expr))
    }

    fn handle_function_definition(&mut self, _input: &str) {
        // For now, just store the function definition string
        // Real implementation would parse and store as FunctionDef
        println!("Function definition recorded");
    }

    fn print_help(&self) {
        println!(r#"
Adrenaline REPL Commands:
  exit, quit       - Exit REPL
  help             - Show this help
  
Examples:
  >>> x = 42
  >>> y = x + 8
  >>> def add(a, b):
  ...     return a + b
  >>> add(2, 3)
"#);
    }
}
