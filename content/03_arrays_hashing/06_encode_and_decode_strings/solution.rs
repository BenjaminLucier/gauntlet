// Encode and Decode Strings — reference solution.
// Length-prefix each string as "<byte length>#<bytes>". Decoding repeatedly
// splits the remaining input at the first '#', validates and parses the
// length, and slices that many bytes. Time O(total), space O(total).

use std::fmt::Write;

#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// No `#` follows the current position.
    MissingSeparator,
    /// The length before `#` is empty, not all ASCII digits, or too large.
    InvalidLength,
    /// Fewer bytes remain than the length says.
    Truncated,
    /// The length ends inside a multi-byte character.
    NotCharBoundary,
}

pub fn encode<S: AsRef<str>>(strs: &[S]) -> String {
    let total: usize = strs.iter().map(|s| s.as_ref().len() + 4).sum();
    let mut out = String::with_capacity(total);
    for s in strs {
        let s = s.as_ref();
        write!(out, "{}#", s.len()).expect("writing to a String can't fail");
        out.push_str(s);
    }
    out
}

pub fn decode(encoded: &str) -> Result<Vec<&str>, DecodeError> {
    let mut out = Vec::new();
    let mut rest = encoded;
    while !rest.is_empty() {
        let (digits, tail) = rest
            .split_once('#')
            .ok_or(DecodeError::MissingSeparator)?;
        // `parse` alone would accept a leading '+'.
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(DecodeError::InvalidLength);
        }
        let len: usize = digits.parse().map_err(|_| DecodeError::InvalidLength)?;
        if len > tail.len() {
            return Err(DecodeError::Truncated);
        }
        if !tail.is_char_boundary(len) {
            return Err(DecodeError::NotCharBoundary);
        }
        let (item, remaining) = tail.split_at(len);
        out.push(item);
        rest = remaining;
    }
    Ok(out)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
