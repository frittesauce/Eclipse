use std::fmt::Display;

use crate::compiler::{
    analyzer::IRType, errors::{CompileResult, Location}, path::Path, types::{BaseType, ReferenceManager, ReferenceState, Type}
};

#[derive(Debug, Default)]
pub enum Node {
    #[default]
    Uknown,
    Continue,
    Break,
    Enum {
        name: String,
        fields: Vec<String>,
    },
    Struct {
        name: String,
        fields: Vec<(String, Type)>,
    },
    Function {
        export: bool,
        name: String,
        key: String,
        parameters: Vec<(bool, String, Type)>,
        return_type: Type,
        body: Vec<NodeInfo>,
    },
    Scope(Vec<NodeInfo>),
    SetVariable {
        name: String,
        expression: Option<ExpressionInfo>,
    },
    DeclareVariable {
        name: String,
        mutable: bool,
        data_type: Option<Type>,
        expression: Option<ExpressionInfo>,
    },
    IfStatement {
        expression: (ExpressionInfo, Vec<NodeInfo>),
        elseif: Vec<(ExpressionInfo, Vec<NodeInfo>)>,
        else_body: Option<Vec<NodeInfo>>,
    },
    Loop {
        condition: Option<ExpressionInfo>,
        body: Vec<NodeInfo>
    },
    Call(Path, Vec<ExpressionInfo>),
    Return(Option<ExpressionInfo>),
    NameSpace {
        public: bool,
        static_path: Path,
    },
}

#[derive(Debug, Default)]
pub struct NodeInfo {
    pub location: Location,
    pub node: Node,
}
impl NodeInfo {
    pub fn void() -> Self {
        Self {
            location: Location::void(),
            node: Node::Uknown
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Value(Value),
    GetVariable(Path),
    Call(Path, Vec<ExpressionInfo>),
    BinaryOperation(Box<ExpressionInfo>, ArithmeticOperator, Box<ExpressionInfo>),
    CompareOperation(Box<ExpressionInfo>, CompareOperator, Box<ExpressionInfo>),
    Array(Vec<ExpressionInfo>),
    Tuple(Vec<ExpressionInfo>),
    Minus(Box<ExpressionInfo>),
    Not(Box<ExpressionInfo>),
    // Field(Box<ExpressionInfo>, Box<ExpressionInfo>)
}

#[derive(Debug)]
pub struct ExpressionInfo {
    pub location: Location,
    pub ref_state: ReferenceState,
    pub expression: Expression,
}
impl ReferenceManager for ExpressionInfo {
    fn add_pointer(&mut self) -> CompileResult<()> {
        match self.ref_state {
            ReferenceState::None => self.ref_state = ReferenceState::Pointer(1),
            ReferenceState::Pointer(p) => self.ref_state = ReferenceState::Pointer(p + 1),
            _ => return Err(())
        }
        return Ok(())
    }
    fn add_reference(&mut self) -> CompileResult<()> {
        match self.ref_state {
            ReferenceState::None | ReferenceState::Shared => self.ref_state = ReferenceState::Shared,
            _ => return Err(())
        }
        return Ok(())
    }
} 

#[derive(Debug)]
pub enum ArithmeticOperator {
    // Modulus
    Plus,
    Subtract,
    Division,
    Multiply,
}
impl ArithmeticOperator {
    pub fn convert(&self, data_type: &IRType) -> String {
        if data_type.is_float() {
            return format!("{}", match &self {
                ArithmeticOperator::Plus => "fadd",
                ArithmeticOperator::Subtract => "fsub",
                ArithmeticOperator::Multiply => "fmul",
                ArithmeticOperator::Division => "fdiv",
            })
        }

        if data_type.signed() {
            return format!("{}", match &self {
                ArithmeticOperator::Plus => "add",
                ArithmeticOperator::Subtract => "sub",
                ArithmeticOperator::Multiply => "mul",
                ArithmeticOperator::Division => "sdiv",
            })
        }

        format!("{}", match &self {
            ArithmeticOperator::Plus => "uadd",
            ArithmeticOperator::Subtract => "usub",
            ArithmeticOperator::Multiply => "umul",
            ArithmeticOperator::Division => "udiv",
        })
    }
}

#[derive(Debug)]
pub enum CompareOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,
}
impl CompareOperator {
    pub fn convert(&self, data_type: &IRType) -> String {
        if data_type.is_float() {
            return format!("fcmp {}", match &self {
                Self::Equals => "oeq",
                Self::NotEquals => "one",
                Self::GreaterThan => "ogt",
                Self::GreaterThanOrEquals => "oge",
                Self::LessThan => "olt",
                Self::LessThanOrEquals => "ole",
            })
        }

        if data_type.signed() {
            return format!("icmp {}", match &self {
                Self::Equals => "eq",
                Self::NotEquals => "ne",
                Self::GreaterThan => "sgt",
                Self::GreaterThanOrEquals => "sge",
                Self::LessThan => "slt",
                Self::LessThanOrEquals => "sle",
            })
        }

        format!("icmp {}", match &self {
            Self::Equals => "eq",
            Self::NotEquals => "ne",
            Self::GreaterThan => "ugt",
            Self::GreaterThanOrEquals => "uge",
            Self::LessThan => "ult",
            Self::LessThanOrEquals => "ule",
        })
    }
}


#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Integer(String),
    Float(String),
    StaticString(String),
}
impl Value {
    pub fn default_type(&self) -> Type {
        let mut a = Type::default();
        match &self {
            Self::Boolean(_) => a.base = BaseType::Boolean,
            Self::Float(_) => a.base = BaseType::Float64,
            Self::Integer(_) => a.base = BaseType::Int(32),
            Self::StaticString(_) => todo!()
        }
        return a;
    }
}
