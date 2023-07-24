#![doc = include_str!("../README.md")]

use crate::generators::{MessagesToTokens, StateMachineToTokens};
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;
mod generators;
mod parsers;
mod trace;
mod types;
use crate::types::{
    DeriveTransition, DeriveTransitionBase, Machine, MatchStateEntry, Messages, State, TryMachine,
};

/// Generates a state machine from a given state machine definition.
///
/// The state machine definition is expected too hold to the following pattern:
/// ```rust,ignore
/// add_state_machine!(
///     StateMachineName,
///     InitialState,
///     [State1, State2, StateN, ...],
///     [StateN => StateN, ...]
/// );
///```
/// - StateMachineName: Defines the name of the state machine.
/// - InitialState: The initial state the state machine will start with.
/// - [State1, State2, StateN, ...]: Specifies all state structs that will be known to the state machine. Each state must implement the ``` State ``` trait.
/// - [StateN => StateN, ...]: Defines all transitions between states that can occur. For each transition, the state must implement the according ``` Transition ``` trait.
///
/// An example might look like this:
/// ```rust
/// # use sfsm_proc::add_state_machine;
/// # use sfsm_base::non_fallible::*;
/// # use sfsm_base::*;
/// # use std::marker::PhantomData;
/// # #[derive(Debug)]
/// # struct Ascent {}
/// # #[derive(Debug)]
/// # struct Descent {}
/// # #[derive(Debug)]
/// # struct Action<T> {
/// #    phantom: PhantomData<T>
/// # }
/// # impl State for Action<Ascent> { }
/// # impl State for Action<Descent> { }
/// #
/// # impl Into<Action<Ascent>> for Action<Descent> {
/// #     fn into(self) -> Action<Ascent> {
/// #         todo!()
/// #     }
/// # }
/// # impl Transition<Action<Ascent>> for Action<Descent> {
/// #    fn guard(&self) -> TransitGuard {
/// #        todo!()
/// #    }
/// # }
/// #
/// # impl Into<Action<Descent>> for Action<Ascent> {
/// #    fn into(self) -> Action<Descent> {
/// #         todo!()
/// #     }
/// # }
/// # impl Transition<Action<Descent>> for Action<Ascent> {
/// #    fn guard(&self) -> TransitGuard {
/// #        todo!()
/// #    }
/// # }
/// #
/// add_state_machine!(
///         #[derive(Debug)]
///         Rocket,
///         Action<Ascent>,
///         [Action<Ascent>, Action<Descent>],
///         [
///             Action<Ascent> => Action<Descent>,
///             Action<Descent> => Action<Ascent>
///         ]
/// );
///```
/// Expand the example to see more, or check out the examples folder for a more complete example.
#[proc_macro]
pub fn add_state_machine(input: TokenStream) -> TokenStream {
    let definition = syn::parse_macro_input!(input as Machine);
    let sfsm_to_tokens = StateMachineToTokens::new(&definition);

    TokenStream::from(quote! {
        #sfsm_to_tokens
    })
}

