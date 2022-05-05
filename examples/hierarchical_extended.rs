use sfsm::*;

// An example of how a extended hierarchical state machine can be built

pub struct Online {
    state_machine: OnlineMachine,
}
pub struct Offline {}
pub struct Standby {}
pub struct Requesting {}
pub struct Observing {}
pub struct Reporting {}

add_state_machine!(
    ForwardObserver,
    Offline,
    [Offline, Online],
    [
        Offline => Online,
        Online => Offline,
    ]
);

// Defines the Online inner state machine.
add_state_machine!(
    OnlineMachine,
    Standby,
    [Standby, Requesting, Observing, Reporting],
    [
        Standby => Requesting,
        Requesting => Observing,
        Observing => Reporting,
    ]
);

impl State for Online {

    fn entry(&mut self) {
        println!("Online: Entry");
    }

    /// Executes the sub-state machine on each step.
    fn execute(&mut self) {
        self.state_machine.step().unwrap();
    }
}

// Initialize the Online state machine on transition.
impl From<Offline> for Online {
    /// Constructs, and starts, the [`Online`] state machine on a transition from Offline
    fn from(_: Offline) -> Self {
        let mut online = Self {
            state_machine: OnlineMachine::new(),
        };
        online.state_machine.start(Standby {}).unwrap();
        online
    }
}

impl Transition<Offline> for Online {
    fn guard(&self) -> TransitGuard {
        IsState::<Reporting>::is_state(&self.state_machine).into()
    }
}

impl State for Offline {
    fn entry(&mut self) {
        println!("Offline: Entry");
    }
}

impl State for Standby {
    fn entry(&mut self) {
        println!("Standby: Entry");
    }
}

impl State for Requesting  {
    fn entry(&mut self) {
        println!("Requesting: Entry");
    }
}

impl State for Observing  {
    fn entry(&mut self) {
        println!("Observing: Entry");
    }
}

impl State for Reporting  {
    fn entry(&mut self) {
        println!("Reporting: Entry");
    }
}

fn run_hierarchical_extended() -> Result<(), SfsmError> {

    // Build the outer state machine
    let mut forward_observer = ForwardObserver::new();

    // Build the initial state
    let offline = Offline {};
    forward_observer.start(offline)?;

    // The IsState trait can be used to check in which state the state machine is in.
    assert!(IsState::<Offline>::is_state(&forward_observer));
    // Run the state machine with .step().
    forward_observer.step()?;
    assert!(IsState::<Online>::is_state(&forward_observer));

    forward_observer.step()?;
    assert!(IsState::<Online>::is_state(&forward_observer));

    forward_observer.step()?;
    assert!(IsState::<Online>::is_state(&forward_observer));

    forward_observer.step()?;
    assert!(IsState::<Offline>::is_state(&forward_observer));

    forward_observer.step()?;
    assert!(IsState::<Online>::is_state(&forward_observer));

    Ok(())
}

fn main() {
    run_hierarchical_extended().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::run_hierarchical_extended;

    #[test]
    fn hierarchical_extended() {
        run_hierarchical_extended().unwrap();
    }
}

impl Into<Offline> for Online {
    fn into(self) -> Offline {
        Offline {}
    }
}
impl Transition<Online> for Offline {
    fn guard(&self) -> TransitGuard {
        true.into()
    }
}
impl Into<Requesting> for Standby {
    fn into(self) -> Requesting {
        Requesting {}
    }
}

impl Transition<Requesting> for Standby {
    fn guard(&self) -> TransitGuard {
        true.into()
    }
}

impl Into<Observing> for Requesting {
    fn into(self) -> Observing {
        Observing {}
    }
}
impl Transition<Observing> for Requesting {
    fn guard(&self) -> TransitGuard {
        true.into()
    }
}

impl Into<Reporting> for Observing {
    fn into(self) -> Reporting {
        Reporting {}
    }
}

impl Transition<Reporting> for Observing {
    fn guard(&self) -> TransitGuard {
        true.into()
    }
}

impl Into<Standby> for Reporting {
    fn into(self) -> Standby {
        Standby {}
    }
}

impl Transition<Standby> for Reporting {
    fn guard(&self) -> TransitGuard {
        true.into()
    }
}
