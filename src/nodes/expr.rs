use std::rc::Rc;

use crate::nodes::stmt::Stmt;
use crate::token::Token;
use crate::types::Type;

#[derive(Clone, Debug)]
pub enum Expr {
    Assign(Token, Rc<Expr>),
    Binary(Rc<Expr>, Token, Rc<Expr>),
    Grouping(Rc<Expr>),
    Literal(Type),
    Unary(Token, Rc<Expr>),
    Variable(Token),
    Block(Vec<Stmt>),
    Logical(Rc<Expr>, Token, Rc<Expr>),
    Ternary(Rc<Expr>, Rc<Expr>, Rc<Expr>),
    Call(Rc<Expr>, Token, Vec<Expr>),
    IfExpr(Rc<Expr>, Vec<Stmt>, Vec<(Expr, Vec<Stmt>)>, Option<Vec<Stmt>>),
    Get(Rc<Expr>, Token, Rc<Expr>),
    Set(Rc<Expr>, Token, Rc<Expr>, Rc<Expr>),
    Prop(Rc<Expr>, Token),
    Array(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Range(Rc<Expr>, Token, Rc<Expr>, bool),
}
