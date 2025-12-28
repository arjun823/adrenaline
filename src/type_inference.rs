/// Type inference system
/// Infers types automatically from Python code
use crate::ast_types::*;
use std::collections::HashMap;

pub struct TypeInference {
    variable_types: HashMap<String, Type>,
    function_return_types: HashMap<String, Type>,
}

impl TypeInference {
    pub fn new() -> Self {
        Self {
            variable_types: HashMap::new(),
            function_return_types: HashMap::new(),
        }
    }

    pub fn infer_program(&mut self, program: &mut Program) {
        // First pass: collect function signatures
        for stmt in &program.statements {
            if let Statement::FunctionDef(func) = stmt {
                self.function_return_types
                    .insert(func.name.clone(), Type::Unknown);
            }
        }

        // Second pass: infer types in statements
        for stmt in &mut program.statements {
            self.infer_statement(stmt);
        }
    }

    fn infer_statement(&mut self, stmt: &mut Statement) {
        match stmt {
            Statement::Assign(assign) => {
                let expr_type = self.infer_expression(&assign.value);
                for target in &assign.targets {
                    self.variable_types
                        .insert(target.clone(), expr_type.clone());
                }
            }
            Statement::FunctionDef(func) => {
                // Infer parameter types and return type
                for stmt in &mut func.body {
                    self.infer_statement(stmt);
                }

                // Infer return type from return statements
                func.return_type = self.infer_function_return_type(&func.body);
            }
            Statement::For(for_loop) => {
                let iter_type = self.infer_expression(&for_loop.iter);
                if let Type::List(element_type) = iter_type {
                    self.variable_types
                        .insert(for_loop.target.clone(), *element_type);
                } else if let Type::Array(element_type, _) = iter_type {
                    self.variable_types
                        .insert(for_loop.target.clone(), *element_type);
                }

                for stmt in &mut for_loop.body {
                    self.infer_statement(stmt);
                }
            }
            Statement::While(while_loop) => {
                for stmt in &mut while_loop.body {
                    self.infer_statement(stmt);
                }
            }
            Statement::If(if_stmt) => {
                for stmt in &mut if_stmt.then_body {
                    self.infer_statement(stmt);
                }
                if let Some(else_body) = &mut if_stmt.else_body {
                    for stmt in else_body {
                        self.infer_statement(stmt);
                    }
                }
            }
            _ => {}
        }
    }

    fn infer_expression(&self, expr: &Expression) -> Type {
        match expr {
            Expression::IntLit(_) => Type::Int,
            Expression::FloatLit(_) => Type::Float,
            Expression::BoolLit(_) => Type::Bool,
            Expression::StringLit(_) => Type::String,
            Expression::Identifier(name) => self
                .variable_types
                .get(name)
                .cloned()
                .unwrap_or(Type::Unknown),
            Expression::BinOp(left, op, right) => {
                let left_type = self.infer_expression(left);
                let right_type = self.infer_expression(right);

                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mult | BinOp::Div | BinOp::Pow => {
                        if left_type == Type::Float || right_type == Type::Float {
                            Type::Float
                        } else if left_type == Type::Int && right_type == Type::Int {
                            if matches!(op, BinOp::Div) {
                                Type::Float
                            } else {
                                Type::Int
                            }
                        } else {
                            Type::Unknown
                        }
                    }
                    BinOp::FloorDiv | BinOp::Mod => Type::Int,
                    BinOp::Eq
                    | BinOp::NotEq
                    | BinOp::Lt
                    | BinOp::LtE
                    | BinOp::Gt
                    | BinOp::GtE
                    | BinOp::Is
                    | BinOp::IsNot
                    | BinOp::In
                    | BinOp::NotIn
                    | BinOp::And
                    | BinOp::Or => Type::Bool,
                    _ => Type::Unknown,
                }
            }
            Expression::Call(func, _) => {
                if let Expression::Identifier(name) = &**func {
                    self.function_return_types
                        .get(name)
                        .cloned()
                        .unwrap_or(Type::Unknown)
                } else {
                    Type::Unknown
                }
            }
            Expression::List(elements) => {
                if elements.is_empty() {
                    Type::List(Box::new(Type::Unknown))
                } else {
                    let elem_type = self.infer_expression(&elements[0]);
                    Type::List(Box::new(elem_type))
                }
            }
            Expression::Tuple(elements) => {
                let types = elements.iter().map(|e| self.infer_expression(e)).collect();
                Type::Tuple(types)
            }
            _ => Type::Unknown,
        }
    }

    fn infer_function_return_type(&self, body: &[Statement]) -> Type {
        for stmt in body {
            if let Statement::Return(Some(expr)) = stmt {
                return self.infer_expression(expr);
            }
        }
        Type::Unknown
    }

    pub fn get_variable_type(&self, name: &str) -> Type {
        self.variable_types
            .get(name)
            .cloned()
            .unwrap_or(Type::Unknown)
    }
}

impl Default for TypeInference {
    fn default() -> Self {
        Self::new()
    }
}
