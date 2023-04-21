use crate::state_symbol_movement::{Movement, Symbol};

#[derive(Debug, Clone)]
pub struct Tape {
    /// The "blank" symbol for this tape
    blank_symbol: Symbol,

    /// The symbols from the home position moving to the right
    /// The home position is at index 0
    right_symbols: Vec<Symbol>,
    /// The symbols to the left of the home position, not including it. Index
    /// 0 is the symbol immediately to the left of home position, and each next
    /// element in the vec is one to the left on the conceptual tape
    left_symbols: Vec<Symbol>,
    /// The position of the head of the tape. 0 indicates the home position,
    /// positive numbers are to the right of the home position, and negative
    /// numbers are to the left of the home position. This index must always be
    /// in-bounds on the tape.
    head_pos: isize,
}

impl Tape {
    /// Constructs a new Tape using the given blank symbol. By default, the tape
    /// is filled with only blank symbols, and the head is at the home position.
    pub fn new(
        blank_symbol: Symbol,
        mut right_symbols: Vec<Symbol>,
        left_symbols: Vec<Symbol>,
    ) -> Self {
        // Ensure the home position (where the head starts) is in-bounds
        if right_symbols.is_empty() {
            right_symbols.push(blank_symbol);
        }

        Self {
            blank_symbol,
            right_symbols,
            left_symbols,
            head_pos: 0,
        }
    }

    /// Reads the symbol at the head's current position on the tape
    pub fn read(&self) -> Symbol {
        if self.head_pos.is_negative() {
            self.left_symbols[(-self.head_pos - 1) as usize]
        } else {
            self.right_symbols[self.head_pos as usize]
        }
    }

    /// Writes a symbol at the head's current position on the tape
    pub fn write(&mut self, symbol: Symbol) {
        if self.head_pos.is_negative() {
            self.left_symbols[(-self.head_pos - 1) as usize] = symbol
        } else {
            self.right_symbols[self.head_pos as usize] = symbol
        }
    }

    /// Moves the head's current position on the tape
    pub fn make_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Left => {
                if self.head_pos - 1 < -(self.left_symbols.len() as isize) {
                    self.left_symbols.push(self.blank_symbol);
                }
                self.head_pos -= 1;
            }
            Movement::Right => {
                if self.head_pos + 1 >= self.right_symbols.len() as isize {
                    self.right_symbols.push(self.blank_symbol);
                }
                self.head_pos += 1;
            }
            Movement::Stay => { /* Nothing to do here */ }
        }
    }
}
