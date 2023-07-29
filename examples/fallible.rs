use sfsm::*;

// The states
pub struct Launch {}
pub struct WaitForLaunch {
    boosters_started: bool,
}

// The error state
pub struct HandleMalfunction {
    res: Result<(), RocketMalfunction>,
}

// Some helper functions
impl HandleMalfunction {
    pub fn new() -> Self {
        Self { res: Ok(()) }
    }
}

// The errors that can occur in the state machine
#[derive(Debug)]
pub enum RocketMalfunction {
    BoostersWontStart,
    BoostersFellOff,
}

// Implement all the functions for the various states
impl TryState for Launch {
    type Error = RocketMalfunction;
    fn try_execute(&mut self) -> Result<(), Self::Error> {
        Err(RocketMalfunction::BoostersFellOff) // During the launch, the boosters fell off.
                                                // Return an error to jump into the error state
    }
}

impl TryState for WaitForLaunch {
    type Error = RocketMalfunction;

    fn try_entry(&mut self) -> Result<(), Self::Error> {
        println!("Start launch procedure");
        Ok(())
    }

    fn try_execute(&mut self) -> Result<(), Self::Error> {
        if !self.boosters_started {
            Err(RocketMalfunction::BoostersWontStart) // During the first start, the boosters did not start properly
        } else {
            println!("Everything ok. Proceed with launch");
            Ok(()) // If the boosters have properly been started, proceed with the launch
        }
    }
}

// Implement the various transitions
derive_transition_into!(WaitForLaunch, Launch);

// Start the launch
derive_try_transition!(WaitForLaunch, Launch, TransitGuard::Transit);

// Every state must implement a Into trait for the error state. Otherwise valuable data could get
// lost.
impl Into<HandleMalfunction> for Launch {
    fn into(self) -> HandleMalfunction {
        HandleMalfunction::new()
    }
}
impl Into<HandleMalfunction> for WaitForLaunch {
    fn into(self) -> HandleMalfunction {
        HandleMalfunction::new()
    }
}

// Restart the launch as soon as the malfunction is handled
derive_try_transition!(HandleMalfunction, WaitForLaunch, TransitGuard::Transit);

impl Into<WaitForLaunch> for HandleMalfunction {
    fn into(self) -> WaitForLaunch {
        WaitForLaunch {
            boosters_started: true,
        }
    }
}

// Implement the error handling in the error state
impl TryState for HandleMalfunction {
    type Error = RocketMalfunction;

    fn try_entry(&mut self) -> Result<(), Self::Error> {
        if let Err(err) = &(self.res) {
            match err {
                RocketMalfunction::BoostersWontStart => {
                    // If the boosters won't start, just restart the launch.
                    println!("Handle error: Turn off and restart launch");
                }
                RocketMalfunction::BoostersFellOff => {
                    println!("Handle error: Abort the launch");
                    return Err(RocketMalfunction::BoostersFellOff); // Its a old rocket. There is nothing we can do.
                                                                    // So we are returning the error which will then
                                                                    // Have to be handled in the main application
                }
            }
        }
        Ok(())
    }
}
// As the last step, add the TryErrorState implementation. It must be implemented by the
// error state to specify what it should do with the error. Here it is stored in the
// struct for processing in the try_entry function.
impl TryErrorState for HandleMalfunction {
    fn consume_error(&mut self, err: Self::Error) {
        println!("Error state received a new error: {:?}", err);
        self.res = Err(err);
    }
}

add_fallible_state_machine!(
    Rocket,                                 // Name of the state machine. Accepts a visibility modifier.
    WaitForLaunch,                               // The initial state the state machine will start in
    [WaitForLaunch, Launch, HandleMalfunction],  // All possible states
    [
        WaitForLaunch => Launch,                 // All possible Transitions
        HandleMalfunction => WaitForLaunch
    ],
    RocketMalfunction,                      // The error type
    HandleMalfunction                       // The error state
);

/// Register a logger function
/// Enable the trace features for the tracing to work
/// The logger function receives logs from the state machine and forwards them
/// to what ever logging mechanism desired.
#[sfsm_trace]
fn trace(log: &str) {
    println!("{}", log);
}

fn run_error_example() -> Result<(), ExtendedSfsmError<RocketMalfunction>> {
    let mut rocket = Rocket::new();

    let wait_for_launch = WaitForLaunch {
        boosters_started: false,
    };
    rocket.start(wait_for_launch)?;

    assert!(IsState::<WaitForLaunch>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<HandleMalfunction>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<WaitForLaunch>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<Launch>::is_state(&rocket));
    let res = rocket.step(); // There is an error during the launch which causes a transition
                             // to the error state. The error state knows it cannot handle
                             // the error and thus aborts right in the entry of the error state
    assert!(res.is_err());

    Ok(())
}

fn main() {
    run_error_example().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::run_error_example;

    #[test]
    fn fallible_example() {
        run_error_example().unwrap();
    }
}