/// Generates a fallible state machine from a given state machine definition with error handling.
///
/// The state machine definition is expected too hold to the following pattern:
/// ```rust,ignore
/// add_fallible_state_machine!(
///     StateMachineName,
///     InitialState,
///     [State1, State2, StateN, ...],
///     [StateN => StateN, ...],
///     ErrorType,
///     ErrorState
/// );
///```
/// - StateMachineName: Defines the name of the state machine.
/// - InitialState: The initial state the state machine will start with.
/// - [State1, State2, StateN, ...]: Specifies all state structs that will be known to the state machine. Each state must implement the ``` State ``` trait.
/// - [StateN => StateN, ...]: Defines all transitions between states that can occur. For each transition, the state must implement the according ``` Transition ``` trait.
/// - ErrorType: Defines the type of error that can be returned from the states.
/// - ErrorState: Defines the state that will act as the error handle state. It must implement the ``` TryErrorState ``` trait. Adding it to the state definitions is optional.
///
/// ```rust
/// # use sfsm_base::fallible::*;
/// # use sfsm_proc::add_fallible_state_machine;
/// # use sfsm_base::*;
/// # struct Ascent {} // Ascent state
/// # struct WaitForLaunch {} // WaitForLaunch state
/// # // The error state
/// # struct HandleMalfunction {}
/// # // The error returned by all states and transitions
/// # enum RocketMalfunction {}
/// #
/// # // The implementations of the states
/// # impl TryState for Ascent {
/// #     type Error = RocketMalfunction;
/// # }
/// # impl TryState for WaitForLaunch {
/// #     type Error = RocketMalfunction;
/// # }
/// # impl TryState for HandleMalfunction {
/// #     type Error = RocketMalfunction;
/// # }
/// #
/// # impl Into<WaitForLaunch> for HandleMalfunction {
/// #     fn into(self) -> WaitForLaunch {
/// #         todo!()
/// #     }
/// # }
/// #
/// # impl Into<Ascent> for WaitForLaunch {
/// #     fn into(self) -> Ascent {
/// #         todo!()
/// #     }
/// # }
/// #
/// # impl TryTransition<WaitForLaunch> for HandleMalfunction {
/// #    fn guard(&self) -> TransitGuard {
/// #        todo!()
/// #    }
/// # }
/// # impl TryTransition<Ascent> for WaitForLaunch {
/// #    fn guard(&self) -> TransitGuard {
/// #        todo!()
/// #    }
/// # }
/// #
/// # impl Into<HandleMalfunction> for WaitForLaunch {
/// #     fn into(self) -> HandleMalfunction {
/// #         todo!()
/// #     }
/// # }
/// #
/// # impl Into<HandleMalfunction> for Ascent {
/// #     fn into(self) -> HandleMalfunction {
/// #         todo!()
/// #     }
/// # }
/// #
/// # // The TryErrorState implementation for the error state
/// # impl TryErrorState for HandleMalfunction {
/// #     fn consume_error(&mut self, err: Self::Error) {
/// #         // Do something with the error
/// #     }
/// # }
/// #
/// add_fallible_state_machine!(
///     Rocket,
///     WaitForLaunch,
///     [WaitForLaunch, Ascent, HandleMalfunction],
///     [
///         WaitForLaunch => Ascent,
///         HandleMalfunction => WaitForLaunch
///     ],
///     RocketMalfunction,
///     HandleMalfunction
/// );
///```
/// Expand the example to see more, or check out the examples folder for a more complete example.
#[proc_macro]
pub fn add_fallible_state_machine(input: TokenStream) -> TokenStream {
    let definition = syn::parse_macro_input!(input as TryMachine);
    let sfsm_to_tokens = StateMachineToTokens::new(&definition.state_machine);

    TokenStream::from(quote! {
        #sfsm_to_tokens
    })
}

