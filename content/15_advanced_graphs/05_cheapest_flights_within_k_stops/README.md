## Description

There are `n` cities labelled `0` to `n - 1`. Each flight `(from, to, price)`
is a one-way flight costing `price`.

Return the cheapest total price to get from `src` to `dst` using **at most
`max_stops` stops** (a stop is an intermediate city, so at most
`max_stops + 1` flights), or `None` if no such route exists. Getting from a
city to itself costs nothing.

## Signature

```rust
pub fn find_cheapest_price(
    n: usize,
    flights: &[(usize, usize, u32)],
    src: usize,
    dst: usize,
    max_stops: usize,
) -> Option<u32>
```

Flights are weighted edges `(from, to, price)`: cities are indices, so
`usize`; prices are never negative, so `u32`. A route has at most `n` flights
of at most `10^4` each, so its price stays below `10^6` and `u32` sums can't
overflow. "No route" is `None`,
not `-1`.

## Examples

```rust
let flights = [(0, 1, 100), (1, 2, 100), (2, 0, 100), (1, 3, 600), (2, 3, 200)];
// 0 -> 1 -> 3 costs 700 with one stop. 0 -> 1 -> 2 -> 3 costs only 400 but
// needs two stops.
assert_eq!(find_cheapest_price(4, &flights, 0, 3, 1), Some(700));
assert_eq!(find_cheapest_price(4, &flights, 0, 3, 2), Some(400));

let flights = [(0, 1, 100), (1, 2, 100), (0, 2, 500)];
assert_eq!(find_cheapest_price(3, &flights, 0, 2, 1), Some(200));
assert_eq!(find_cheapest_price(3, &flights, 0, 2, 0), Some(500)); // direct only

// Reachable, but not within the allowed stops.
assert_eq!(find_cheapest_price(3, &[(0, 1, 1), (1, 2, 1)], 0, 2, 0), None);
```

## Constraints

- `1 <= n <= 100`
- `0 <= flights.len() <= n * (n - 1)`
- `from, to < n`, `from != to`, `1 <= price <= 10^4`
- At most one flight between any ordered pair of cities
- `src, dst < n`, `max_stops < n`

## Rust notes

- `Vec<Option<u32>>` says "not reached yet" without a `u32::MAX` sentinel that
  overflows the moment you add a price to it. `Option::map` and
  `Option::min`-style matching keep the relaxation tidy.
- A round must read the previous round's prices while writing the new ones:
  `let prev = cost.clone();` (or keep two buffers and `std::mem::swap` them)
  makes that explicit, and the borrow checker won't let you read and write the
  same `Vec` through two live references anyway.
- If you try a `BinaryHeap<Reverse<(u32, usize, usize)>>` (cost, city, flights
  used) instead, a plain "visited" set is wrong here — see the hints.
- Destructure flights in the loop header: `for &(from, to, price) in flights`.
