A [turing machine](https://en.wikipedia.org/wiki/Turing_machine) simulation, written in Rust.

The program expects a `.turing` file path as an argument, the format of which is
overviewed below. The input (initial symbols on the tape) is then expected on
stdin - first the "home position" (where the head starts) symbol and symbols to
the right, then after a newline the symbols to the left of the home position
moving left.
```
cargo run -r -- examples/add.turing < examples/add.stdin
```

The `.turing` file contains the program the turing machine should run. It is a
text format that represents states and symbols with single characters. It
specifies the initial state, what symbol is the "blank" symbol, and the
transitions from (state, symbol) to (new symbol, movement, new state). See
[examples](examples) for detailed examples.

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