/// Generates code to push messages into states or poll messages from states.
///
/// The messaging definition is expected too hold to the following pattern:
/// ```rust,ignore
/// add_messages!(
///     StateMachineName,
///     [
///         Message1 <- State1,
///         Message2 <- State1,
///         Message1 -> State2,
///         ...
///     ]
/// );
/// ```
/// - StateMachineName: This must match a previously with add_state_machine defined state machine.
/// - [ Message1 <- State1, ... ] Defines all messages that can be passed back an forth. The message specifies the struct/enum that will be used as a message, the <- arrow defines a poll and the -> a push and the state is the target or source state.
/// For each message, the source/target state must implement the according ``` ReceiveMessage ``` or ``` ReturnMessage ``` trait.
/// An example might look like this.
/// ```rust
/// # use sfsm_proc::add_state_machine;
/// # use sfsm_proc::add_messages;
/// # use sfsm_base::*;
/// # use std::marker::PhantomData;
/// # #[derive(Debug)]
/// # struct Launch {}
/// # #[derive(Debug)]
/// # struct Land {}
/// # struct Ascent {}
/// # struct Descent {}
/// # struct Action<T> {
/// #    phantom: PhantomData<T>
/// # }
/// # #[derive(Debug)]
/// # struct Command<T> {
/// #    phantom: PhantomData<T>
/// # }
/// # struct Status { height: f32, speed: f32}
/// # impl State for Action<Ascent> { }
/// # impl State for Action<Descent> { }
/// #
/// # impl ReceiveMessage<Command<Launch>> for Action<Descent> {
/// #    fn receive_message(&mut self, message: Command<Launch>) {
/// #        println!("Received message {:?}", message);
/// #    }
/// # }
/// #
/// # impl ReceiveMessage<Command<Land>> for Action<Ascent> {
/// #    fn receive_message(&mut self, message: Command<Land>) {
/// #        println!("Received message {:?}", message);
/// #    }
/// # }
///#
/// # impl ReturnMessage<Status> for Action<Ascent> {
/// #    fn return_message(&mut self) -> Option<Status> {
/// #        return Some(Status { height: 1.0f32, speed: 2.0f32 });
/// #    }
/// # }
/// #
/// # impl ReturnMessage<Status> for Action<Descent> {
/// #    fn return_message(&mut self) -> Option<Status> {
/// #        return Some(Status { height: 1.0f32, speed: 2.0f32 });
/// #    }
/// # }
/// #
/// # add_state_machine!(
/// #         Rocket,
/// #         Action<Ascent>,
/// #         [Action<Descent>, Action<Ascent>],
/// #         []
/// # );
/// #
/// add_messages!(
///         Rocket,
///         [
///             Command<Launch> -> Action<Descent>,
///             Command<Land> -> Action<Ascent>,
///             Status <- Action<Ascent>,
///             Status <- Action<Descent>
///         ]
/// );
///```
#[proc_macro]
pub fn add_messages(input: TokenStream) -> TokenStream {
    let definition = syn::parse_macro_input!(input as Messages);
    let messages_to_tokens = MessagesToTokens::new(&definition);

    TokenStream::from(quote! {
        #messages_to_tokens
    })
}

/// Generate the enum entry of a state. Expects the name of the sfsm and the name (and type args)
/// of the state as well as the desired name of the variable to work with as arguments.
/// Can be used to generate match branches for example.
/// ```rust,ignore
/// match exit {
///     match_state_entry!(NameOfTheSfsm, DesiredState<AndType>, var_name) => {
///         // Access "var_name" here.
///         // Var name will be Option<DesiredState<AndType>>
///     },
///     _ => {
///     }
/// }
/// ```
#[proc_macro]
pub fn match_state_entry(input: TokenStream) -> TokenStream {
    let match_state_entry: MatchStateEntry = syn::parse_macro_input!(input as MatchStateEntry);
    let state_entry = match_state_entry.state_entry;
    let enum_name = state_entry.enum_name;
    let state_entry = state_entry.state_entry;
    let var_name = match_state_entry.var_name;

    TokenStream::from(quote! {
        #enum_name::#state_entry(#var_name)
    })
}

/// Creates a wrapper around a log function to forward the logs to.
/// With the help of ``` sfsm_trace ```, a logger function to which all logs from the state machine
/// are forwarded to can be configured
/// ```rust,ignore
/// #[sfsm_trace]
/// fn trace(log: &str) {
///     println!("{}", log);
/// }
/// ```
#[proc_macro_attribute]
pub fn sfsm_trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let trace_function: ItemFn = syn::parse_macro_input!(item as ItemFn);
    let trace_function_ident: &proc_macro2::Ident = &trace_function.sig.ident;
    TokenStream::from(quote! {
        #trace_function
        fn __sfsm_trace(str: &str) {
            #trace_function_ident(str);
        }
    })
}

/// Derives an empty transition of a transition from one state into another and allows to
/// customise if it should always transit or never.
/// ```rust,ignore
/// derive_transition!(Foo, Bar, TransitGuard::Transit);
/// // Generates
/// impl Transition<Bar> for Bar {
///     fn guard(&self) -> TransitGuard {
///         TransitGuard::Transit
///     }
/// }
/// ```
/// This macro is implemented as a proc macro instead of a derive macro because it needs additional
/// info that is difficult to get into a derive macro in a semantic way.
#[proc_macro]
pub fn derive_transition(input: TokenStream) -> TokenStream {
    let transition: DeriveTransition = syn::parse_macro_input!(input as DeriveTransition);
    let src = transition.transition.src;
    let dst = transition.transition.dst;
    let guard = transition.guard;
    TokenStream::from(quote! {
        impl sfsm::Transition<#dst> for #src {
            fn guard(&self) -> TransitGuard {
                #guard
            }
        }
    })
}

