use proc_macro2::{Ident, Span};
use proc_macro::{TokenStream};
use syn::{Result, AngleBracketedGenericArguments, Visibility, Attribute, Error};
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::{Punctuated};
use syn::Token;
use quote::{quote};
use crate::types::{State, Transition, Machine, StateEntry, MatchStateEntry, StateMessage, Messages, Message, MessageDir, ErrorType, TryMachine, Mode, TraitDefinitions};

/// Parses the name of a state and optionally a type.
/// For example Foo or Bar<T>
impl Parse for State {
    fn parse(input: ParseStream) -> Result<Self> {

        let name: Ident = input.parse()?;

        let generics = if input.peek(Token![<]) {
            input.parse::<AngleBracketedGenericArguments>().ok()
        } else {
            None
        };

        let enum_name = State::state_to_enum(&name, &generics);

        Ok(Self {
            name,
            transits: vec![],
            generics,
            enum_name,
        })
    }
}

/// Parses a transition that must be in the form of
/// Foo -> Bar or optionally with types like Foo<T> -> Bar<T>
impl Parse for Transition {
    fn parse(input: ParseStream) -> Result<Self> {
        let src: State = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        input.parse::<syn::Token![>]>()?;
        let dst: State = input.parse()?;

        Ok(Self {
            src,
            dst
        })
    }
}

impl Machine {
    pub fn enum_name(sfsm_name: &Ident) -> Ident {
        Ident::new(format!("{}States", sfsm_name.to_string()).as_str(),
                   Span::call_site())
    }
}

/// Parses the state machine in the form of
/// name, Foo, [Foo, Bar], [Foo -> Bar]
impl Parse for Machine {
    fn parse(input: ParseStream) -> Result<Self> {

        let attributes = input.call(Attribute::parse_outer)?;

        let visibility: Option<Visibility> = input.parse().ok();

        let name: Ident = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let init_definition: State = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let state_group = input.parse::<proc_macro2::Group>()?;
        let state_group_ts: TokenStream = state_group.stream().into();
        let state_parser = Punctuated::<State, Token![,]>::parse_terminated;
        let punctuated_state_names = state_parser.parse(state_group_ts)?;
        let states_names: Vec<State> = punctuated_state_names.into_iter().collect();

        input.parse::<syn::Token![,]>()?;

        let transition_group = input.parse::<proc_macro2::Group>()?;
        let transition_group_ts: TokenStream = transition_group.stream().into();
        let transition_parser =
            Punctuated::<Transition, Token![,]>::parse_terminated;
        let punctuated_transitions = transition_parser.parse(transition_group_ts)?;
        let transitions: Vec<Transition> = punctuated_transitions.into_iter().collect();

        let states: Vec<State> = states_names.into_iter().map(|state| {

            let transitions: Vec<State> = (&transitions).into_iter().filter(|trans| {
                return trans.src.enum_name == state.enum_name;
            }).map(|trans| (*trans).dst.clone()).collect();

            State {
                name: state.name,
                transits: transitions,
                generics: state.generics,
                enum_name: state.enum_name,
            }

        }).collect();

        let init = (&states).into_iter().find(|state| {
            return init_definition.enum_name == state.enum_name;
        }).expect("Expected to find the init state in the list of states").clone();

        let enum_name = Machine::enum_name(&name);

        let sfsm_error = proc_macro2::TokenStream::from(quote! {
            SfsmError
        });

        let trait_definitions = TraitDefinitions {
            state_trait: proc_macro2::TokenStream::from(quote! {
                State
            }),
            transit_trait: proc_macro2::TokenStream::from(quote! {
                Transition
            }),
            entry: proc_macro2::TokenStream::from(quote! {
                entry
            }),
            exit: proc_macro2::TokenStream::from(quote! {
                exit
            }),
            execute: proc_macro2::TokenStream::from(quote! {
                execute
            }),
        };


        Ok(Self {
            attributes,
            visibility,
            name,
            init,
            states,
            enum_name,
            sfsm_error,
            trait_definitions,
            mode: Mode::NonFallible,
            error_state: None,
            custom_error: None,
        })
    }
}

impl Parse for StateEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let sfsm_name: Ident = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let state: State = input.parse()?;

        Ok(Self {
            enum_name: Machine::enum_name(&sfsm_name),
            state_entry: state.enum_name,
        })
    }
}

