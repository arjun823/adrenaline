/// Intermediate Representation (IR)
/// Typed, optimizable representation between AST and Rust codegen
use crate::ast_types::Type;
use crate::directives::DirectiveSet;

#[derive(Debug, Clone)]
pub struct IRModule {
    pub functions: Vec<IRFunction>,
    pub globals: Vec<IRGlobal>,
    pub hot_functions: Vec<String>, // Profiled hot functions
}

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub params: Vec<IRParam>,
    pub return_type: Type,
    pub blocks: Vec<BasicBlock>,
    pub directives: DirectiveSet,
    pub optimization_level: OptimizationLevel,
}

#[derive(Debug, Clone)]
pub struct IRParam {
    pub name: String,
    pub typ: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    None,
    Basic,
    Aggressive,
    Extreme,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: usize,
    pub instructions: Vec<IRInstruction>,
    pub successors: Vec<usize>,
}

#[derive(Debug, Clone)]
pub enum IRInstruction {
    // Arithmetic
    BinOp {
        result: IRValue,
        op: BinOpIR,
        left: IRValue,
        right: IRValue,
    },
    UnaryOp {
        result: IRValue,
        op: UnaryOpIR,
        operand: IRValue,
    },

    // Memory
    Assign {
        target: IRValue,
        value: IRValue,
    },
    Load {
        result: IRValue,
        source: String,
    },
    Store {
        target: String,
        value: IRValue,
    },
    Index {
        result: IRValue,
        array: IRValue,
        index: IRValue,
    },
    IndexStore {
        array: IRValue,
        index: IRValue,
        value: IRValue,
    },

    // Control flow
    Branch {
        condition: IRValue,
        true_block: usize,
        false_block: usize,
    },
    Jump {
        target: usize,
    },
    Return {
        value: Option<IRValue>,
    },

    // Function calls
    Call {
        result: IRValue,
        function: String,
        args: Vec<IRValue>,
    },

    // Loops (for optimization passes)
    LoopStart {
        iterator: IRValue,
        body_block: usize,
        exit_block: usize,
    },
    LoopEnd,

    // Hints for optimization
    Vectorizable,
    Parallelizable,
    CanElideCheck,
    Pure, // Function has no side effects
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOpIR {
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Mod,
    Pow,
    BitAnd,
    BitOr,
    BitXor,
    LShift,
    RShift,
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOpIR {
    Neg,
    Not,
    BitNot,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IRValue {
    Const(IRConstant),
    Local(String),
    Temporary(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IRConstant {
    Int(i64),
    Bool(bool),
    String(String),
    Null,
}

#[derive(Debug, Clone)]
pub struct IRGlobal {
    pub name: String,
    pub typ: Type,
    pub initializer: Option<IRValue>,
}

impl IRModule {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            globals: Vec::new(),
            hot_functions: Vec::new(),
        }
    }

    pub fn mark_hot_function(&mut self, name: &str) {
        if !self.hot_functions.contains(&name.to_string()) {
            self.hot_functions.push(name.to_string());
        }
    }

    pub fn get_function(&self, name: &str) -> Option<&IRFunction> {
        self.functions.iter().find(|f| f.name == name)
    }

    pub fn get_function_mut(&mut self, name: &str) -> Option<&mut IRFunction> {
        self.functions.iter_mut().find(|f| f.name == name)
    }
}

impl Default for IRModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BasicBlock {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            instructions: Vec::new(),
            successors: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instr: IRInstruction) {
        self.instructions.push(instr);
    }
}
