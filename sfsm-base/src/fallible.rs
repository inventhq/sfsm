use crate::TransitGuard;

/// An error type that will be returned by the state machine if something goes wrong.
/// 
/// It fulfills the same purpose that the ordinary ``` SfsmError ``` does, but allows the
/// user to extend it with custom error types that are required by the fallible state machine.
#[derive(Debug)]
#[non_exhaustive]
pub enum ExtendedSfsmError<T> {
    /// Returned if the state machine gets stuck due to an internal error or if the state
    /// machine has not been started before stepping.
    Internal,

    /// The custom error can be returned from the error state if an error cannot be handled.
    /// In that case, the state machine bubbles the error up to the calling start or step
    /// function where it then must be handled by the user.
    Custom(T)
}

/// Trait that must be implemented by all states that are used by the fallible state machine.
///
/// Behaves similar to the normal ``` State ``` trait, but requires the user to specify
/// an Error type. If this error is returned, the state machine immediately transitions into the
/// error state.
pub trait TryState {

    // The error type that can be returned by the state
    type Error;

    /// Implement any behavior that hast to be executed when entering the state.
    /// Return ``` Ok(()) ``` if no error occurred or ``` Err(Self::Error) ``` if something happened.
    ///
    /// ```rust
    /// # use sfsm_base::fallible::TryState;
    /// # struct FooState;
    /// # impl TryState for FooState {
    /// #     type Error = ();
    ///     fn try_entry(&mut self) -> Result<(), Self::Error> {
    ///         println!("Called right after being transitioned into");
    ///         return Ok(());
    ///     }
    /// # }
    /// ```
    fn try_entry(&mut self) -> Result<(), Self::Error> { Ok(()) }


    /// Implement any behavior that hast to be stepping.
    /// Return ``` Ok(()) ``` if no error occurred or ``` Err(Self::Error) ``` if something happened.
    ///
    /// ```rust
    /// # use sfsm_base::fallible::TryState;
    /// # struct FooState;
    /// # impl TryState for FooState {
    /// #    type Error = ();
    ///     fn try_entry(&mut self) -> Result<(), Self::Error> {
    ///         println!("Called during every step");
    ///         return Ok(());
    ///     }
    /// # }
    /// ```
    fn try_execute(&mut self) -> Result<(), Self::Error> { Ok(()) }


    /// Implement any behavior that hast to be executed when exiting the state.
    /// Return ``` Ok(()) ``` if no error occurred or ``` Err(Self::Error) ``` if something happened.
    ///
    /// ```rust
    /// # use sfsm_base::fallible::TryState;
    /// # struct FooState;
    /// # impl TryState for FooState {
    /// #    type Error = ();
    ///     fn try_entry(&mut self) -> Result<(), Self::Error> {
    ///         println!("Called before transitioning to another state");
    ///         return Ok(());
    ///     }
    /// # }
    /// ```
    fn try_exit(&mut self) -> Result<(), Self::Error> { Ok(()) }
}

/// Trait that must be implemented by all states have a transition.
///
/// Behaves similar to the ``` TryTransition ``` trait but errors can be returned during every
/// call.
pub trait TryTransition<DestinationState>: Into<DestinationState> + TryState {

    /// Implement any behavior that hast to be executed when transitioning to the next the state.
    /// Return ``` Ok(()) ``` if no error occurred or ``` Err(Self::Error) ``` if something happened.
    ///
    /// ```rust
    /// # use sfsm_base::TransitGuard;
    /// # use sfsm_base::fallible::{TryState, TryTransition};
    /// # struct FooState;
    /// # struct BarState;
    /// # impl TryState for FooState {
    /// #      type Error = ();
    /// # };
    /// # impl Into<BarState> for FooState {
    /// #     fn into(self) -> BarState {
    /// #         BarState{}
    /// #     }
    /// # }
    ///
    /// # impl TryTransition<BarState> for FooState {
    ///     fn try_action(&mut self) -> Result<(), Self::Error> {
    ///         println!("Called right after being transitioned into");
    ///         Ok(())
    ///     }
    /// #    fn guard(&self) -> TransitGuard {
    /// #            todo!()
    /// #    }
    /// # }
    /// ```
    fn try_action(&mut self) -> Result<(), Self::Error> { Ok(()) }

    /// Specifies when the state has to transit. Return ``` TransitGuard::Remain ``` to remain
    /// in the current state and ``` TransitGuard::Transit ``` to transit into the next one.
    /// This is the only function that must be implemented by the transition.
    /// The others are optional and situational.
    /// ```rust
    /// # use sfsm_base::TransitGuard;
    /// # use sfsm_base::fallible::{TryState, TryTransition};
    /// # struct FooState;
    /// # struct BarState;
    /// # impl TryState for FooState {
    /// #      type Error = ();
    /// # };
    /// # impl Into<BarState> for FooState {
    /// #     fn into(self) -> BarState {
    /// #         BarState{}
    /// #     }
    /// # }
    /// #
    /// # impl TryTransition<BarState> for FooState {
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

/// This trait must be implemented by the error state.
///
/// The error is being injected into the error state after it has been generated and the
/// ``` consume_error ``` function allows to specify how the incoming error should be handled.
pub trait TryErrorState: TryState {

    /// Handle the incoming error
    /// ```rust
    /// # use sfsm_base::fallible::{TryState, TryErrorState};
    /// # struct ErrorState;
    /// # impl TryState for ErrorState {
    /// #      type Error = ();
    /// # };
    /// #
    /// # impl TryErrorState for ErrorState {
    ///     fn consume_error(&mut self, err: Self::Error) {
    ///         // Store it into the error trait or  it
    ///         println!("Received an error: {:?}", err);
    ///     }
    /// # }
    ///
    /// ```
    fn consume_error(&mut self, err: Self::Error);
}
