// Cheapest Flights Within K Stops — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn find_cheapest_price(
    n: usize,
    flights: &[(usize, usize, u32)],
    src: usize,
    dst: usize,
    max_stops: usize,
) -> Option<u32> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
