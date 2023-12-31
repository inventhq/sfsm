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

derive_state!(Offline);
derive_state!(Standby);
derive_state!(Requesting);
derive_state!(Observing);
derive_state!(Reporting);
derive_transition!(Standby, Requesting, TransitGuard::Transit);
derive_transition!(Requesting, Observing, TransitGuard::Transit);
derive_transition!(Reporting, Standby, TransitGuard::Transit);
derive_transition!(Observing, Reporting, TransitGuard::Transit);
derive_transition!(Offline, Online, TransitGuard::Transit);
derive_transition_into!(Online, Offline);
derive_transition_into!(Standby, Requesting);
derive_transition_into!(Requesting, Observing);
derive_transition_into!(Observing, Reporting);
derive_transition_into!(Reporting, Standby);

impl State for Online {
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

/// Register a logger function
/// Enable the trace features for the tracing to work
/// The logger function receives logs from the state machine and forwards them
/// to what ever logging mechanism desired.
#[sfsm_trace]
fn trace(log: &str) {
    println!("{}", log);
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

