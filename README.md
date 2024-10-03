# intervalues
Efficient combining of (valued) intervals of numbers for various applications.

## Installing

```sh
$ cargo add intervalues
```

Alternatively, you can edit your `Cargo.toml` directly and run `cargo update`:

```toml
[dependencies]
intervalues = "0.2.0"
```

## Getting started

A simple example to show how two valued intervals can be combined. In this case, one interval is on the interval [0, 2]
with a value of 1, and the other interval is on the interval [1, 3] with a value of 2. The result of combining is three
intervals: [0, 1] with a value of 1, [1, 2] with a value of 3, and [2, 3] with a value of 2. (Note that for now the 
distinction between open and closed intervals is ignored.)

```rust
use std::collections::HashMap;
use intervalues;

// Two intervals, from 0 to 2 with count 1 and 1 to 3 with count 2
let input: Vec<[isize; 3]> = vec!([0, 2, 1], [1, 3, 2]);
let out: HashMap<(isize, isize), isize> = intervalues::combine_intervals_values(input);

// 'out' = {(0, 1): 1, (2, 3): 2, (1, 2): 3}
assert_eq!(out[&(0, 1)], 1);
assert_eq!(out[&(1, 2)], 3);
assert_eq!(out[&(2, 3)], 2);
```

See the [docs](https://docs.rs/intervalues/) for more functions and details.

## Motivation & goal
This package is the Rust implementation of the Python package [intervalues](https://github.com/debruijn/intervalues).
The Python package contains organizational functionality around a core interval combination algorithm. In order to
explore speeding up this algorithm, this Rust implementation has been started. For now, the main goal is on coming to 
a stable implementation for that core implementation, but reimplementing some of the convenience wrappers is open to
discussion. Feel free to let me know if that would interest you!
