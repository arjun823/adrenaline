/// Python source code parser
/// Extracts functions, statements, and directives from Python code
use crate::ast_types::*;
use anyhow::{anyhow, Result};

pub struct AdrenalineParser;

impl AdrenalineParser {
    pub fn parse(source: &str) -> Result<Program> {
        let mut parser = PythonParser::new(source);
        parser.parse_program()
    }
}

struct PythonParser {
    lines: Vec<String>,
    current: usize,
}

impl PythonParser {
    fn new(source: &str) -> Self {
        Self {
            lines: source.lines().map(|s| s.to_string()).collect(),
            current: 0,
        }
    }

    fn parse_program(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        let mut imports = Vec::new();

        while self.current < self.lines.len() {
            let line = self.lines[self.current].clone();
            let trimmed = line.trim().to_string();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                self.current += 1;
                continue;
            }

            if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
                imports.push(self.parse_import(&trimmed)?);
                self.current += 1;
            } else if trimmed.starts_with("def ") {
                statements.push(Statement::FunctionDef(self.parse_function()?));
            } else {
                self.current += 1;
            }
        }

        Ok(Program { statements, imports })
    }

    fn parse_function(&mut self) -> Result<FunctionDef> {
        let def_line = self.lines[self.current].clone();
        self.current += 1;

        // Parse: def name(params) -> return_type:
        let def_regex = r#"def\s+(\w+)\s*\((.*?)\)\s*(?:->\s*(\w+))?\s*:"#;
        let re = regex::Regex::new(def_regex)?;
        let caps = re.captures(&def_line)
            .ok_or_else(|| anyhow!("Invalid function definition: {}", def_line))?;

        let name = caps.get(1).unwrap().as_str().to_string();
        let params_str = caps.get(2).unwrap().as_str();
        let return_type_str = caps.get(3).map(|m| m.as_str()).unwrap_or("()");

        let params = self.parse_parameters(params_str)?;
        let return_type = self.parse_type(return_type_str);

        let mut directives = Vec::new();
        let mut body = Vec::new();
        let base_indent = self.get_indent(&self.lines[self.current]);

        while self.current < self.lines.len() {
            let line = self.lines[self.current].clone();
            let indent = self.get_indent(&line);
            let trimmed = line.trim().to_string();

            if !line.is_empty() && !trimmed.starts_with("#") && indent <= base_indent {
                break;
            }

            if trimmed.contains("#adrenaline:") {
                if let Some(start) = trimmed.find("#adrenaline:") {
                    let directive = trimmed[start + 12..].to_string();
                    directives.push(directive);
                }
            } else if !trimmed.is_empty() && !trimmed.starts_with("#") {
                if let Ok(stmt) = self.parse_statement(&trimmed) {
                    body.push(stmt);
                }
            }

            self.current += 1;
        }

        if body.is_empty() {
            body.push(Statement::Pass);
        }

        Ok(FunctionDef {
            name,
            params,
            return_type,
            body,
            directives,
        })
    }

    fn parse_statement(&self, line: &str) -> Result<Statement> {
        let trimmed = line.trim();

        // Assignment
        if trimmed.contains('=') && !trimmed.contains("==") {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            let targets = vec![parts[0].trim().to_string()];
            let value = self.parse_expression(parts[1].trim())?;
            return Ok(Statement::Assign(Assignment { targets, value }));
        }

        // For loop
        if trimmed.starts_with("for ") {
            let for_regex = regex::Regex::new(r"for\s+(\w+)\s+in\s+(.+?)\s*:")?;
            if let Some(caps) = for_regex.captures(trimmed) {
                let target = caps.get(1).unwrap().as_str().to_string();
                let iter = self.parse_expression(caps.get(2).unwrap().as_str())?;
                return Ok(Statement::For(ForLoop {
                    target,
                    iter,
                    body: vec![],
                }));
            }
        }

        // If statement
        if trimmed.starts_with("if ") {
            let cond_str = trimmed[3..].trim_end_matches(':');
            let condition = self.parse_expression(cond_str)?;
            return Ok(Statement::If(IfStatement {
                condition,
                then_body: vec![],
                else_body: None,
            }));
        }

        // Return
        if trimmed.starts_with("return ") {
            let expr_str = &trimmed[7..];
            let expr = if expr_str.trim().is_empty() {
                None
            } else {
                Some(self.parse_expression(expr_str)?)
            };
            return Ok(Statement::Return(expr));
        }

        let expr = self.parse_expression(trimmed)?;
        Ok(Statement::ExprStatement(expr))
    }

    fn parse_expression(&self, expr_str: &str) -> Result<Expression> {
        let trimmed = expr_str.trim();

        // Literals
        if let Ok(n) = trimmed.parse::<i64>() {
            return Ok(Expression::IntLit(n));
        }
        if let Ok(f) = trimmed.parse::<f64>() {
            return Ok(Expression::FloatLit(f));
        }
        if trimmed == "True" {
            return Ok(Expression::BoolLit(true));
        }
        if trimmed == "False" {
            return Ok(Expression::BoolLit(false));
        }
        if (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        {
            return Ok(Expression::StringLit(trimmed[1..trimmed.len() - 1].to_string()));
        }

        // Function call
        if trimmed.contains('(') && trimmed.ends_with(')') {
            if let Some(paren_pos) = trimmed.find('(') {
                let func_name = trimmed[..paren_pos].trim();
                let args_str = &trimmed[paren_pos + 1..trimmed.len() - 1];
                let args = if args_str.is_empty() {
                    vec![]
                } else {
                    args_str
                        .split(',')
                        .map(|arg| self.parse_expression(arg.trim()).unwrap_or(Expression::IntLit(0)))
                        .collect()
                };
                return Ok(Expression::Call(
                    Box::new(Expression::Identifier(func_name.to_string())),
                    args,
                ));
            }
        }

        // Binary operations (check longest first)
        let ops = vec![
            ("**", BinOp::Pow),
            ("//", BinOp::FloorDiv),
            ("==", BinOp::Eq),
            ("!=", BinOp::NotEq),
            ("<=", BinOp::LtE),
            (">=", BinOp::GtE),
            ("+", BinOp::Add),
            ("-", BinOp::Sub),
            ("*", BinOp::Mult),
            ("/", BinOp::Div),
            ("%", BinOp::Mod),
            ("<", BinOp::Lt),
            (">", BinOp::Gt),
        ];

        for (op_str, op) in ops {
            if trimmed.contains(op_str) {
                if let Some(pos) = trimmed.rfind(op_str) {
                    let left_str = trimmed[..pos].trim();
                    let right_str = trimmed[pos + op_str.len()..].trim();
                    if !left_str.is_empty() && !right_str.is_empty() {
                        let left = self.parse_expression(left_str)?;
                        let right = self.parse_expression(right_str)?;
                        return Ok(Expression::BinOp(Box::new(left), op, Box::new(right)));
                    }
                }
            }
        }

        // Index
        if trimmed.contains('[') && trimmed.ends_with(']') {
            if let Some(bracket_pos) = trimmed.find('[') {
                let arr_name = trimmed[..bracket_pos].trim();
                let idx_str = &trimmed[bracket_pos + 1..trimmed.len() - 1];
                let index = self.parse_expression(idx_str)?;
                return Ok(Expression::Index(
                    Box::new(Expression::Identifier(arr_name.to_string())),
                    Box::new(index),
                ));
            }
        }

        Ok(Expression::Identifier(trimmed.to_string()))
    }

    fn parse_parameters(&self, params_str: &str) -> Result<Vec<Parameter>> {
        let mut params = Vec::new();
        if params_str.trim().is_empty() {
            return Ok(params);
        }

        for param in params_str.split(',') {
            let trimmed = param.trim();
            let parts: Vec<&str> = trimmed.split(':').collect();
            let name = parts[0].trim().to_string();
            let typ = if parts.len() > 1 {
                self.parse_type(parts[1].trim())
            } else {
                Type::Int
            };
            params.push(Parameter {
                name,
                typ,
                default: None,
            });
        }

        Ok(params)
    }

    fn parse_type(&self, type_str: &str) -> Type {
        match type_str.trim() {
            "int" => Type::Int,
            "float" => Type::Float,
            "bool" => Type::Bool,
            "str" => Type::String,
            "list" => Type::List(Box::new(Type::Unknown)),
            _ => Type::Unknown,
        }
    }

    fn parse_import(&self, line: &str) -> Result<Import> {
        let import_regex = regex::Regex::new(r"from\s+(\w+)\s+import\s+(.+)")?;
        if let Some(caps) = import_regex.captures(line) {
            let module = caps.get(1).unwrap().as_str().to_string();
            let items_str = caps.get(2).unwrap().as_str();
            let items = if items_str.trim() == "*" {
                None
            } else {
                Some(
                    items_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect(),
                )
            };
            Ok(Import {
                module,
                items,
                alias: None,
            })
        } else {
            let module = line.replace("import ", "").trim().to_string();
            Ok(Import {
                module,
                items: None,
                alias: None,
            })
        }
    }

    fn get_indent(&self, line: &str) -> usize {
        line.len() - line.trim_start().len()
    }
}
