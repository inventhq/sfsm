use sfsm::*;

struct CountDownToLiftoff {
    malfunction: bool,
    do_liftoff: bool,
}
struct Liftoff {}
struct Abort {}

add_state_machine!(
    Rocket,
    CountDownToLiftoff,
    [CountDownToLiftoff, Liftoff, Abort],
    [
        CountDownToLiftoff => Liftoff,
        CountDownToLiftoff => Abort,
        Abort => CountDownToLiftoff,
    ]
);

// Define the messages to be passed back and forth
#[derive(Debug)]
struct Malfunction {malfunction: bool}
#[derive(Debug)]
struct StartLiftoff {start: bool}
struct Status { velocity: f32, height: f32, }

add_messages!(Rocket,
    [
        StartLiftoff -> CountDownToLiftoff,         // Command the CountDownToLiftoff state to lift off
        Malfunction -> CountDownToLiftoff,          // Tell the CountDownToLiftoff that there is a malfunction
        Status <- Liftoff,                          // Poll the status of the lift
    ]
);

// Add the countdown state implementation
impl State for CountDownToLiftoff {
    fn entry(&mut self) {
        println!("Begin countdown");
    }
}
impl Into<Abort> for CountDownToLiftoff {
    fn into(self) -> Abort {Abort {}}
}
impl Transition<Abort> for CountDownToLiftoff {
    fn guard(&self) -> TransitGuard {
        return self.malfunction.into();
    }
}
// Implement the message passing traits
impl ReceiveMessage<StartLiftoff> for CountDownToLiftoff {
    fn receive_message(&mut self, message: StartLiftoff) {
        self.do_liftoff = message.start;
    }
}
impl ReceiveMessage<Malfunction> for CountDownToLiftoff {
    fn receive_message(&mut self, message: Malfunction) {
        self.malfunction = message.malfunction;
    }
}


// Then handle the transition to Liftoff
impl Into<Liftoff> for CountDownToLiftoff {
    fn into(self) -> Liftoff {Liftoff {}}
}
impl Transition<Liftoff> for CountDownToLiftoff {
    fn guard(&self) -> TransitGuard {
        return self.do_liftoff.into();
    }
}

impl State for Abort {
    fn entry(&mut self) {
        println!("Malfunction found");
    }
    fn execute(&mut self) {
        println!("Fix malfunction");
    }
}
impl Into<CountDownToLiftoff> for Abort {
    fn into(self) -> CountDownToLiftoff {
        CountDownToLiftoff {
            malfunction: false,
            do_liftoff: false,
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
impl ReturnMessage<Status> for Liftoff {
    fn return_message(&mut self) -> Option<Status> {
        Some(Status {height: 1000.0f32, velocity: 300.0f32})
    }
}

fn run_liftoff_sequence_with_message() -> Result<(), SfsmError> {

    let mut rocket = Rocket::new();

    let countdown = CountDownToLiftoff {
        malfunction: false,
        do_liftoff: false,
    };
    rocket.start(countdown)?;

    // Check the initial state
    assert!(IsState::<CountDownToLiftoff>::is_state(&rocket));
    rocket.step()?;

    // Tell the state machine that there has been a malfunction
    PushMessage::<CountDownToLiftoff, Malfunction>::push_message(&mut rocket, Malfunction {malfunction: true}).unwrap();
    rocket.step()?;
    assert!(IsState::<Abort>::is_state(&rocket));

    // Telling the state machine to start the lift off while not in the CountDownToLiftoff state will result in an error
    let result = PushMessage::<CountDownToLiftoff, StartLiftoff>::push_message(&mut rocket, StartLiftoff {start: true});
    assert!(result.is_err());
    // But it allows us to return the lost message and do something else with it
    if let Err(start_result) = result {
        if let MessageError::StateIsNotActive(start) = start_result {
            assert!(start.start)
        }
    }

    rocket.step()?;
    assert!(IsState::<CountDownToLiftoff>::is_state(&rocket));

    // Once we are back in the CountDownToLiftoff state, we can send the message again
    PushMessage::<CountDownToLiftoff, StartLiftoff>::push_message(&mut rocket, StartLiftoff {start: true}).unwrap();
    rocket.step()?;
    // And we should be in liftoff
    assert!(IsState::<Liftoff>::is_state(&rocket));

    rocket.step()?;
    // Now we can poll the status. If we had done it earlier, we would have gotten an error
    let status_option = PollMessage::<Liftoff, Status>::poll_message(&mut rocket).unwrap();

    if let Some(status) = status_option {
        assert_eq!(status.velocity, 300.0f32);
        assert_eq!(status.height, 1000.0f32);
    } else {
        assert!(false);
    }

    Ok(())
}

fn main() {
    run_liftoff_sequence_with_message().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::run_liftoff_sequence_with_message;

    #[test]
    fn liftoff_sequence_with_message() {
        run_liftoff_sequence_with_message().unwrap();
    }
}