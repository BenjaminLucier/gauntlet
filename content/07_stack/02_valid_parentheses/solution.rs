// Valid Parentheses — reference solution.
// Push the expected closer for each opener; every closer must pop exactly that
// byte, and the stack must end empty. Time O(n), space O(n).

pub fn is_valid(s: &str) -> bool {
    let mut expected: Vec<u8> = Vec::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'(' => expected.push(b')'),
            b'[' => expected.push(b']'),
            b'{' => expected.push(b'}'),
            closer => {
                if expected.pop() != Some(closer) {
                    return false;
                }
            }
        }
    }
    expected.is_empty()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
