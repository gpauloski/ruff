use ruff_diagnostics::{Violation, Diagnostic, Fix, Edit};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{self as ast, Stmt};
use ruff_text_size::Ranged;

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

    let mut diagnostic = Diagnostic::new(EmptyIfStatement, *range);

    diagnostic.set_fix(Fix::unsafe_edit(Edit::range_deletion(diagnostic.range())));

    checker.diagnostics.push(diagnostic);
}
