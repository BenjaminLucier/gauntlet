// Tests for Cheapest Flights Within K Stops — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::HashSet;

type Flight = (usize, usize, u32);

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// Exhaustive DFS over every route of at most max_stops + 1 flights (tiny inputs only).
fn brute_dfs(n: usize, flights: &[Flight], src: usize, dst: usize, max_stops: usize) -> Option<u32> {
    fn go(u: usize, dst: usize, left: usize, cost: u64, adj: &[Vec<(usize, u64)>], best: &mut Option<u64>) {
        if u == dst {
            *best = Some(best.map_or(cost, |b| b.min(cost)));
            return;
        }
        if left == 0 {
            return;
        }
        for &(v, p) in &adj[u] {
            go(v, dst, left - 1, cost + p, adj, best);
        }
    }
    let mut adj = vec![Vec::new(); n];
    for &(u, v, p) in flights {
        adj[u].push((v, u64::from(p)));
    }
    let mut best = None;
    go(src, dst, max_stops + 1, 0, &adj, &mut best);
    best.map(|b| u32::try_from(b).expect("fits"))
}

/// Layered DP: layer[v] = cheapest cost to v using exactly j flights.
fn brute_layers(n: usize, flights: &[Flight], src: usize, dst: usize, max_stops: usize) -> Option<u32> {
    const INF: u64 = u64::MAX / 4;
    let mut layer = vec![INF; n];
    layer[src] = 0;
    let mut best = layer[dst];
    for _ in 0..=max_stops {
        let mut next = vec![INF; n];
        for &(u, v, p) in flights {
            if layer[u] < INF {
                next[v] = next[v].min(layer[u] + u64::from(p));
            }
        }
        best = best.min(next[dst]);
        layer = next;
    }
    (best < INF).then(|| u32::try_from(best).expect("fits"))
}

fn random_flights(rng: &mut Rng, n: usize, m: usize, max_p: u64) -> Vec<Flight> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    while out.len() < m.min(n * (n - 1)) {
        let u = rng.below(n as u64) as usize;
        let v = rng.below(n as u64) as usize;
        if u != v && seen.insert((u, v)) {
            out.push((u, v, 1 + rng.below(max_p) as u32));
        }
    }
    out
}

#[test]
fn example_1() {
    let fl = [(0, 1, 100), (1, 2, 100), (2, 0, 100), (1, 3, 600), (2, 3, 200)];
    assert_eq!(find_cheapest_price(4, &fl, 0, 3, 1), Some(700));
    assert_eq!(find_cheapest_price(4, &fl, 0, 3, 2), Some(400));
}

#[test]
fn example_2() {
    assert_eq!(find_cheapest_price(3, &[(0, 1, 100), (1, 2, 100), (0, 2, 500)], 0, 2, 1), Some(200));
}

#[test]
fn example_3() {
    assert_eq!(find_cheapest_price(3, &[(0, 1, 100), (1, 2, 100), (0, 2, 500)], 0, 2, 0), Some(500));
}

#[test]
fn example_4_too_many_stops_is_none() {
    assert_eq!(find_cheapest_price(3, &[(0, 1, 1), (1, 2, 1)], 0, 2, 0), None);
}

#[test]
fn no_flights() {
    assert_eq!(find_cheapest_price(2, &[], 0, 1, 1), None);
}

#[test]
fn unreachable_destination() {
    assert_eq!(find_cheapest_price(3, &[(0, 1, 5), (2, 1, 5)], 0, 2, 2), None);
}

#[test]
fn flights_are_one_way() {
    assert_eq!(find_cheapest_price(2, &[(1, 0, 5)], 0, 1, 1), None);
    assert_eq!(find_cheapest_price(2, &[(1, 0, 5)], 1, 0, 0), Some(5));
}

#[test]
fn source_equals_destination_costs_nothing() {
    assert_eq!(find_cheapest_price(1, &[], 0, 0, 0), Some(0));
    assert_eq!(find_cheapest_price(2, &[(0, 1, 3), (1, 0, 3)], 0, 0, 1), Some(0));
}

#[test]
fn reachable_only_with_too_many_stops() {
    let fl = [(0, 1, 1), (1, 2, 1), (2, 3, 1)];
    assert_eq!(find_cheapest_price(4, &fl, 0, 3, 1), None);
    assert_eq!(find_cheapest_price(4, &fl, 0, 3, 2), Some(3));
}

