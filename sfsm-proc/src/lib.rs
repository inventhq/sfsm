mod parsers;
mod generators;
mod types;

use quote::{quote};
use proc_macro::{TokenStream};
use types::Machine;
use crate::generators::{StateMachineToTokens, MessagesToTokens};
use crate::types::{MatchStateEntry, Messages};

/// Generates a state machine from a given state machine definition.
///
/// The state machine definition is expected too hold to the following pattern:
/// ```ignore
/// add_state_machine!(
///     StateMachineName,
///     InitialState,
///     [State1, State2, StateN, ...],
///     [StateN => StateN, ...]
/// );
///```
/// - StateMachineName: Defines the name of the state machine.
/// - InitialState: The initial state the state machine will start with.
/// - [State1, State2, StateN, ...]: Specifies all state structs that will be known to the state machine. Each state must implement the State trait.
/// - [StateN => StateN, ...]: Defines all transitions between states that can occur. For each transition, the state must implement the according Transition trait.
///
/// An example might look like this.
/// ```ignore
/// struct Up {}
/// struct Down {}
/// struct Move<T> {}
/// impl State for Move<Up> { ... }
/// impl State for Move<Down> { ... }
/// impl Transition<Move<Up>> for Move<Down> { ... }
/// impl Transition<Move<Down>> for Move<Up> { ... }
/// add_state_machine!(
///         #[derive(Debug)]
///         pub Rocket,
///         Move<Up>,
///         [Move<Up>, Move<Down>],
///         [
///             Move<Up> => Move<Down>
///             Move<Down> => Move<Up>
///         ]
/// );
///```
#[proc_macro]
pub fn add_state_machine(input: TokenStream) -> TokenStream {

    let definition = syn::parse_macro_input!(input as Machine);
    let sfsm_to_tokens = StateMachineToTokens::new(&definition);

    TokenStream::from(quote!{
        #sfsm_to_tokens
    })
}

/// Generates code to push messages into states or poll messages from states.
///
/// The messaging definition is expected too hold to the following pattern:
/// ```ignore
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
/// For each message, the source/target state must implement the according ReceiveMessage or ReturnMessage trait.
/// An example might look like this.
/// ```ignore
/// struct Liftoff {}
/// struct Land {}
/// struct Command<T> {}
/// struct Status { height: float32, speed: float32}
/// add_messages!(
///         Rocket,
///         [
///             Command<Liftoff> -> Move<Down>,
///             Command<Land> -> Move<Up>,
///             Status <- Move<Up>
///             Status <- Move<Down>
///         ]
/// );
///```
#[proc_macro]
pub fn add_messages(input: TokenStream) -> TokenStream {

    let definition = syn::parse_macro_input!(input as Messages);
    let messages_to_tokens = MessagesToTokens::new(&definition);

    TokenStream::from(quote!{
        #messages_to_tokens
    })
}

/// Generate the enum entry of a state. Expects the name of the sfsm and the name (and type args)
/// of the state as well as the desired name of the variable to work with as arguments.
/// Can be used to generate match branches for example.
/// ```ignore
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

    TokenStream::from(quote!{
        #enum_name::#state_entry(#var_name)
    })
}