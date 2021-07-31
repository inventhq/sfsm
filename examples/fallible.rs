use sfsm::*;

// The states
pub struct MoveUp {}
pub struct Grounded {
    boosters_started: bool
}

// The error state
pub struct HandleMalfunction {
    res: Result<(), RocketMalfunction>,
}

// Some helper functions
impl HandleMalfunction {
    pub fn new() -> Self {
        Self {
            res: Ok(())
        }
    }
}

// The errors that can occur in the state machine
#[derive(Debug)]
pub enum RocketMalfunction {
    BoostersWontStart,
    BoostersFellOff
}

// Implement all the functions for the various states
impl TryState for MoveUp {
    type Error = RocketMalfunction;
    fn try_execute(&mut self) -> Result<(), Self::Error> {
        Err(RocketMalfunction::BoostersFellOff) // During the launch, the boosters fell off.
                                                // Return an error to jump into the error state
    }
}

impl TryState for Grounded {
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
            Ok(())  // If the boosters have properly been started, proceed with the launch
        }
    }
}

// Implement the various transitions
impl Into<MoveUp> for Grounded {
    fn into(self) -> MoveUp {
        MoveUp {}
    }
}

// Start the launch
impl TryTransition<MoveUp> for Grounded {
    fn guard(&self) -> TransitGuard {
        TransitGuard::Transit
    }
}

// Every state must implement a Into trait for the error state. Otherwise valuable data could get
// lost.
impl Into<HandleMalfunction> for MoveUp {
    fn into(self) -> HandleMalfunction {
        HandleMalfunction::new()
    }
}
impl Into<HandleMalfunction> for Grounded {
    fn into(self) -> HandleMalfunction {
        HandleMalfunction::new()
    }
}

// Restart the launch as soon as the malfunction is handled
impl TryTransition<Grounded> for HandleMalfunction {
    fn guard(&self) -> TransitGuard {
        TransitGuard::Transit
    }
}
impl Into<Grounded> for HandleMalfunction {
    fn into(self) -> Grounded {
        Grounded {
            boosters_started: true
        }
    }
}

// Implement the error handling in the error state
impl TryState for HandleMalfunction {
    type Error = RocketMalfunction;

    fn try_entry(&mut self) -> Result<(), Self::Error> {
        if let Err(err) = &(self.res) {
            match err {
                RocketMalfunction::BoostersWontStart => {   // If the boosters won't start, just restart the launch.
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
    Grounded,                               // The initial state the state machine will start in
    [Grounded, MoveUp, HandleMalfunction],  // All possible states
    [
        Grounded => MoveUp,                 // All possible Transitions
        HandleMalfunction => Grounded
    ],
    RocketMalfunction,                      // The error type
    HandleMalfunction                       // The error state
);

fn run_error_example() -> Result<(), ExtendedSfsmError<RocketMalfunction>> {

    let standstill = Grounded {boosters_started: false};
    let mut rocket = Rocket::new(standstill);

    assert!(IsState::<Grounded>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<HandleMalfunction>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<Grounded>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<MoveUp>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<HandleMalfunction>::is_state(&rocket));
    let res = rocket.step();
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