impl Parse for MatchStateEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let state_entry: StateEntry = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let var_name: Ident = input.parse()?;
        Ok(Self {
            state_entry,
            var_name
        })
    }
}
impl Parse for Message {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        // Only parse the generic argument if the bracket is opened and no - follows.
        // If we only checked for the < the arrow <- would trigger the parsing.
        let generics = if input.peek(Token![<])
            && !input.peek2(Token![-]) {
            input.parse::<AngleBracketedGenericArguments>().ok()
        } else {
            None
        };

        Ok(Self {
            name,
            generics
        })
    }
}

impl Parse for StateMessage {
    fn parse(input: ParseStream) -> Result<Self> {
        let message: Message = input.parse()?;

        let message_dir: MessageDir = if input.peek(syn::Token![->]) {
            input.parse::<syn::Token![->]>()?;
            MessageDir::Push(message)
        } else if input.peek(syn::Token![<-]) {
            input.parse::<syn::Token![<-]>()?;
            MessageDir::Poll(message)
        } else {
            return Err(Error::new(input.span(), format!("A direction must be specified with either '->' or '<-' but got '{}' instead", input)))
        };

        let state: State = input.parse()?;
        Ok(Self {
            message: message_dir,
            state
        })
    }
}


/// Parses the message definitions in the form of
/// name, [M1 -> Foo, M2 <- Bar]
impl Parse for Messages {
    fn parse(input: ParseStream) -> Result<Self> {

        let name: Ident = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let state_message_group = input.parse::<proc_macro2::Group>()?;
        let state_message_group_ts: TokenStream = state_message_group.stream().into();
        let state_message_parser = Punctuated::<StateMessage, Token![,]>::parse_terminated;
        let punctuated_state_names = state_message_parser.parse(state_message_group_ts)?;
        let messages: Vec<StateMessage> = punctuated_state_names.into_iter().collect();

        let enum_name = Machine::enum_name(&name);

        Ok(Self {
            name,
            enum_name,
            messages
        })
    }
}

impl Parse for ErrorType {
    fn parse(input: ParseStream) -> Result<Self> {

        let error_name: Ident = input.parse()?;

        let generics = if input.peek(Token![<]) {
            input.parse::<AngleBracketedGenericArguments>().ok()
        } else {
            None
        };

        Ok(Self {
            error_name,
            generics
        })
    }
}

/// Parses the state machine in the form of
/// name, Foo, [Foo, Bar], [Foo -> Bar], ErrorType, ErrorState
impl Parse for TryMachine {
    fn parse(input: ParseStream) -> Result<Self> {

        let mut state_machine: Machine = input.parse().expect("Expected a state machine definition");
        input.parse::<syn::Token![,]>()?;
        let error_type: ErrorType = input.parse().expect("Expected an error type");
        input.parse::<syn::Token![,]>()?;
        let error_state_entry: State = input.parse().expect("Expected an error state");

        let error_type_name = error_type.error_name;
        let error_type_generics = error_type.generics;
        let custom_error = proc_macro2::TokenStream::from(quote! {
            <#error_type_name#error_type_generics>
        });
        let sfsm_error = proc_macro2::TokenStream::from(quote! {
            ExtendedSfsmError
        });

        let states = &(state_machine.states);
        let error_state = (&states).into_iter().find(|state| {
            return error_state_entry.enum_name == state.enum_name;
        }).expect("Expected to find the error state in the list of states").clone();

        state_machine.mode = Mode::Fallible;
        state_machine.error_state = Some(error_state.clone());
        state_machine.sfsm_error = sfsm_error;
        state_machine.custom_error = Some(custom_error);
        state_machine.trait_definitions = TraitDefinitions {
            state_trait: proc_macro2::TokenStream::from(quote! {TryState}),
            transit_trait: proc_macro2::TokenStream::from(quote! {TryTransition}),
            entry: proc_macro2::TokenStream::from(quote! {try_entry}),
            exit: proc_macro2::TokenStream::from(quote! {try_exit}),
            execute: proc_macro2::TokenStream::from(quote! {try_execute}),
        };

        Ok(Self {
            state_machine
        })
    }
}
