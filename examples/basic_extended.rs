use sfsm::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Ascent {}
#[derive(Debug)]
pub struct Descent {}
#[derive(Debug)]
pub struct WaitForLaunch {}
#[derive(Debug)]
pub struct Action<T> {
    phantom: PhantomData<T>
}

add_state_machine!(
    #[derive(Debug)]                                            // Attributes for the generated struct can be defined
    pub Rocket,                                                 // Name of the state machine. Accepts a visibility modifier.
    Action<WaitForLaunch>,                                      // The state machine will start at the count Descent
    [Action<WaitForLaunch>, Action<Ascent>, Action<Descent>],   // All possible states
    [
        Action<WaitForLaunch> => Action<Ascent>,
        Action<Ascent> => Action<Descent>
    ]
);

// Use the derive macros to implement empty default implementations.
// Check out the basic examples to know how to implement them manually
derive_state!(Action<WaitForLaunch>);
derive_state!(Action<Ascent>);
derive_state!(Action<Descent>);
derive_transition!(Action<WaitForLaunch>, Action<Ascent>, TransitGuard::Transit);
derive_transition!(Action<Ascent>, Action<Descent>, TransitGuard::Transit);

/// Register a logger function
/// Enable the trace features for the tracing to work
/// The logger function receives logs from the state machine and forwards them
/// to what ever logging mechanism desired.
#[sfsm_trace]
fn trace(log: &str) {
    println!("{}", log);
}

impl Into<Action<Ascent>> for Action<WaitForLaunch> {
    fn into(self) -> Action<Ascent> { Action { phantom: PhantomData }}
}
impl Into<Action<Descent>> for Action<Ascent> {
    fn into(self) -> Action<Descent> { Action { phantom: PhantomData }}
}

fn run_basic_extended_example() -> Result<(), SfsmError> {

    let mut rocket = Rocket::new();

    let wait_for_launch = Action {phantom: PhantomData};
    rocket.start(wait_for_launch)?;

    assert!(IsState::<Action<WaitForLaunch>>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<Action<Ascent>>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<Action<Descent>>::is_state(&rocket));
    rocket.step()?;

    let stopped_state = rocket.stop()?;
    match stopped_state {
        match_state_entry!(Rocket, Action<Descent>, exit_state) => {
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