#[test]
fn in_place_relaxation_trap() {
    // Edges listed along the chain: relaxing in place chains three flights in
    // one round and returns 3 instead of the direct 100.
    let fl = [(0, 1, 1), (1, 2, 1), (2, 3, 1), (0, 3, 100)];
    assert_eq!(find_cheapest_price(4, &fl, 0, 3, 0), Some(100));
}

#[test]
fn dijkstra_visited_trap() {
    // City 2 is cheapest via 0->1->2 (2 flights), but with one stop the route
    // must use the pricier 0->2 (1 flight) and then 2->3.
    let fl = [(0, 1, 1), (1, 2, 1), (0, 2, 5), (2, 3, 1)];
    assert_eq!(find_cheapest_price(4, &fl, 0, 3, 1), Some(6));
    assert_eq!(find_cheapest_price(4, &fl, 0, 3, 2), Some(3));
}

#[test]
fn cycles_do_not_help() {
    let fl = [(0, 1, 1), (1, 0, 1), (1, 2, 10)];
    assert_eq!(find_cheapest_price(3, &fl, 0, 2, 2), Some(11));
}

#[test]
fn large_k_behaves_like_shortest_path() {
    let fl = [(0, 1, 2), (1, 2, 2), (2, 3, 2), (0, 3, 7), (1, 3, 5)];
    assert_eq!(find_cheapest_price(4, &fl, 0, 3, 3), Some(6));
}

#[test]
fn max_prices_on_longest_route() {
    // 100 cities in a line, every flight 10^4: the sum stays well inside u32.
    let fl: Vec<Flight> = (0..99).map(|i| (i, i + 1, 10_000)).collect();
    assert_eq!(find_cheapest_price(100, &fl, 0, 99, 98), Some(990_000));
    assert_eq!(find_cheapest_price(100, &fl, 0, 99, 97), None);
}

#[test]
fn randomized_vs_exhaustive_dfs() {
    let mut rng = Rng(0x0787_0787_ABCD_EF01);
    for _ in 0..400 {
        let n = 2 + rng.below(5) as usize;
        let m = rng.below((n * (n - 1)) as u64 + 1) as usize;
        let flights = random_flights(&mut rng, n, m, 20);
        let src = rng.below(n as u64) as usize;
        let mut dst = rng.below(n as u64) as usize;
        while dst == src {
            dst = rng.below(n as u64) as usize;
        }
        let k = rng.below(n as u64) as usize;
        let expected = brute_dfs(n, &flights, src, dst, k);
        assert_eq!(
            find_cheapest_price(n, &flights, src, dst, k),
            expected,
            "n = {n}, flights = {flights:?}, src = {src}, dst = {dst}, max_stops = {k}"
        );
    }
}

#[test]
fn large_random_vs_layered_dp() {
    let mut rng = Rng(787);
    for k in [0, 1, 3, 10, 99] {
        let flights = random_flights(&mut rng, 100, 4950, 10_000);
        let src = rng.below(100) as usize;
        let dst = (src + 1 + rng.below(99) as usize) % 100;
        let expected = brute_layers(100, &flights, src, dst, k);
        assert_eq!(find_cheapest_price(100, &flights, src, dst, k), expected, "max_stops = {k}");
    }
}

#[test]
fn large_dense_complete_graph() {
    // Every ordered pair has a flight: 9900 flights, 99 rounds.
    let mut rng = Rng(4242);
    let flights = random_flights(&mut rng, 100, 9900, 10_000);
    for k in [0, 2, 99] {
        let expected = brute_layers(100, &flights, 0, 99, k);
        assert_eq!(find_cheapest_price(100, &flights, 0, 99, k), expected, "max_stops = {k}");
    }
}

#[test]
fn large_chain_with_shortcuts() {
    // 0 -> 1 -> ... -> 99 costs 1 each; shortcuts i -> i+10 cost 15.
    let mut flights: Vec<Flight> = (0..99).map(|i| (i, i + 1, 1)).collect();
    flights.extend((0..90).map(|i| (i, i + 10, 15)));
    assert_eq!(find_cheapest_price(100, &flights, 0, 99, 98), Some(99));
    // The fewest flights is 18 (nine shortcuts to 90, then nine hops).
    assert_eq!(find_cheapest_price(100, &flights, 0, 99, 17), Some(9 * 15 + 9));
    assert_eq!(find_cheapest_price(100, &flights, 0, 99, 16), None);
    let expected = brute_layers(100, &flights, 0, 99, 40);
    assert_eq!(find_cheapest_price(100, &flights, 0, 99, 40), expected);
}
