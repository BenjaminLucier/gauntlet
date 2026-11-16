// Multiply Strings — reference solution.
// Validate both inputs into digit vectors, then positional long multiplication:
// digit i * digit j lands at index i + j + 1 of an (m + n)-length array, with
// the carry pushed to i + j. Strip leading zeros. Time O(m*n), space O(m + n).

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberError {
    Empty,
    InvalidDigit(char),
}

fn digits(s: &str) -> Result<Vec<u32>, NumberError> {
    if s.is_empty() {
        return Err(NumberError::Empty);
    }
    s.chars()
        .map(|c| c.to_digit(10).ok_or(NumberError::InvalidDigit(c)))
        .collect()
}

pub fn multiply(a: &str, b: &str) -> Result<String, NumberError> {
    let a = digits(a)?;
    let b = digits(b)?;
    let mut res = vec![0u32; a.len() + b.len()];
    for (i, &da) in a.iter().enumerate().rev() {
        for (j, &db) in b.iter().enumerate().rev() {
            let sum = res[i + j + 1] + da * db;
            res[i + j + 1] = sum % 10;
            res[i + j] += sum / 10;
        }
    }
    let product: String = res
        .iter()
        .skip_while(|&&d| d == 0)
        .map(|&d| char::from_digit(d, 10).expect("every slot holds a single digit"))
        .collect();
    Ok(if product.is_empty() {
        "0".to_string()
    } else {
        product
    })
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
