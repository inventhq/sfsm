use sfsm::*;

// A very basic usage of the crate

// First define all the state structs
#[derive(Debug)]
pub struct MoveUp {}
pub struct Grounded {}

// Then define the whole state machine
add_state_machine!(
    Rocket,                      // Name of the state machine. Accepts a visibility modifier.
    Grounded,                    // The initial state the state machine will start in
    [Grounded, MoveUp],          // All possible states
    [
        Grounded => MoveUp,      // All transitions
    ]
);

// Now the functionality for the states has to be implemented
// Every state can define an entry, execute and exit function.
// The execute function will be called multiple times, while the entry function will only be called
// initially and the exit when the state is left.
impl State for Grounded {
    fn entry(&mut self) {
        println!("Standstill: Entry")
    }
    fn execute(&mut self) {
        println!("Standstill: Execute");
    }
    fn exit(&mut self) {
        println!("Standstill: Exit");
    }
}
// Then implement the transitions.
// Each transition can define an entry, execute and exit function and must define the guard function.
// Additionally a Into implementation has to be provided so that each state can be transformed
// Into the next one.
impl Into<MoveUp> for Grounded {
    fn into(self) -> MoveUp { MoveUp {} }
}
impl Transition<MoveUp> for Grounded {
    fn entry(&mut self) {
        println!("Standstill => MoveUp: Entry")
    }
    fn execute(&mut self) {
        println!("Standstill => MoveUp: Execute");
    }
    fn exit(&mut self) {
        println!("Standstill => MoveUp: Exit");
    }
    fn guard(&self) -> TransitGuard {
        println!("Standstill => MoveUp: Guard");
        return TransitGuard::Transit;
    }
}

impl State for MoveUp {
    fn entry(&mut self) {
        println!("MoveUp: Entry")
    }
    fn execute(&mut self) {
        println!("MoveUp: Execute");
    }
    fn exit(&mut self) {
        println!("MoveUp: Exit");
    }
}

fn run_basic_example() -> Result<(), SfsmError> {

    // The initial state has to be manually created
    let standstill = Grounded {};
    // Then the state machine can be started. Note: This corresponds to the name given in the
    // add_state_machine macro.
    let mut rocket = Rocket::new(standstill);

    // The IsState trait can be used to check in which state the state machine is in.
    assert!(IsState::<Grounded>::is_state(&rocket));
    // Run the state machine with .step().
    rocket.step()?;

    assert!(IsState::<MoveUp>::is_state(&rocket));

    // If the state machine has to be stopped and the data recovered, it can be done so by calling .stop();
    let stopped_state = rocket.stop()?;
    match stopped_state {
        // If you don't want to type out the state enum use the match_state_entry! macro here
        // It generates the following: [SFSM_NAME]States::[STATE_NAME_AND_TYPES]State(state)
        // Otherwise you have to type it out manually with the given schema.
        match_state_entry!(Rocket, MoveUp, exit_state) => {
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
    run_basic_example().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::run_basic_example;

    #[test]
    fn simple_example() {
        run_basic_example().unwrap();
    }
}