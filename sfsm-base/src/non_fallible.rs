use crate::TransitGuard;

/// An error type that will be returned by the state machine if something goes wrong.
/// Specifically, when the state machine gets stuck in a state due to an internal error.
/// The state machine is designed in a way where this should not happen, so this can largely be
/// ignored. It is used in situations that are other wise hard to avoid without a panic!.
/// It might be extended in the future to contains custom error codes generated from the states
/// themselves
#[derive(Debug)]
#[non_exhaustive]
pub enum SfsmError {
    Internal,
}

/// Trait that must be implemented by all states
///
/// Allows to define behavior when entering, exiting and running the state. Both the entry and exit
/// function will only be executed once for each state. The execute function will be executed as
/// long as the state does not transition into another state. There can only ever be one single
/// state active.
pub trait State {

    /// Implement any behavior that hast to be executed when entering the state.
    fn entry(&mut self) {}

    /// Implement any behavior that has to be executed when the state is being executed.
    /// This function will be called as long as the state does not transit.
    fn execute(&mut self) {}

    /// Implement any behavior that hast to be executed when exiting the state.
    fn exit(&mut self) {}
}

/// Trait that must be implemented by a state that want to transition to DestinationState.
///
/// All states can have none or many transitions.
/// Both the entry and exit function will only be executed once for each state. The execute
/// function will be executed as long as the state does not transition into another state.
/// On top of the transition trait the state must implement the Into<DestinationState> trait
/// to specify what happens with the source state data while transitioning and how the destination
/// state is generated.
/// The only non optional function is the guard function that specifies when the state transitions.
/// Note: All transition behavior is always executed after the state trait behavior.
pub trait Transition<DestinationState>: Into<DestinationState> + State {
    /// Implement any behavior that hast to be executed when entering the state.
    fn entry(&mut self) {}

    /// Implement any behavior that has to be executed when the state is being executed.
    /// This function will be called as long as the state does not transit.
    fn execute(&mut self) {}

    /// Implement any behavior that hast to be executed when exiting the state.
    fn exit(&mut self) {}

    /// Specifies when the state has to transit. As long as the guard returns false, the state
    /// stays in the current state. When true is returned, the state machine will transit to
    /// DestinationState
    fn guard(&self) -> TransitGuard;
}