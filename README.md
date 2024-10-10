# intervalues
Efficient combining of (valued) intervals of numbers for various applications.

## Installing

```sh
$ cargo add intervalues
```

Alternatively, you can edit your `Cargo.toml` directly and run `cargo update`:

```toml
[dependencies]
intervalues = "0.3.0"
```

## Getting started

A simple example to show how two valued intervals can be combined. In this case, one interval is on the interval [0, 2]
with a value of 1, and the other interval is on the interval [1, 3] with a value of 2. The result of combining is three
intervals: [0, 1] with a value of 1, [1, 2] with a value of 3, and [2, 3] with a value of 2. (Note that for now the 
distinction between open and closed intervals is ignored.)

```rust
use std::collections::HashMap;
use intervalues;

// Two intervals, from 0 to 2 with value 1 and 1 to 3 with value 2
let input: Vec<[i64; 3]> = vec!([0, 2, 1], [1, 3, 2]);
let input = input.iter()
     .map(|x| Interval::new(x[0], x[1], x[2]))
     .collect();
let out: IntervalCollection<i64,i64> = intervalues::combine_intervals(input);

// 'out' = IntervalCollection { intervals: [[0;1]x1, [1;2]x3, [2;3]x2] }

```

Note that the input can be anything that implements the Num trait (and some other sensible traits). This package has
been tested with the standard library integer variable types (isize, usize, i32, etc) and also with 
rust_decimal::Decimal and intfloat::IntFloat. Standard library floats are not supported due to not being Hashable, but 
can be converted to Decimal and IntFloat.

Also note that the variable types used for the interval bounds and for the value/count don't have to be of the same
type, e.g. one can do `Interval::new(5, 10, Decimal::from(12.3))`.

See the [docs](https://docs.rs/intervalues/) for more functions and details. Feel free to open an issue in case some
things are unclear.

## Motivation & goal
This package is the Rust implementation of the Python package [intervalues](https://github.com/debruijn/intervalues).
The Python package contains organizational functionality around a core interval combination algorithm. In order to
explore speeding up this algorithm, this Rust implementation has been started. For now, the main goal is on coming to 
a stable implementation for the most important parts of the Python package, but if there are other ideas, then please
let me know!

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as 
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions. 