use crate::{
    state_symbol_movement::{Movement, State, Symbol},
    turing_machine::HaltKind,
};

/// A transition for a turing machine
#[derive(Debug, Clone)]
pub enum Transition {
    NonHalting {
        /// The symbol to write at the head's current position, before moving
        symbol_to_write: Symbol,

        /// The new state to enter
        new_state: State,

        /// The movement to make
        movement: Movement,
    },
    Halting(HaltKind),
}
