use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct State(char);

impl State {
    /// Constructs a new State from the given char
    pub fn new(value: char) -> Self {
        Self(value)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Symbol(char);

impl Symbol {
    /// Constructs a new Symbol from the given char
    pub fn new(value: char) -> Self {
        Self(value)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    Left,
    Right,
    Stay,
}
