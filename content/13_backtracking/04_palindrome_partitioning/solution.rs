// Palindrome Partitioning — reference solution.
// DP table is_pal[i][j] over chars, then backtrack over the end of the next
// piece, pushing borrowed slices of `s`. Time O(n * 2^n), space O(n^2) extra.

pub fn partition(s: &str) -> Vec<Vec<&str>> {
    struct Ctx<'a> {
        s: &'a str,
        cuts: Vec<usize>, // byte offset of every char boundary, ending with s.len()
        is_pal: Vec<Vec<bool>>,
    }

    fn go<'a>(i: usize, ctx: &Ctx<'a>, path: &mut Vec<&'a str>, out: &mut Vec<Vec<&'a str>>) {
        let n = ctx.is_pal.len();
        if i == n {
            out.push(path.clone());
            return;
        }
        for j in i..n {
            if ctx.is_pal[i][j] {
                path.push(&ctx.s[ctx.cuts[i]..ctx.cuts[j + 1]]);
                go(j + 1, ctx, path, out);
                path.pop();
            }
        }
    }

    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut is_pal = vec![vec![false; n]; n];
    for i in (0..n).rev() {
        for j in i..n {
            is_pal[i][j] = chars[i] == chars[j] && (j - i < 2 || is_pal[i + 1][j - 1]);
        }
    }
    let cuts = s.char_indices().map(|(at, _)| at).chain([s.len()]).collect();

    let ctx = Ctx { s, cuts, is_pal };
    let mut out = Vec::new();
    go(0, &ctx, &mut Vec::new(), &mut out);
    out
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
