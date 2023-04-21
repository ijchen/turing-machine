//! A turing machine simulation, written in Rust.
//! https://en.wikipedia.org/wiki/Turing_machine

#![warn(missing_docs, missing_debug_implementations)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::{
    parser::parse_turing_program, state_symbol_movement::Symbol, tape::Tape,
    turing_machine::TuringMachine,
};

mod parser;
mod state_symbol_movement;
mod tape;
mod transition;
mod turing_machine;

fn main() {
    // TODO read the file path from env args
    let schematic = match parse_turing_program("examples/accept.turing") {
        Ok(schematic) => schematic,
        Err(msg) => {
            eprintln!("Something went wrong: {msg}");
            return;
        }
    };

    // TODO read the tape from stdin
    let mut right_symbols: Vec<Symbol> = Vec::new();
    let mut left_symbols: Vec<Symbol> = Vec::new();
    right_symbols.push(schematic.blank_symbol());
    // 3 is less than or equal to 7, so this should ACCEPT
    let (a, b) = (3, 7);
    right_symbols.extend(std::iter::repeat(Symbol::new('x')).take(a));
    left_symbols.extend(std::iter::repeat(Symbol::new('x')).take(b));
    let tape = Tape::new(schematic.blank_symbol(), right_symbols, left_symbols);

    let mut turing_machine = TuringMachine::new(&schematic, tape);

    while !turing_machine.halted() {
        turing_machine.step();
    }

    println!(
        "Turing machine halted in a(n) {} state",
        turing_machine.halt_kind().unwrap()
    );
}
