use sfsm::*;

// First define all the state structs
struct WaitForLaunch {
    malfunction: bool,
    countdown: u32,
    tries: u32,
}
struct Launch {}
struct Abort {tries: u32}

add_state_machine!(
    Rocket,                           // Name of the state machine
    WaitForLaunch,                    // The state machine will start at the count down
    [WaitForLaunch, Launch, Abort],   // All possible states
    [
        WaitForLaunch => Launch,      // If all is ok, the launch will start
        WaitForLaunch => Abort,       // If there is a malfunction, abort
        Abort => WaitForLaunch,       // Once the malfunction is cleared, we can restart the countdown
    ]
);

// Add the countdown state implementation
impl State for WaitForLaunch {
    fn entry(&mut self) {
        println!("Begin countdown");
        self.countdown = 3;             // Set the countdown to 3 seconds
    }
    fn execute(&mut self) {
        println!("{} seconds to launch", self.countdown);
        // A malfunction has been found during the first run
        if self.countdown == 2 && self.tries == 0 {
            self.malfunction = true;
        }
        self.countdown -= 1;            // Count down the seconds to launch
    }
}

// Implement the transitions for WaitForLaunch
// Begin with the transition to Abort
// Every transition can define an action method. The guard function must be defined.
impl Into<Abort> for WaitForLaunch {
    fn into(self) -> Abort {Abort {tries: self.tries}}
}
impl Transition<Abort> for WaitForLaunch {
    fn guard(&self) -> TransitGuard {
        return self.malfunction.into();  // Immediately transition if there is a malfunction
    }
}

// Then handle the transition to Launch
impl Into<Launch> for WaitForLaunch {
    fn into(self) -> Launch {Launch {}}
}
impl Transition<Launch> for WaitForLaunch {
    fn guard(&self) -> TransitGuard {
        if self.countdown == 0 {
            return TransitGuard::Transit;   // Transit as soon as the countdown reaches 0
        }
        return TransitGuard::Remain;
    }
}

// Now handle the Abort state
impl State for Abort {
    fn entry(&mut self) {
        println!("Malfunction found");
        self.tries += 1;
    }
    fn execute(&mut self) {
        println!("Fix malfunction");
    }
}
impl Into<WaitForLaunch> for Abort {
    fn into(self) -> WaitForLaunch {
        WaitForLaunch {
            countdown: 0,
            malfunction: false,
            tries: self.tries,                       // Update the number of previous tries
        }
    }
}
impl Transition<WaitForLaunch> for Abort {
    fn guard(&self) -> TransitGuard {
        return TransitGuard::Transit;
    }
}

// And finally the launch state that has no transitions
impl State for Launch {
    fn entry(&mut self) {
        println!("Firing up boosters");
    }
}


fn run_launch_sequence() -> Result<(), SfsmError> {

    let mut rocket = Rocket::new();

    let wait_for_launch = WaitForLaunch {
        tries: 0,
        malfunction: false,
        countdown: 0,
    };
    rocket.start(wait_for_launch)?;

    // Check the initial state
    assert!(IsState::<WaitForLaunch>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<WaitForLaunch>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<Abort>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<WaitForLaunch>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<WaitForLaunch>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<WaitForLaunch>::is_state(&rocket));
    rocket.step()?;

    // Now we should be lifting off
    assert!(IsState::<Launch>::is_state(&rocket));
    rocket.step()?;

    Ok(())
}

fn main() {
    run_launch_sequence().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::run_launch_sequence;

    #[test]
    fn launch_sequence() {
        run_launch_sequence().unwrap();
    }
}
