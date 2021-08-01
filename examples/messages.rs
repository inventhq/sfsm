use sfsm::*;

struct WaitForLaunch {
    malfunction: bool,
    do_launch: bool,
}
struct Launch {}
struct Abort {}

add_state_machine!(
    Rocket,
    WaitForLaunch,
    [WaitForLaunch, Launch, Abort],
    [
        WaitForLaunch => Launch,
        WaitForLaunch => Abort,
        Abort => WaitForLaunch,
    ]
);

// Define the messages to be passed back and forth
#[derive(Debug)]
struct Malfunction {malfunction: bool}
#[derive(Debug)]
struct StartLaunch {start: bool}
struct Status { velocity: f32, height: f32, }

add_messages!(Rocket,
    [
        StartLaunch -> WaitForLaunch,         // Command the WaitForLaunch state to lift off
        Malfunction -> WaitForLaunch,          // Tell the WaitForLaunch that there is a malfunction
        Status <- Launch,                          // Poll the status of the lift
    ]
);

// Add the countdown state implementation
impl State for WaitForLaunch {
    fn entry(&mut self) {
        println!("Begin countdown");
    }
}
impl Into<Abort> for WaitForLaunch {
    fn into(self) -> Abort {Abort {}}
}
impl Transition<Abort> for WaitForLaunch {
    fn guard(&self) -> TransitGuard {
        return self.malfunction.into();
    }
}
// Implement the message passing traits
impl ReceiveMessage<StartLaunch> for WaitForLaunch {
    fn receive_message(&mut self, message: StartLaunch) {
        self.do_launch = message.start;
    }
}
impl ReceiveMessage<Malfunction> for WaitForLaunch {
    fn receive_message(&mut self, message: Malfunction) {
        self.malfunction = message.malfunction;
    }
}


// Then handle the transition to Launch
impl Into<Launch> for WaitForLaunch {
    fn into(self) -> Launch {Launch {}}
}
impl Transition<Launch> for WaitForLaunch {
    fn guard(&self) -> TransitGuard {
        return self.do_launch.into();
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
impl Into<WaitForLaunch> for Abort {
    fn into(self) -> WaitForLaunch {
        WaitForLaunch {
            malfunction: false,
            do_launch: false,
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
impl ReturnMessage<Status> for Launch {
    fn return_message(&mut self) -> Option<Status> {
        Some(Status {height: 1000.0f32, velocity: 300.0f32})
    }
}

fn run_launch_sequence_with_message() -> Result<(), SfsmError> {

    let mut rocket = Rocket::new();

    let wait_for_launch = WaitForLaunch {
        malfunction: false,
        do_launch: false,
    };
    rocket.start(wait_for_launch)?;

    // Check the initial state
    assert!(IsState::<WaitForLaunch>::is_state(&rocket));
    rocket.step()?;

    // Tell the state machine that there has been a malfunction
    PushMessage::<WaitForLaunch, Malfunction>::push_message(&mut rocket, Malfunction {malfunction: true}).unwrap();
    rocket.step()?;
    assert!(IsState::<Abort>::is_state(&rocket));

    // Telling the state machine to start the lift off while not in the WaitForLaunch state will result in an error
    let result = PushMessage::<WaitForLaunch, StartLaunch>::push_message(&mut rocket, StartLaunch {start: true});
    assert!(result.is_err());
    // But it allows us to return the lost message and do something else with it
    if let Err(start_result) = result {
        if let MessageError::StateIsNotActive(start) = start_result {
            assert!(start.start)
        }
    }

    rocket.step()?;
    assert!(IsState::<WaitForLaunch>::is_state(&rocket));

    // Once we are back in the WaitForLaunch state, we can send the message again
    PushMessage::<WaitForLaunch, StartLaunch>::push_message(&mut rocket, StartLaunch {start: true}).unwrap();
    rocket.step()?;
    // And we should be in launch
    assert!(IsState::<Launch>::is_state(&rocket));

    rocket.step()?;
    // Now we can poll the status. If we had done it earlier, we would have gotten an error
    let status_option = PollMessage::<Launch, Status>::poll_message(&mut rocket).unwrap();

    if let Some(status) = status_option {
        assert_eq!(status.velocity, 300.0f32);
        assert_eq!(status.height, 1000.0f32);
    } else {
        assert!(false);
    }

    Ok(())
}

fn main() {
    run_launch_sequence_with_message().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::run_launch_sequence_with_message;

    #[test]
    fn launch_sequence_with_message() {
        run_launch_sequence_with_message().unwrap();
    }
}