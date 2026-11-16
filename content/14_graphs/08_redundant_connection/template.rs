// Redundant Connection — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

pub fn find_redundant_connection(edges: &[(usize, usize)]) -> Option<(usize, usize)> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
