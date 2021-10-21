use crate::TransitGuard;

/// An error type that will be returned by the state machine if something goes wrong.
/// 
/// Specifically, when the state machine gets stuck in a state due to an internal error.
/// The state machine is designed in a way where this should not happen, so this can largely be
/// ignored. It is used in situations that are other wise hard to avoid without a panic!.
/// It might be extended in the future to contains custom error codes generated from the states
/// themselves
#[derive(Debug)]
#[non_exhaustive]
pub enum SfsmError {
    /// Returned if the state machine gets stuck due to an internal error or if the state
    /// machine has not been started before stepping.
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
    ///
    /// ```rust
    /// # use sfsm_base::non_fallible::State;
    /// # struct FooState;
    /// # impl State for FooState {
    ///     fn entry(&mut self) {
    ///         println!("Called right after being transitioned into");
    ///     }
    /// # }
    /// ```
    fn entry(&mut self) {}

    /// Implement any behavior that has to be executed when the state is being executed.
    /// This function will be called as long as the state does not transit.
    ///
    /// ```rust
    /// # use sfsm_base::non_fallible::State;
    /// # struct FooState;
    /// # impl State for FooState {
    ///     fn execute(&mut self) {
    ///         println!("Called during every step");
    ///     }
    /// # }
    /// ```
    fn execute(&mut self) {}

    /// Implement any behavior that hast to be executed when exiting the state.
    ///
    /// ```rust
    /// # use sfsm_base::non_fallible::State;
    /// # struct FooState;
    /// # impl State for FooState {
    ///     fn exit(&mut self) {
    ///         println!("Called before transitioning to another state");
    ///     }
    /// # }
    /// ```
    fn exit(&mut self) {}
}

/// Trait that must be implemented by a state that want to transition to DestinationState.
///
/// All states can have none or many transitions.
/// On top of the transition trait the state must implement the ``` Into<DestinationState> ```
/// trait to specify what happens with the source state data while transitioning and how the
/// destination state is generated.
/// The action method is run once the transition executes.
/// The only non optional method is the guard function that specifies when the state transitions.
pub trait Transition<DestinationState>: Into<DestinationState> + State {
    /// Implement any behavior that hast to be executed when exiting the state.
    /// ```rust
    /// # use sfsm_base::non_fallible::{Transition, State};
    /// # use sfsm_base::TransitGuard;
    /// # struct FooState;
    /// # struct BarState;
    /// # impl State for FooState {};
    /// # impl Into<BarState> for FooState {
    /// #     fn into(self) -> BarState { BarState{} }
    /// # }
    ///
    /// # impl Transition<BarState> for FooState {
    ///     fn action(&mut self) {
    ///         println!("Called while transitioning to another state");
    ///     }
    /// #    fn guard(&self) -> TransitGuard {
    /// #            todo!()
    /// #    }
    /// # }
    /// ```
    fn action(&mut self) {}

    /// Specifies when the state has to transit. Return ``` TransitGuard::Remain ``` to remain
    /// in the current state and ``` TransitGuard::Transit ``` to transit into the next one.
    /// This is the only function that must be implemented by the transition.
    /// The others are optional and situational.
    /// ```rust
    /// # use sfsm_base::non_fallible::{Transition, State};
    /// # use sfsm_base::TransitGuard;
    /// # struct FooState;
    /// # struct BarState;
    /// # impl State for FooState {};
    /// # impl Into<BarState> for FooState {
    /// #     fn into(self) -> BarState { BarState{} }
    /// # }
    ///
    /// # impl Transition<BarState> for FooState {
    ///     fn guard(&self) -> TransitGuard {
    ///         let foo = 0;
    ///         if foo == 0 {
    ///             TransitGuard::Remain
    ///         } else {
    ///             TransitGuard::Transit
    ///         }
    ///     }
    /// # }
    /// ```
    fn guard(&self) -> TransitGuard;
}
