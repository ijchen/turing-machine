use std::{collections::HashMap, path::Path};

use crate::{
    state_symbol_movement::{Movement, State, Symbol},
    transition::Transition,
    turing_machine::{HaltKind, TuringMachineSchematic},
};

fn parse_initial_state_line(line: &str, line_num: usize) -> Result<State, String> {
    let chars: Vec<char> = line.chars().collect();
    if !(chars.len() == "INITIAL STATE: X".len() && line.starts_with("INITIAL STATE: ")) {
        return Err(format!("expected `INITIAL STATE: ...` on line {line_num}"));
    }
    let initial_state = State::new(chars["INITIAL STATE: ".len()]);

    Ok(initial_state)
}

fn parse_blank_symbol_line(line: &str, line_num: usize) -> Result<Symbol, String> {
    let chars: Vec<char> = line.chars().collect();
    if !(chars.len() == "BLANK SYMBOL: [X]".len()
        && line.starts_with("BLANK SYMBOL: [")
        && line.ends_with(']'))
    {
        return Err(format!("expected `BLANK SYMBOL: [...]` on line {line_num}"));
    }
    let blank_symbol = Symbol::new(chars["BLANK SYMBOL: [".len()]);

    Ok(blank_symbol)
}

fn parse_transitions_line(line: &str, line_num: usize) -> Result<(), String> {
    if line != "TRANSITIONS:" {
        return Err(format!("expected `TRANSITIONS:` on line {line_num}"));
    }

    Ok(())
}

fn parse_transition_line(
    line: &str,
    line_num: usize,
    transitions: &mut HashMap<(State, Symbol), Transition>,
) -> Result<(), String> {
    let chars: Vec<char> = line.chars().collect();
    if chars.len() < "1x: y>2".len() || chars[2..4] != [':', ' '] {
        return Err(format!("malformed transition on line {line_num}"));
    }

    let from_state = State::new(chars[0]);
    let from_symbol = Symbol::new(chars[1]);

    // Ensure we haven't already encountered this transition
    if transitions.contains_key(&(from_state, from_symbol)) {
        return Err(format!("duplicate transition on line {line_num} from state '{from_state}' and symbol '{from_symbol}'"));
    }

    let transition = match (chars[4], chars[5], chars[6]) {
        ('A', 'C', 'C') => Transition::Halting(HaltKind::Accept),
        ('R', 'E', 'J') => Transition::Halting(HaltKind::Reject),
        (symbol_to_write, movement, new_state) => Transition::NonHalting {
            symbol_to_write: Symbol::new(symbol_to_write),
            new_state: State::new(new_state),
            movement: match movement {
                '<' => Movement::Left,
                '>' => Movement::Right,
                '=' => Movement::Stay,
                movement => return Err(format!("invalid movement direction '{movement}'")),
            },
        },
    };

    transitions.insert((from_state, from_symbol), transition);

    Ok(())
}

/// Constructs a new TuringMachineSchematic from a .turing file
///
/// # Errors
/// - If the file at the given path cannot be read
/// - If the input data is malformed
pub fn parse_turing_program<P: AsRef<Path>>(path: P) -> Result<TuringMachineSchematic, String> {
    // Read the file contents to a string
    let file_contents = std::fs::read_to_string(path).map_err(|err| err.to_string())?;

    // Filter out blank and comment lines, and track line numbers for error
    // messages
    let mut lines = file_contents
        .lines()
        .enumerate()
        .filter(|(_, line)| !(line.is_empty() || line.starts_with('#')))
        .map(|(line_index, line)| (line, line_index + 1));

    // Parse the initial state line
    let (initial_state_line, initial_state_line_num) = lines
        .next()
        .ok_or_else(|| "missing expected `INITIAL STATE: ...` line".to_string())?;
    let initial_state = parse_initial_state_line(initial_state_line, initial_state_line_num)?;

    // Parse the blank symbol line
    let (blank_symbol_line, blank_symbol_line_num) = lines
        .next()
        .ok_or_else(|| "missing expected `BLANK SYMBOL: [...]` line".to_string())?;
    let blank_symbol = parse_blank_symbol_line(blank_symbol_line, blank_symbol_line_num)?;

    // Parse the TRANSITIONS line
    let (transitions_line, transitions_line_number) = lines
        .next()
        .ok_or_else(|| "missing expected `TRANSITIONS:` line".to_string())?;
    parse_transitions_line(transitions_line, transitions_line_number)?;

    // Parse the transitions
    let mut transitions: HashMap<(State, Symbol), Transition> = HashMap::new();
    for (line, line_number) in lines {
        parse_transition_line(line, line_number, &mut transitions)?;
    }

    // Construct and return the turing machine schematic
    Ok(TuringMachineSchematic::new(
        initial_state,
        blank_symbol,
        transitions,
    ))
}

/// Reads the contents of a tape from stdin
///
/// The expected input is the symbol for the home position, followed immediately
/// by the symbols to the right of the home position from left to right,
/// followed by a newline character (either \n or \r\n), followed by the symbols
/// to the left of the home position from right to left, then end of input.
pub fn read_tape() -> Result<(Vec<Symbol>, Vec<Symbol>), String> {
    println!("Enter the input symbols from (and including) the home position moving to the right:");
    let right_symbols = match std::io::stdin().lines().next() {
        Some(Ok(line)) => Ok(line),
        Some(Err(err)) => Err(err.to_string()),
        None => Err("failed to read line from stdin".to_string()),
    }?;
    println!("Enter the input symbols from (but excluding) the home position moving to the left:");
    let left_symbols = match std::io::stdin().lines().next() {
        Some(Ok(line)) => Ok(line),
        Some(Err(err)) => Err(err.to_string()),
        None => Err("failed to read line from stdin".to_string()),
    }?;

    Ok((
        right_symbols.chars().map(Symbol::new).collect(),
        left_symbols.chars().map(Symbol::new).collect(),
    ))
}
