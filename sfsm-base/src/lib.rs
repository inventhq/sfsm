#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

/// Contains definitions for a state machine that contains error handling mechanisms
pub mod fallible;

/// Contains definitions used by a state machine without any error handling support
pub mod non_fallible;

/// Contains definitions and code for the messaging system
pub mod message;

/// Enum used to indicate to the guard function if the transition should transit to the
/// next state or remain in the current one.
/// ```rust
/// # use sfsm_base::non_fallible::{Transition, State};
/// # use sfsm_base::TransitGuard;
/// # struct FooState;
/// # struct BarState;
/// # impl State for FooState {};
/// # impl Into<BarState> for FooState {
/// #     fn into(self) -> BarState {
/// #         BarState{}
/// #     }
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
#[derive(PartialEq)]
pub enum TransitGuard {
    /// Remains in the current state
    Remain,
    /// Transits into the next state
    Transit
}

/// Implements from<bool> trait for use of use.
/// This allows to transit by returning true. Which simplify the code since it allows to return the
/// TransitGuard from a simple comparison.
/// ```rust
/// # use sfsm_base::non_fallible::{Transition, State};
/// # use sfsm_base::TransitGuard;
/// # struct FooState;
/// # struct BarState;
/// # impl State for FooState {};
/// # impl Into<BarState> for FooState {
/// #     fn into(self) -> BarState {
/// #         BarState{}
/// #     }
/// # }
///
/// # impl Transition<BarState> for FooState {
///     fn guard(&self) -> TransitGuard {
///         let foo = 0;
///         (foo == 0).into() // Returns TransitGuard::Transit
///     }
/// # }
/// ```
impl From<bool> for TransitGuard {
    fn from(transit: bool) -> Self {
        if transit {
            TransitGuard::Transit
        } else {
            TransitGuard::Remain
        }
    }
}

/// Contains traits that are used to interact with the state machine but should not be implemented
/// manually. All necessary implementations will be created by the macros.
pub mod __protected {

    /// Trait that will be implemented for the state machine.
    pub trait StateMachine {
        /// The initial state of the state machine.
        type InitialState;

        /// The returned error. This is also implemented in non fallible state machines,
        /// but will be ignore as there is no case this error could occur.
        type Error;

        /// The generator enum containing all states
        type StatesEnum;

        /// Start function that must be called first. It populates the internal enum with the
        /// initial state. If step is called before start, the state machine will return an error.
        fn start(&mut self, state: Self::InitialState) -> Result<(), Self::Error>;

        /// The step function that executes all states and transitions.
        fn step(&mut self) -> Result<(), Self::Error>;

        /// If desired, the state machine can be stopped. When doing so, the internal states enum
        /// is returned.
        fn stop(self) -> Result<Self::StatesEnum, Self::Error>;

        /// Peek the internal states enum.
        fn peek_state(&self) -> &Self::StatesEnum;
    }

    /// An implementation of this trait will be generated for every state.
    /// This is can be used to test if the state machine is in a desired state.
    pub trait IsState<State>: StateMachine {
        /// The method must be called with the turbo fish syntax as otherwise Rust cannot figure out
        /// which implementation to call. To check if the state machine is in a given state call:
        ///
        /// ```ignore
        /// let is_in_state: bool = IsState::<State>::is_state(&sfsm);
        /// ```
        fn is_state(&self) -> bool;
    }
}

pub use __protected::*;
pub use non_fallible::*;
pub use fallible::*;
pub use message::*;
pub use message::__protected::*;


