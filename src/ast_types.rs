#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    List(Box<Type>),
    Dict(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Array(Box<Type>, Option<usize>), // NumPy-style arrays
    Unknown,
    Never,
    NoneType
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Float)
    }

    pub fn is_array_like(&self) -> bool {
        matches!(self, Type::Array(..) | Type::List(_))
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub imports: Vec<Import>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    FunctionDef(FunctionDef),
    ClassDef(ClassDef),
    Assign(Assignment),
    AugAssign(AugAssignment),
    For(ForLoop),
    While(WhileLoop),
    If(IfStatement),
    Return(Option<Expression>),
    Pass,
    Break,
    Continue,
    ExprStatement(Expression),
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Vec<Statement>,
    pub return_type: Type,
    pub directives: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub typ: Type,
    pub default: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct ClassDef {
    pub name: String,
    pub bases: Vec<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub targets: Vec<String>,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct AugAssignment {
    pub target: String,
    pub op: BinOp,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub target: String,
    pub iter: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_body: Vec<Statement>,
    pub else_body: Option<Vec<Statement>>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    IntLit(i64),
    FloatLit(f64),
    BoolLit(bool),
    StringLit(String),
    Identifier(String),
    BinOp(Box<Expression>, BinOp, Box<Expression>),
    UnaryOp(UnaryOp, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Index(Box<Expression>, Box<Expression>),
    Slice(
        Box<Expression>,
        Option<Box<Expression>>,
        Option<Box<Expression>>,
        Option<Box<Expression>>,
    ),
    Attribute(Box<Expression>, String),
    List(Vec<Expression>),
    Dict(Vec<(Expression, Expression)>),
    Tuple(Vec<Expression>),
    Lambda(Vec<String>, Box<Expression>),
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
    FloorDiv,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
    And,
    Or,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Not,
    Neg,
    Pos,
    Invert,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub module: String,
    pub items: Option<Vec<String>>, // None = import *
    pub alias: Option<String>,
}