/// Derives an empty implementation of the state.
/// ```rust,ignore
/// derive_state!(Foo);
/// // Generates
/// impl State for Foo {};
/// ```
/// It's somewhat redundant, but it is added for consistency to match the other derive_* functions
/// and to help keep state machine short and clean.
#[proc_macro]
pub fn derive_state(input: TokenStream) -> TokenStream {
    let state: State = syn::parse_macro_input!(input as State);
    let name = state.name;
    let generics = state.generics;
    TokenStream::from(quote! {
        impl sfsm::State for #name #generics {}
    })
}

/// Derives an empty transition of a transition from one state into another and allows to
/// customise if it should always transit or never.
/// ```rust,ignore
/// derive_try_transition!(Foo, Bar, TransitGuard::Transit);
/// // Generates
/// impl TryTransition<Bar> for Bar {
///     fn guard(&self) -> TransitGuard {
///         TransitGuard::Transit
///     }
/// }
/// ```
/// This macro is implemented as a proc macro instead of a derive macro because it needs additional
/// info that is difficult to get into a derive macro in a semantic way.
#[proc_macro]
pub fn derive_try_transition(input: TokenStream) -> TokenStream {
    let transition: DeriveTransition = syn::parse_macro_input!(input as DeriveTransition);
    let src = transition.transition.src;
    let dst = transition.transition.dst;
    let guard = transition.guard;
    TokenStream::from(quote! {
        impl sfsm::TryTransition<#dst> for #src {
            fn guard(&self) -> TransitGuard {
                #guard
            }
        }
    })
}

/// Derives an empty implementation of the TryState.
/// ```rust,ignore
/// derive_try_state!(Foo);
/// // Generates
/// impl TryState for Foo {};
/// ```
/// It's somewhat redundant, but it is added for consistency to match the other derive_* functions
/// and to help keep state machine short and clean.
#[proc_macro]
pub fn derive_try_state(input: TokenStream) -> TokenStream {
    let state: State = syn::parse_macro_input!(input as State);
    let name = state.name;
    let generics = state.generics;
    TokenStream::from(quote! {
        impl sfsm::TryState for #name #generics {}
    })
}

/// Derives an a implementation of the into trait for the transition if the target state does
/// not contains any members
/// ```rust,ignore
/// derive_transition_into!(Foo, Bar);
/// // Generates
/// impl Into<Bar> for Foo {
///     fn into(self) -> Bar {
///         Bar {}
///     }
/// }
/// ```
/// This macro is implemented as a proc macro instead of a derive macro because it needs additional
/// info that is difficult to get into a derive macro in a semantic way.
#[proc_macro]
pub fn derive_transition_into(input: TokenStream) -> TokenStream {
    let transition: DeriveTransitionBase = syn::parse_macro_input!(input as DeriveTransitionBase);
    let src = transition.src;
    let dst = transition.dst;
    TokenStream::from(quote! {
        impl Into<#dst> for #src {
            fn into(self) -> #dst {
                #dst {}
            }
        }
    })
}

/// Derives an empty a implementation fo the into trait for the transition if the target state
/// implements the ``` Default ``` trait.
/// ```rust,ignore
/// derive_transition_into_default!(Foo, Bar);
/// // Generates
/// impl Into<Bar> for Foo {
///     fn into(self) -> Bar {
///         Bar::default()
///     }
/// }
/// ```
/// This macro is implemented as a proc macro instead of a derive macro because it needs additional
/// info that is difficult to get into a derive macro in a semantic way.
#[proc_macro]
pub fn derive_transition_into_default(input: TokenStream) -> TokenStream {
    let transition: DeriveTransitionBase = syn::parse_macro_input!(input as DeriveTransitionBase);
    let src = transition.src;
    let dst = transition.dst;
    TokenStream::from(quote! {
        impl Into<#dst> for #src {
            fn into(self) -> #dst {
                #dst::default()
            }
        }
    })
}
