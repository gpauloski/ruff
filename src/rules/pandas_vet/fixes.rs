use crate::ast::helpers;
use rustpython_ast::{Expr, ExprKind, Keyword, Location};

use crate::ast::types::Range;
use crate::autofix::apply_fixes;
use crate::autofix::helpers::remove_argument;
use crate::fix::Fix;
use crate::source_code::Locator;

fn match_name(expr: &Expr) -> Option<&str> {
    if let ExprKind::Call { func, .. } = &expr.node {
        if let ExprKind::Attribute { value, .. } = &func.node {
            if let ExprKind::Name { id, .. } = &value.node {
                Some(id)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

/// Remove the `inplace` argument from a function call and replace it with an
/// assignment.
pub fn fix_inplace_argument(
    locator: &Locator,
    expr: &Expr,
    violation_at: Location,
    violation_end: Location,
    args: &[Expr],
    keywords: &[Keyword],
) -> Option<Fix> {
    if let Ok(fix) = remove_argument(
        locator,
        expr.location,
        violation_at,
        violation_end,
        args,
        keywords,
        false,
    ) {
        // Reset the line index.
        let fix_me = Fix::deletion(
            helpers::to_relative(fix.location, expr.location),
            helpers::to_relative(fix.end_location, expr.location),
        );

        // Apply the deletion step.
        // TODO(charlie): Find a way to
        let contents =
            locator.slice_source_code_range(&Range::new(expr.location, expr.end_location.unwrap()));
        let (output, _) = apply_fixes([fix_me].iter(), &Locator::new(contents));

        // Obtain the name prefix.
        let name = match_name(expr)?;

        // Apply the assignment.
        let new_contents = format!("{name} = {output}");

        // Create the new fix.
        Some(Fix::replacement(
            new_contents,
            expr.location,
            expr.end_location.unwrap(),
        ))
    } else {
        None
    }
}
