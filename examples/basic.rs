use sfsm::*;

// A very basic usage of the crate

// First define all the state structs
#[derive(Debug)]
pub struct Launch {}
pub struct WaitForLaunch {}

// Then define the whole state machine
add_state_machine!(
    Rocket,                      // Name of the state machine. Accepts a visibility modifier.
    WaitForLaunch,                    // The initial state the state machine will start in
    [WaitForLaunch, Launch],          // All possible states
    [
        WaitForLaunch => Launch,      // All transitions
    ]
);

// Now the functionality for the states has to be implemented
// Every state can define an entry, execute and exit function.
// The execute function will be called multiple times, while the entry function will only be called
// initially and the exit when the state is left.
impl State for WaitForLaunch {
    fn entry(&mut self) {
        println!("WaitForLaunch: Entry")
    }
    fn execute(&mut self) {
        println!("WaitForLaunch: Execute");
    }
    fn exit(&mut self) {
        println!("WaitForLaunch: Exit");
    }
}
// Then implement the transitions.
// Each transition can define an entry, execute and exit function and must define the guard function.
// Additionally a Into implementation has to be provided so that each state can be transformed
// Into the next one.
impl Into<Launch> for WaitForLaunch {
    fn into(self) -> Launch { Launch {} }
}
impl Transition<Launch> for WaitForLaunch {
    fn entry(&mut self) {
        println!("WaitForLaunch => Launch: Entry")
    }
    fn execute(&mut self) {
        println!("WaitForLaunch => Launch: Execute");
    }
    fn exit(&mut self) {
        println!("WaitForLaunch => Launch: Exit");
    }
    fn guard(&self) -> TransitGuard {
        println!("WaitForLaunch => Launch: Guard");
        return TransitGuard::Transit;
    }
}

impl State for Launch {
    fn entry(&mut self) {
        println!("Launch: Entry")
    }
    fn execute(&mut self) {
        println!("Launch: Execute");
    }
    fn exit(&mut self) {
        println!("Launch: Exit");
    }
}

fn run_basic_example() -> Result<(), SfsmError> {

    // Now the state machine can be instantiated. Note: This corresponds to the name given in the
    // add_state_machine macro.
    let mut rocket = Rocket::new();

    // Once instantiated, the state machine must be started. For this, the initial state must
    // be manually generated and then be moved into the state machine to start it.
    let wait_for_launch = WaitForLaunch {};
    rocket.start(wait_for_launch)?;

    // The IsState trait can be used to check in which state the state machine is in.
    assert!(IsState::<WaitForLaunch>::is_state(&rocket));
    // Run the state machine with .step().
    rocket.step()?;

    assert!(IsState::<Launch>::is_state(&rocket));

    // If the state machine has to be stopped and the data recovered, it can be done so by calling .stop();
    let stopped_state = rocket.stop()?;
    match stopped_state {
        // If you don't want to type out the state enum use the match_state_entry! macro here
        // It generates the following: [SFSM_NAME]States::[STATE_NAME_AND_TYPES]State(state)
        // Otherwise you have to type it out manually with the given schema.
        match_state_entry!(Rocket, Launch, exit_state) => {
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
    fn basic_example() {
        run_basic_example().unwrap();
    }
}