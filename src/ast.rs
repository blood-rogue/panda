use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use num_bigint::BigInt;

use crate::token::Position;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}-{})", self.start, self.end)
    }
}

pub type BlockStatement = Vec<Statement>;
pub type Identifier = String;

#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Program {
        span: Span,
        statements: BlockStatement,
    },
    Stmt(Statement),
    Expr(Expression),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Program { statements, .. } => {
                for stmt in statements {
                    write!(f, "{stmt}")?;
                }
            }

            Self::Stmt(stmt) => write!(f, "{stmt}")?,

            Self::Expr(expr) => write!(f, "{expr}")?,
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct DeclarationAst {
    pub span: Span,
    pub name: Identifier,
    pub mutable: bool,
    pub value: Option<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ReturnAst {
    pub span: Span,
    pub return_value: Expression,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DeleteAst {
    pub span: Span,
    pub delete_ident: Identifier,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ExpressionStmtAst {
    pub span: Span,
    pub returns: bool,
    pub expression: Expression,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionAst {
    pub span: Span,
    pub ident: Identifier,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

#[derive(Clone, PartialEq, Debug)]
pub struct WhileAst {
    pub span: Span,
    pub condition: Expression,
    pub body: BlockStatement,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ForAst {
    pub span: Span,
    pub ident: Identifier,
    pub iterator: Expression,
    pub body: BlockStatement,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ClassDeclAst {
    pub span: Span,
    pub ident: Identifier,
    pub initializers: Vec<Identifier>,
    pub body: Vec<ClassStatement>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImportAst {
    pub span: Span,
    pub path: Identifier,
    pub alias: Option<Identifier>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ClassStatement {
    Declaration(DeclarationAst),
    Function(FunctionAst),
}

impl ClassStatement {
    pub fn to_statement(&self) -> Statement {
        match self {
            Self::Declaration(ast_node) => Statement::Declaration(ast_node.clone()),
            Self::Function(ast_node) => Statement::Function(ast_node.clone()),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Statement {
    Declaration(DeclarationAst),
    Return(ReturnAst),
    Delete(DeleteAst),
    ExpressionStmt(ExpressionStmtAst),
    Function(FunctionAst),
    While(WhileAst),
    For(ForAst),
    ClassDecl(ClassDeclAst),
    Import(ImportAst),
    Break(Span),
    Continue(Span),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Declaration(DeclarationAst {
                name,
                mutable,
                value,
                ..
            }) => write!(
                f,
                "{} {}{};",
                if *mutable { "let" } else { "const" },
                name,
                value
                    .clone()
                    .map_or_else(String::new, |value| format!("= {value}"))
            ),

            Self::ClassDecl(ClassDeclAst {
                ident,
                initializers,
                body,
                ..
            }) => write!(
                f,
                "class({}) {} {}",
                initializers.join(", "),
                ident,
                body.iter()
                    .map(|stmt| stmt.to_statement().to_string())
                    .collect::<String>()
            ),

            Self::ExpressionStmt(ExpressionStmtAst {
                returns,
                expression,
                ..
            }) => write!(f, "{}{}", expression, if *returns { "" } else { ";" }),

            Self::For(ForAst {
                ident,
                iterator,
                body,
                ..
            }) => write!(
                f,
                "for ({} in {}) {}",
                ident,
                iterator,
                body.iter().map(ToString::to_string).collect::<String>()
            ),

            Self::Function(FunctionAst {
                ident,
                parameters,
                body,
                ..
            }) => write!(
                f,
                "fn {}({}) {}",
                ident,
                parameters.join(", "),
                body.iter().map(ToString::to_string).collect::<String>()
            ),

            Self::Import(ImportAst { path, alias, .. }) => write!(
                f,
                "import \"{}\"{}",
                path,
                alias
                    .clone()
                    .map_or_else(String::new, |alias| format!(" as {alias}"))
            ),

            Self::Return(ReturnAst { return_value, .. }) => write!(
                f,
                "return {};",
                if matches!(
                    return_value,
                    Expression::Literal(LiteralAst {
                        lit: Literal::Null,
                        ..
                    })
                ) {
                    String::new()
                } else {
                    return_value.to_string()
                }
            ),

            Self::While(WhileAst {
                condition, body, ..
            }) => write!(
                f,
                "while ({}) {}",
                condition,
                body.iter().map(ToString::to_string).collect::<String>()
            ),

            Self::Break(_) => write!(f, "break"),

            Self::Continue(_) => write!(f, "continue"),

            Self::Delete(DeleteAst { delete_ident, .. }) => write!(f, "delete {delete_ident};"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct MethodAst {
    pub span: Span,
    pub left: Box<Expression>,
    pub method: Identifier,
    pub arguments: Option<Vec<Expression>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ConstructorAst {
    pub span: Span,
    pub constructable: Constructable,
}

#[derive(Clone, PartialEq, Debug)]
pub struct RangeAst {
    pub span: Span,
    pub start: Box<Expression>,
    pub stop: Box<Expression>,
    pub step: Option<Box<Expression>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IdentifierAst {
    pub span: Span,
    pub value: Identifier,
}

#[derive(Clone, PartialEq, Debug)]
pub struct AssignAst {
    pub span: Span,
    pub to: Assignable,
    pub value: Box<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct PrefixAst {
    pub span: Span,
    pub operator: Operator,
    pub right: Box<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct InfixAst {
    pub span: Span,
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct IfAst {
    pub span: Span,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LambdaAst {
    pub span: Span,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub name: Identifier,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CallAst {
    pub span: Span,
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct IndexAst {
    pub span: Span,
    pub left: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LiteralAst {
    pub span: Span,
    pub lit: Literal,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ScopeAst {
    pub span: Span,
    pub module: Identifier,
    pub member: Box<Expression>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
    Method(MethodAst),
    Constructor(ConstructorAst),
    Range(RangeAst),
    Identifier(IdentifierAst),
    Assign(AssignAst),
    Prefix(PrefixAst),
    Infix(InfixAst),
    If(IfAst),
    Lambda(LambdaAst),
    Call(CallAst),
    Index(IndexAst),
    Literal(LiteralAst),
    Scope(ScopeAst),
}

impl Expression {
    pub fn get_span(&self) -> Span {
        match self {
            Self::Method(node) => node.span,
            Self::Constructor(node) => node.span,
            Self::Range(node) => node.span,
            Self::Identifier(node) => node.span,
            Self::Assign(node) => node.span,
            Self::Prefix(node) => node.span,
            Self::Infix(node) => node.span,
            Self::If(node) => node.span,
            Self::Lambda(node) => node.span,
            Self::Call(node) => node.span,
            Self::Index(node) => node.span,
            Self::Literal(node) => node.span,
            Self::Scope(node) => node.span,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign(AssignAst { to, value, .. }) => write!(f, "{to} = {value};"),

            Self::Call(CallAst {
                function,
                arguments,
                ..
            }) => write!(
                f,
                "{}({})",
                function,
                arguments
                    .iter()
                    .map(ToString::to_string)
                    .collect::<String>()
            ),

            Self::Constructor(ConstructorAst { constructable, .. }) => {
                write!(f, "new {constructable};")
            }

            Self::Lambda(LambdaAst {
                parameters,
                body,
                name,
                ..
            }) => write!(
                f,
                "{}fn({}) {}",
                if name.is_empty() {
                    String::new()
                } else {
                    format!("<{name}>")
                },
                parameters.join(", "),
                body.iter().map(ToString::to_string).collect::<String>()
            ),

            Self::Identifier(IdentifierAst { value, .. }) => write!(f, "{value}"),

            Self::If(IfAst {
                condition,
                consequence,
                alternative,
                ..
            }) => write!(
                f,
                "if {} {}{}",
                condition,
                consequence
                    .iter()
                    .map(ToString::to_string)
                    .collect::<String>(),
                alternative.as_ref().map_or_else(String::new, |alt| format!(
                    "else {}",
                    alt.iter().map(ToString::to_string).collect::<String>()
                ))
            ),

            Self::Index(IndexAst { left, index, .. }) => write!(f, "({left}[{index}])"),

            Self::Infix(InfixAst {
                left,
                operator,
                right,
                ..
            }) => write!(f, "({left} {operator} {right})"),

            Self::Literal(LiteralAst { lit, .. }) => write!(f, "{lit}"),

            Self::Method(MethodAst {
                left,
                method,
                arguments,
                ..
            }) => write!(
                f,
                "{}.{}{}",
                left,
                method,
                arguments
                    .as_ref()
                    .map_or_else(String::new, |arguments| format!(
                        "({})",
                        arguments
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
            ),

            Self::Prefix(PrefixAst {
                operator, right, ..
            }) => {
                write!(f, "({operator}{right})")
            }

            Self::Range(RangeAst {
                start, stop, step, ..
            }) => write!(
                f,
                "{start}..{stop}{}",
                step.as_ref()
                    .map_or_else(String::new, |step| format!("..{step}"))
            ),

            Self::Scope(ScopeAst { module, member, .. }) => write!(f, "{module}::{member}"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Literal {
    Int {
        value: BigInt,
    },
    Float {
        value: f64,
    },
    Bool {
        value: bool,
    },
    Null,
    Str {
        value: String,
    },
    Char {
        value: char,
    },
    Array {
        elements: Vec<Expression>,
    },
    Hash {
        pairs: Vec<(Expression, Expression)>,
    },
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array { elements } => write!(
                f,
                "[{}]",
                elements
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),

            Self::Bool { value } => write!(f, "{value}"),

            Self::Char { value } => write!(f, "{value}"),

            Self::Float { value } => write!(f, "{value}"),

            Self::Hash { pairs } => write!(
                f,
                "{{{}}}",
                pairs
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),

            Self::Int { value } => write!(f, "{value}"),

            Self::Null => write!(f, "null"),

            Self::Str { value } => write!(f, "{value}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Operator {
    Eq,
    NotEq,
    And,
    Or,
    Bang,
    Add,
    Sub,
    Mul,
    Div,
    BitXor,
    BitAnd,
    BitOr,
    Shr,
    Shl,
    Gt,
    Lt,
    GtEq,
    LtEq,
}

impl FromStr for Operator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match s {
            "==" => Self::Eq,
            "!=" => Self::NotEq,
            "&&" => Self::And,
            "||" => Self::Or,
            "!" => Self::Bang,
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            "^" => Self::BitXor,
            "&" => Self::BitAnd,
            "|" => Self::BitOr,
            ">>" => Self::Shr,
            "<<" => Self::Shl,
            ">" => Self::Gt,
            "<" => Self::Lt,
            ">=" => Self::GtEq,
            "<=" => Self::LtEq,
            _ => return Err(format!("invalid operator: {s}")),
        };

        Ok(op)
    }
}

impl TryFrom<String> for Operator {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::Eq => "==",
            Self::NotEq => "!=",
            Self::And => "&&",
            Self::Or => "||",
            Self::Bang => "!",
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::BitXor => "^",
            Self::BitAnd => "&",
            Self::BitOr => "|",
            Self::Shr => ">>",
            Self::Shl => "<<",
            Self::Gt => ">",
            Self::Lt => "<",
            Self::GtEq => ">=",
            Self::LtEq => "<=",
        };
        write!(f, "{out}")
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Constructable {
    Identifier(IdentifierAst),
    Call(CallAst),
    Scope(ScopeAst),
}

impl Display for Constructable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Call(CallAst {
                function,
                arguments,
                ..
            }) => write!(
                f,
                "{}({})",
                function,
                arguments
                    .iter()
                    .map(ToString::to_string)
                    .collect::<String>()
            ),

            Self::Scope(ScopeAst { module, member, .. }) => write!(f, "{module}::{member}"),

            Self::Identifier(IdentifierAst { value, .. }) => write!(f, "{value}"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Assignable {
    Identifier(IdentifierAst),
    Method(MethodAst),
    Index(IndexAst),
}

impl Display for Assignable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Method(MethodAst {
                left,
                method,
                arguments,
                ..
            }) => write!(
                f,
                "{}.{}{}",
                left,
                method,
                arguments
                    .as_ref()
                    .map_or_else(String::new, |arguments| format!(
                        "({})",
                        arguments
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
            ),

            Self::Index(IndexAst { left, index, .. }) => write!(f, "{left}[{index}]"),

            Self::Identifier(IdentifierAst { value, .. }) => write!(f, "{value}"),
        }
    }
}