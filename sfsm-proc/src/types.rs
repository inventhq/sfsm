use proc_macro2::{Ident, TokenStream};
use syn::{AngleBracketedGenericArguments, Visibility, Attribute};

pub enum Mode {
    NonFallible,
    Fallible
}

pub struct TraitDefinitions {
    pub transit_trait: TokenStream,
    pub state_trait: TokenStream,
    pub exit: TokenStream,
    pub entry: TokenStream,
    pub execute: TokenStream,
}

pub struct ErrorType {
    pub error_name: Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
}

pub struct TryMachine {
    pub state_machine: Machine,
}

#[derive(Clone)]
/// Contains all data for the states
pub struct State {
    pub name: Ident,
    pub transits: Vec<State>,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub enum_name: Ident,
}

/// Contains a transition from one state to another
pub struct Transition {
    pub src: State,
    pub dst: State,
}

// Contains all data required to generate the state machine
pub struct Machine {
    pub attributes: Vec<Attribute>,
    pub visibility: Option<Visibility>,
    pub name: Ident,
    pub init: State,
    pub states: Vec<State>,
    pub enum_name: Ident,
    pub sfsm_error: TokenStream,
    pub custom_error: Option<TokenStream>,
    pub trait_definitions: TraitDefinitions,
    pub mode: Mode,
    pub error_state: Option<State>,
}

// Contains data needed to generate generate a enum entry for a state
pub struct StateEntry {
    pub enum_name: Ident,
    pub state_entry: Ident,
}

// Contains data needed to generate generate a enum entry for a state to match
pub struct MatchStateEntry {
    pub state_entry: StateEntry,
    pub var_name: Ident,
}

// The actual message containing the struct name and optional generics arguments
pub struct Message {
    pub generics: Option<AngleBracketedGenericArguments>,
    pub name: Ident,
}

// Enum containing the direction of the message. Can be either a push or poll message
pub enum MessageDir {
    Push(Message),
    Poll(Message),
}

// Contains the target state plus all message information used to generate the necessary trait
// implementations
pub struct StateMessage {
    pub state: State,
    pub message: MessageDir,
}

// The whole message that will be used to generate the macro outputs
pub struct Messages {
    pub name: Ident,
    pub enum_name: Ident,
    pub messages: Vec<StateMessage>,
}