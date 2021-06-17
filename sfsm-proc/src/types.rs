use proc_macro2::Ident;
use syn::{AngleBracketedGenericArguments, Visibility, Attribute};

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
