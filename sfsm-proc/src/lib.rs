mod parsers;
mod generators;
mod types;

use quote::{quote};
use proc_macro::{TokenStream};
use types::Machine;
use crate::generators::StateMachineToTokens;
use crate::types::{MatchStateEntry};

/// Generates a state machine from a given state machine definition.
///
/// The state machine definition is expected too hold to the following pattern:
/// ```ignore
/// add_state_machine!(
///     StateMachineName,
///     InitialState,
///     [State1, State2, StateN, ...],
///     [StateN -> StateN, ...]
/// );
///```
/// So the following example:
/// ```ignore
/// add_state_machine!(
///         #[derive(Debug)]
///         pub Elevator,
///         Move<Up>,
///         [Move<Up>, Move<Down>],
///         [
///             Move<Up> -> Move<Down>
///         ]
/// );
///```
/// will expand to this state machine.
///
///```ignore
/// #[derive(Debug)]
/// pub enum ElevatorStates {
///     MoveUpState(Option<Move<Up>>),
///     MoveDownState(Option<Move<Down>>),
/// }
/// #[derive(Debug)]
/// pub struct Elevator {
///     states: ElevatorStates,
///     do_entry: bool,
/// }
/// impl Elevator {
///     pub fn new(data: Move<Up>) -> Self {
///         Self {
///             states: ElevatorStates::MoveUpState(Some(data)),
///             do_entry: true,
///         }
///     }
///     pub fn step(&mut self) -> Result<(), SfsmError>  {
///         use ElevatorStates::*;
///         let ref mut e = self.states;
///         *e = match *e {
///             ElevatorStates::MoveUpState(ref mut state_option) => {
///                 let mut state = state_option.take().ok_or(SfsmError::Internal)?;
///                 if self.do_entry {
///                     State::entry(&mut state);
///                     Transition::<Move<Down>>::entry(&mut state);
///                     self.do_entry = false;
///                 }
///                 State::execute(&mut state);
///                 Transition::<Move<Down>>::execute(&mut state);
///                 if Transition::<Move<Down>>::guard(&state) {
///                     State::exit(&mut state);
///                     Transition::<Move<Down>>::exit(&mut state);
///                     let mut next_state: Move<Down> = state.into();
///                     self.do_entry = true;
///                     ElevatorStates::MoveDownState(Some(next_state))
///                 } else {
///                     ElevatorStates::MoveUpState(Some(state))
///                 }
///             }
///             ElevatorStates::MoveDownState(ref mut state_option) => {
///                 let mut state = state_option.take().unwrap();
///                 if self.do_entry {
///                     State::entry(&mut state);
///                     self.do_entry = false;
///                 }
///                 State::execute(&mut state);
///                 {
///                     ElevatorStates::MoveDownState(Some(state))
///                 }
///             }
///         }
///         Ok(())
///     }
///     pub fn peek_state(&self) -> &ElevatorStates {
///         return &self.states;
///     }
///     pub fn stop(mut self) -> Result<ElevatorStates, SfsmError> {
///         match self.states {
///             ElevatorStates::MoveUpState(ref mut state_option) => {
///                 let mut state = state_option.take().ok_or(SfsmError::Internal)?;
///                 State::exit(&mut state);
///                 Transition::<Move<Down>>::exit(&mut state);
///                 Ok(ElevatorStates::MoveUpState(Some(state)))
///             }
///             ElevatorStates::MoveDownState(ref mut state_option) => {
///                 let mut state = state_option.take().ok_or(SfsmError::Internal)?;
///                 State::exit(&mut state);
///                 Ok(ElevatorStates::MoveDownState(Some(state)))
///             }
///         }
///     }
/// }
///
/// // One for each state
/// impl IsState<Move<Down>> for Elevator {
///     fn is_state(&self) -> bool {
///         return match self.states {
///             ElevatorStates::MoveDownState(_) => {
///                 true
///             }
///             _ => false
///         }
///     }
/// }
///```
///
#[proc_macro]
pub fn add_state_machine(input: TokenStream) -> TokenStream {

    let definition = syn::parse_macro_input!(input as Machine);
    let sfsm_to_tokens = StateMachineToTokens::new(&definition);

    TokenStream::from(quote!{
        #sfsm_to_tokens
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