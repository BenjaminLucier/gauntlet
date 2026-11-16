// Sum of Two Integers — reference solution.
// XOR is addition without carry, (a & b) << 1 is the carry; repeat until no
// carry remains. Done on u32, where the carry out of bit 31 is simply dropped,
// giving two's-complement wrapping. Time O(32), space O(1).

pub fn add(a: i32, b: i32) -> i32 {
    let (mut sum, mut carry) = (a.cast_unsigned(), b.cast_unsigned());
    while carry != 0 {
        (sum, carry) = (sum ^ carry, (sum & carry) << 1);
    }
    sum.cast_signed()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
