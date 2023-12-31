use sfsm::*;
use sfsm::message::{MessageError, ReturnMessage, ReceiveMessage};

/// This example requires the trace* features to be enabled to run

/// Register a logger function
/// The logger function receives logs from the state machine and forwards them 
/// to what ever logging mechanism desired.
#[sfsm_trace]
fn trace(log: &str) {
    println!("{}", log);
}

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

#[derive(Debug)]
struct Malfunction {malfunction: bool}
#[derive(Debug)]
struct StartLaunch {start: bool}
struct Status { }

add_messages!(Rocket,
    [
        StartLaunch -> WaitForLaunch,
        Malfunction -> WaitForLaunch,
        Status <- Launch,
    ]
);


derive_state!(WaitForLaunch);
derive_state!(Launch);
derive_state!(Abort);

derive_transition_into!(WaitForLaunch, Abort);
impl Transition<Abort> for WaitForLaunch {
    fn guard(&self) -> TransitGuard {
        return self.malfunction.into();
    }
}
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

derive_transition_into!(WaitForLaunch, Launch);
impl Transition<Launch> for WaitForLaunch {
    fn guard(&self) -> TransitGuard {
        return self.do_launch.into();
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

derive_transition!(Abort, WaitForLaunch, TransitGuard::Transit);

impl ReturnMessage<Status> for Launch {
    fn return_message(&mut self) -> Option<Status> {
        Some(Status { })
    }
}

fn run_launch_sequence_with_message() -> Result<(), SfsmError> {

    let mut rocket = Rocket::new();

    let wait_for_launch = WaitForLaunch {
        malfunction: false,
        do_launch: false,
    };
    rocket.start(wait_for_launch)?;

    assert!(IsState::<WaitForLaunch>::is_state(&rocket));
    rocket.step()?;

    PushMessage::<WaitForLaunch, Malfunction>::push_message(&mut rocket, Malfunction {malfunction: true}).unwrap();
    rocket.step()?;
    assert!(IsState::<Abort>::is_state(&rocket));

    let result = PushMessage::<WaitForLaunch, StartLaunch>::push_message(&mut rocket, StartLaunch {start: true});
    assert!(result.is_err());

    rocket.step()?;
    assert!(IsState::<WaitForLaunch>::is_state(&rocket));

    PushMessage::<WaitForLaunch, StartLaunch>::push_message(&mut rocket, StartLaunch {start: true}).unwrap();
    rocket.step()?;
    assert!(IsState::<Launch>::is_state(&rocket));

    rocket.step()?;
    PollMessage::<Launch, Status>::poll_message(&mut rocket).unwrap();
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
