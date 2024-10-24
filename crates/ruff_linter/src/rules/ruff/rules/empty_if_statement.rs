use ruff_diagnostics::{Violation, Diagnostic};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{self as ast, Stmt};
use crate::checkers::ast::Checker;

/// ### What is does
/// TODO
#[violation]
pub struct EmptyIfStatement;

impl Violation for EmptyIfStatement {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Empty if statement")
    }
}

pub(crate) fn empty_if_statement(
    checker: &mut Checker,
    stmt: &ast::StmtIf,
) {
    let ast::StmtIf {
        range,
        body,
        elif_else_clauses,
        ..
    } = stmt;

    if !matches!(body[..], [Stmt::Pass(_)]) {
        return;
    }

    for clause in elif_else_clauses {
        if !matches!(clause.body[..], [Stmt::Pass(_)]) {
            return;
        }
    }

    checker.diagnostics.push(Diagnostic::new(EmptyIfStatement, *range));
}
