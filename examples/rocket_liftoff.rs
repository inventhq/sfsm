use sfsm::*;

// First define all the state structs
struct CountDownToLiftoff {
    malfunction: bool,
    countdown: u32,
    tries: u32,
}
struct Liftoff {}
struct Abort {tries: u32}

add_state_machine!(
    Rocket,                                 // Name of the state machine
    CountDownToLiftoff,                     // The state machine will start at the count down
    [CountDownToLiftoff, Liftoff, Abort],   // All possible states
    [
        CountDownToLiftoff => Liftoff,      // If all is ok, the liftoff will start
        CountDownToLiftoff => Abort,        // If there is a malfunction, abort
        Abort => CountDownToLiftoff,        // Once the malfunction is cleared, we can restart the countdown
    ]
);

// Add the countdown state implementation
impl State for CountDownToLiftoff {
    fn entry(&mut self) {
        println!("Begin countdown");
    }
    fn execute(&mut self) {
        println!("{} seconds to liftoff", self.countdown);
        // A malfunction has been found during the first run
        if self.countdown == 2 && self.tries == 0 {
            self.malfunction = true;
        }
    }
}

// Implement the transitions for CountDownToLiftoff
// Begin with the transition to Abort
// Every transition can define an entry, execute and exit function. The guard function must be defined.
impl Into<Abort> for CountDownToLiftoff {
    fn into(self) -> Abort {Abort {tries: self.tries}}
}
impl Transition<Abort> for CountDownToLiftoff {
    fn guard(&self) -> TransitGuard {
        return self.malfunction.into();  // Immediately transition if there is a malfunction
    }
}

// Then handle the transition to Liftoff
impl Into<Liftoff> for CountDownToLiftoff {
    fn into(self) -> Liftoff {Liftoff {}}
}
impl Transition<Liftoff> for CountDownToLiftoff {
    fn entry(&mut self) {
        self.countdown = 3;             // Set the countdown to 3 seconds
    }

    fn execute(&mut self) {
        self.countdown -= 1;            // Count down the seconds to liftoff
    }
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
impl Into<CountDownToLiftoff> for Abort {
    fn into(self) -> CountDownToLiftoff {
        CountDownToLiftoff {
            countdown: 0,
            malfunction: false,
            tries: self.tries,                       // Update the number of previous tries
        }
    }
}
impl Transition<CountDownToLiftoff> for Abort {
    fn guard(&self) -> TransitGuard {
        return TransitGuard::Transit;
    }
}

// And finally the liftoff state that has no transitions
impl State for Liftoff {
    fn entry(&mut self) {
        println!("Firing up boosters");
    }
}


fn run_liftoff_sequence() -> Result<(), SfsmError> {

    let mut rocket = Rocket::new();

    let countdown = CountDownToLiftoff {
        tries: 0,
        malfunction: false,
        countdown: 0,
    };
    rocket.start(countdown)?;

    // Check the initial state
    assert!(IsState::<CountDownToLiftoff>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<CountDownToLiftoff>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<Abort>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<CountDownToLiftoff>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<CountDownToLiftoff>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<CountDownToLiftoff>::is_state(&rocket));
    rocket.step()?;

    // Now we should be lifting off
    assert!(IsState::<Liftoff>::is_state(&rocket));
    rocket.step()?;

    Ok(())
}

fn main() {
    run_liftoff_sequence().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::run_liftoff_sequence;

    #[test]
    fn liftoff_sequence() {
        run_liftoff_sequence().unwrap();
    }
}