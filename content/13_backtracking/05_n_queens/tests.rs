// Tests for N-Queens — shared by template.rs, solution.rs and your attempt.
use super::*;

const COUNTS: [usize; 10] = [1, 1, 0, 0, 2, 10, 4, 40, 92, 352];

fn normalize<T: Ord>(mut v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    v.sort();
    v
}

/// Checks shape, characters, and that no two queens attack each other.
fn validate_board(n: usize, b: &[String]) {
    assert_eq!(b.len(), n, "board must have {n} rows: {b:?}");
    let mut queens = Vec::new();
    for (r, row) in b.iter().enumerate() {
        assert_eq!(row.chars().count(), n, "row {r} must have {n} chars: {b:?}");
        assert!(row.chars().all(|c| c == 'Q' || c == '.'), "only 'Q' and '.' allowed: {b:?}");
        let qs: Vec<usize> = row.match_indices('Q').map(|(i, _)| i).collect();
        assert_eq!(qs.len(), 1, "row {r} must hold exactly one queen: {b:?}");
        queens.push((r, qs[0]));
    }
    for (i, &(r1, c1)) in queens.iter().enumerate() {
        for &(r2, c2) in &queens[i + 1..] {
            assert_ne!(c1, c2, "queens share column {c1}: {b:?}");
            assert_ne!(r1.abs_diff(r2), c1.abs_diff(c2), "queens share a diagonal: {b:?}");
        }
    }
}

fn validate_all(n: usize) -> Vec<Vec<String>> {
    let got = normalize(solve_n_queens(n));
    for b in &got {
        validate_board(n, b);
    }
    for w in got.windows(2) {
        assert_ne!(w[0], w[1], "duplicate board for n={n}: {:?}", w[0]);
    }
    assert_eq!(got.len(), COUNTS[n], "wrong number of solutions for n={n}");
    got
}

/// Try every column permutation and keep the non-attacking ones.
fn brute_solve(n: usize) -> Vec<Vec<String>> {
    fn perms(cur: &mut Vec<usize>, used: &mut [bool], out: &mut Vec<Vec<String>>) {
        let n = used.len();
        if cur.len() == n {
            let ok = (0..n).all(|i| (i + 1..n).all(|j| cur[i].abs_diff(cur[j]) != j - i));
            if ok {
                out.push(
                    cur.iter()
                        .map(|&c| (0..n).map(|j| if j == c { 'Q' } else { '.' }).collect())
                        .collect(),
                );
            }
            return;
        }
        for c in 0..n {
            if !used[c] {
                used[c] = true;
                cur.push(c);
                perms(cur, used, out);
                cur.pop();
                used[c] = false;
            }
        }
    }
    let mut out = Vec::new();
    perms(&mut Vec::new(), &mut vec![false; n], &mut out);
    normalize(out)
}

#[test]
fn example_1() {
    assert_eq!(
        normalize(solve_n_queens(4)),
        vec![vec!["..Q.", "Q...", "...Q", ".Q.."], vec![".Q..", "...Q", "Q...", "..Q."]]
    );
}

#[test]
fn example_2() {
    assert_eq!(solve_n_queens(1), vec![vec!["Q"]]);
}

#[test]
fn n0_has_one_empty_board() {
    assert_eq!(solve_n_queens(0), vec![Vec::<String>::new()]);
}

#[test]
fn n2_and_n3_have_no_solution() {
    assert!(solve_n_queens(2).is_empty());
    assert!(solve_n_queens(3).is_empty());
}

#[test]
fn n5_valid_and_count() {
    validate_all(5);
}

#[test]
fn n6_valid_and_count() {
    validate_all(6);
}

#[test]
fn n7_valid_and_count() {
    validate_all(7);
}

#[test]
fn n8_valid_and_count() {
    validate_all(8);
}

#[test]
fn n9_valid_and_count() {
    validate_all(9);
}

#[test]
fn both_diagonal_directions_are_checked() {
    // Checking only one diagonal direction yields extra boards for n=4.
    let got = solve_n_queens(4);
    for b in &got {
        validate_board(4, b);
    }
    assert_eq!(got.len(), 2);
}

#[test]
fn usize_diagonal_index_does_not_underflow() {
    // r - c goes negative for c > r; the queen in row 0 may sit in the last
    // column. Any n >= 4 exercises it, n = 5 has such a board.
    let got = solve_n_queens(5);
    assert!(got.iter().any(|b| b[0].ends_with('Q')), "got={got:?}");
}

#[test]
fn matches_permutation_brute_force() {
    for n in 0..=7 {
        assert_eq!(normalize(solve_n_queens(n)), brute_solve(n), "n={n}");
    }
}

#[test]
fn n6_exact_boards() {
    assert_eq!(
        normalize(solve_n_queens(6)),
        normalize(vec![
            vec![".Q....", "...Q..", ".....Q", "Q.....", "..Q...", "....Q."],
            vec!["..Q...", ".....Q", ".Q....", "....Q.", "Q.....", "...Q.."],
            vec!["...Q..", "Q.....", "....Q.", ".Q....", ".....Q", "..Q..."],
            vec!["....Q.", "..Q...", "Q.....", ".....Q", "...Q..", ".Q...."],
        ])
    );
}

#[test]
fn n8_is_symmetric_under_mirroring() {
    // Mirroring every row of a solution gives another solution.
    let got = normalize(solve_n_queens(8));
    let mirrored = normalize(
        got.iter()
            .map(|b| b.iter().map(|r| r.chars().rev().collect::<String>()).collect())
            .collect(),
    );
    assert_eq!(got, mirrored);
}

#[test]
fn n8_is_symmetric_under_transposition() {
    // Swapping rows and columns of a solution gives another solution.
    let got = normalize(solve_n_queens(8));
    let transposed = normalize(
        got.iter()
            .map(|b| {
                let rows: Vec<&[u8]> = b.iter().map(|r| r.as_bytes()).collect();
                (0..8)
                    .map(|c| rows.iter().map(|r| char::from(r[c])).collect::<String>())
                    .collect()
            })
            .collect(),
    );
    assert_eq!(got, transposed);
}
