//! A turing machine simulation, written in Rust.
//! https://en.wikipedia.org/wiki/Turing_machine

#![warn(missing_docs, missing_debug_implementations)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::{
    parser::{parse_turing_program, read_tape},
    tape::Tape,
    turing_machine::TuringMachine,
};

mod parser;
mod state_symbol_movement;
mod tape;
mod transition;
mod turing_machine;

fn parse_args() -> Result<String, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        return Err("invalid arguments - expected a single file path".to_string());
    }

    Ok(args.into_iter().nth(1).unwrap())
}

fn main() {
    let path = match parse_args() {
        Ok(path) => path,
        Err(msg) => {
            eprintln!("{msg}");
            return;
        }
    };

    let schematic = match parse_turing_program(path) {
        Ok(schematic) => schematic,
        Err(msg) => {
            eprintln!("Something went wrong: {msg}");
            return;
        }
    };

    let (right_symbols, left_symbols) = match read_tape() {
        Ok(symbols) => symbols,
        Err(msg) => {
            eprintln!("Something went wrong: {msg}");
            return;
        }
    };
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
