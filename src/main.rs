use std::str::FromStr;
use std::io::prelude::*;
use std::io;
use std::io::Result;

fn main() {
    let input = Input::from_stdin();
    let mut state_machine = StateMachine::new(State::X);

    for i in input.buf {
        state_machine.step(i);
    }

    let final_state = state_machine.current();

    println!("Final State: {:?}", final_state);
}

#[derive(Clone, Copy, Debug)]
enum State {
    X,
    Y,
    Z,
}

#[derive(Debug)]
struct StateMachine {
    current: State,
}

impl StateMachine {
    fn new(initial: State) -> StateMachine {
        StateMachine {
            current: initial,
        }
    }

    fn step(&mut self, input: bool) {
        match self.current {
            State::X => {
                if input {
                    // No change
                } else {
                    self.current = State::Y;
                }
            },
            State::Y => {
                if input {
                    self.current = State::X;
                } else {
                    self.current = State::Z;
                }
            },
            State::Z => {
                if input {
                    self.current = State::Y;
                } else {
                    // No change
                }
            },
        }
    }

    fn current(&self) -> State {
        self.current
    }
}

// Prompt for input on stdout, reading in a line from stdin.
fn prompt<T>(sym: &str) -> Result<T>
    where T: FromStr
{
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    try!(stdout.write(sym.as_bytes()));
    try!(stdout.flush());

    let mut buf = String::new();
    try!(stdin.read_line(&mut buf));
    buf.pop(); // Remove newline.

    match buf.parse() {
        Ok(ret) => Ok(ret),
        Err(..) => Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input")),
    }
}

#[derive(Debug)]
struct Input {
    buf: Vec<bool>,
}

impl Input {
    fn from_stdin() -> Input {
        loop {
            match prompt::<Input>("> ") {
                Ok(i) => return i,
                Err(..) => println!("Please enter a valid input string.\n"),
            }
        }
    }
}


impl FromStr for Input {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut buf = Vec::new();
        for ch in s.chars() {
            if ch == '1' {
                buf.push(true);
            } else if ch == '0' {
                buf.push(false);
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid input"));
            }
        }
        Ok(Input { buf: buf })
    }
}
