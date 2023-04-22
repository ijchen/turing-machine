use std::{collections::HashMap, fmt::Display, rc::Rc};

use static_assertions::const_assert;

use crate::{
    state_symbol_movement::{State, Symbol},
    tape::Tape,
    transition::Transition,
};

/// The kind of halt (either accepted or rejected)
#[derive(Debug, Clone, Copy)]
pub enum HaltKind {
    Accept,
    Reject,
}

impl Display for HaltKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HaltKind::Accept => write!(f, "ACCEPT"),
            HaltKind::Reject => write!(f, "REJECT"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TuringMachineSchematic {
    initial_state: State,
    blank_symbol: Symbol,
    transitions: Rc<HashMap<(State, Symbol), Transition>>,
}

impl TuringMachineSchematic {
    /// Constructs a new TuringMachineSchematic
    pub fn new(
        initial_state: State,
        blank_symbol: Symbol,
        transitions: HashMap<(State, Symbol), Transition>,
    ) -> Self {
        Self {
            initial_state,
            blank_symbol,
            transitions: Rc::new(transitions),
        }
    }

    /// Gets the blank symbol for this turing machine schematic
    pub fn blank_symbol(&self) -> Symbol {
        self.blank_symbol
    }
}

#[derive(Debug, Clone)]
pub struct TuringMachine {
    state: State,
    tape: Tape,
    transitions: Rc<HashMap<(State, Symbol), Transition>>,
    halt_kind: Option<HaltKind>,
}

impl TuringMachine {
    /// Constructs a new TuringMachine
    pub fn new(schematic: &TuringMachineSchematic, tape: Tape) -> Self {
        Self {
            state: schematic.initial_state,
            tape,
            transitions: Rc::clone(&schematic.transitions),
            halt_kind: None,
        }
    }

    /// Performs a single step of computation. This consists of reading the
    /// symbol at the current head of the tape and performing the transition
    /// corresponding to the current state and the symbol read from the tape.
    pub fn step(&mut self) {
        if self.halted() {
            return;
        }

        let transition = self
            .transitions
            .get(&(self.state, self.tape.read()))
            .unwrap_or(&Transition::Halting(HaltKind::Reject));

        match transition {
            Transition::NonHalting {
                symbol_to_write,
                new_state,
                movement,
            } => {
                self.tape.write(*symbol_to_write);
                self.tape.make_movement(*movement);
                self.state = *new_state;
            }
            Transition::Halting(halt_kind) => {
                self.halt_kind = Some(*halt_kind);
            }
        }
    }

    /// Returns true iff the turing machine has halted
    pub fn halted(&self) -> bool {
        self.halt_kind.is_some()
    }

    /// Returns the halt kind (accepted or rejected), or None if the turing
    /// machine hasn't halted
    pub fn halt_kind(&self) -> Option<HaltKind> {
        self.halt_kind
    }

    /// Returns a string containing a visual representation of the turing
    /// machine's current tape and state
    pub fn visualize(&self) -> String {
        const SIDE_LEN: usize = 20;

        let len = SIDE_LEN * 4 + 15;
        let mut buffer = String::with_capacity(len);

        buffer += "   ";
        let start_pos = self.tape.head_pos() - SIDE_LEN as isize;
        let end_pos = self.tape.head_pos() + SIDE_LEN as isize;
        for pos in start_pos..=end_pos {
            buffer.push(*self.tape.read_pos(pos))
        }
        buffer += "   ";

        buffer += "\n";

        buffer += "...";
        for _ in 0..SIDE_LEN {
            buffer.push(' ');
        }
        buffer += "^ state: ";
        buffer.push(*self.state);
        const_assert!(SIDE_LEN - " state: ".len() >= 1);
        for _ in 0..SIDE_LEN - " state: ".len() {
            buffer.push(' ');
        }
        buffer += "...";

        // Just to make sure the math is right
        // assert!(buffer.len() == len);

        buffer
    }
}
