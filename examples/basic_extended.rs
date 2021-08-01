use sfsm::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Up {}
#[derive(Debug)]
pub struct Down {}
#[derive(Debug)]
pub struct Grounded {}
#[derive(Debug)]
pub struct Move<T> {
    phantom: PhantomData<T>
}

add_state_machine!(
    #[derive(Debug)]                            // Attributes for the generated struct can be defined
    pub Rocket,                                 // Name of the state machine. Accepts a visibility modifier.
    Move<Grounded>,                             // The state machine will start at the count down
    [Move<Grounded>, Move<Up>, Move<Down>],     // All possible states
    [
        Move<Grounded> => Move<Up>,
        Move<Up> => Move<Down>
    ]
);

impl State for Move<Grounded> {
}
impl Into<Move<Up>> for Move<Grounded> {
    fn into(self) -> Move<Up> { Move { phantom: PhantomData }}
}
impl Transition<Move<Up>> for Move<Grounded> {
    fn guard(&self) -> TransitGuard {
        return TransitGuard::Transit;
    }
}

impl State for Move<Up> {
}
impl Into<Move<Down>> for Move<Up> {
    fn into(self) -> Move<Down> { Move { phantom: PhantomData }}
}
impl Transition<Move<Down>> for Move<Up> {
    fn guard(&self) -> TransitGuard {
        return TransitGuard::Transit;
    }
}

impl State for Move<Down> {
}

fn run_basic_extended_example() -> Result<(), SfsmError> {

    let mut rocket = Rocket::new();

    let standstill = Move {phantom: PhantomData};
    rocket.start(standstill)?;

    assert!(IsState::<Move<Grounded>>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<Move<Up>>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<Move<Down>>::is_state(&rocket));
    rocket.step()?;

    let stopped_state = rocket.stop()?;
    match stopped_state {
        match_state_entry!(Rocket, Move<Down>, exit_state) => {
            // Access "exit_state" here
            println!("Exit state: {:?}", exit_state);
            assert!(true);
        }
        _ => {
            assert!(false);
        }
    }

    Ok(())
}

fn main() {
    run_basic_extended_example().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::run_basic_extended_example;

    #[test]
    fn basic_extended_example() {
        run_basic_extended_example().unwrap();
    }
}