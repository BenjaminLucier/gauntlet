// N-Queens — reference solution.
// One queen per row; used columns and both diagonal directions live in boolean
// arrays for O(1) safety checks, and strings are built only for complete
// boards. Time O(n!), space O(n) besides the output.

struct Search {
    n: usize,
    cols: Vec<bool>,
    down: Vec<bool>, // indexed by r + n - 1 - c
    up: Vec<bool>,   // indexed by r + c
    queens: Vec<usize>, // queens[r] = column of the queen in row r
    boards: Vec<Vec<String>>,
}

impl Search {
    fn place(&mut self, r: usize) {
        let n = self.n;
        if r == n {
            let board = self
                .queens
                .iter()
                .map(|&col| (0..n).map(|j| if j == col { 'Q' } else { '.' }).collect())
                .collect();
            self.boards.push(board);
            return;
        }
        for c in 0..n {
            let (d, u) = (r + n - 1 - c, r + c);
            if self.cols[c] || self.down[d] || self.up[u] {
                continue;
            }
            (self.cols[c], self.down[d], self.up[u]) = (true, true, true);
            self.queens.push(c);
            self.place(r + 1);
            self.queens.pop();
            (self.cols[c], self.down[d], self.up[u]) = (false, false, false);
        }
    }
}

pub fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
    let diagonals = (2 * n).saturating_sub(1);
    let mut search = Search {
        n,
        cols: vec![false; n],
        down: vec![false; diagonals],
        up: vec![false; diagonals],
        queens: Vec::with_capacity(n),
        boards: Vec::new(),
    };
    search.place(0);
    search.boards
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
