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

impl State for Action<WaitForLaunch> {
}
impl Into<Action<Ascent>> for Action<WaitForLaunch> {
    fn into(self) -> Action<Ascent> { Action { phantom: PhantomData }}
}
impl Transition<Action<Ascent>> for Action<WaitForLaunch> {
    fn guard(&self) -> TransitGuard {
        return TransitGuard::Transit;
    }
}

impl State for Action<Ascent> {
}
impl Into<Action<Descent>> for Action<Ascent> {
    fn into(self) -> Action<Descent> { Action { phantom: PhantomData }}
}
impl Transition<Action<Descent>> for Action<Ascent> {
    fn guard(&self) -> TransitGuard {
        return TransitGuard::Transit;
    }
}

impl State for Action<Descent> {
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