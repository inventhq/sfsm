use quote::{quote, ToTokens};
use crate::types::{Machine, State, Messages, StateMessage, MessageDir, Mode};
use proc_macro2::TokenStream;

pub struct TransitToErrorToTokens {}

impl<'a> TransitToErrorToTokens {
    fn wrap_if_fallible(machine: &'a Machine, tokens: TokenStream, current_state: &State) -> proc_macro2::TokenStream {
        match &machine.mode {
            Mode::NonFallible { .. } => {
                proc_macro2::TokenStream::from(quote! {
                    #tokens;
                })
            }
            Mode::Fallible { .. } => {
                let error_state_entry = &(machine.error_state).as_ref().expect("Internal error. Expected to have a error state.").enum_name;
                let enum_name = &machine.enum_name;
                let error_state = &(machine.error_state).as_ref().expect("Internal error. Expected to have a error state.");
                if error_state.enum_name != current_state.enum_name {
                    let transit_trait = &machine.trait_definitions.transit_trait;
                    let entry = &machine.trait_definitions.entry;
                    let state_trait = &machine.trait_definitions.state_trait;
                    let error_state_transition_entries: Vec<TokenStream> = (&error_state.transits).into_iter().map(|transition| {
                        proc_macro2::TokenStream::from(quote! {
                            #transit_trait::<#transition>::#entry(&mut err_state).map_err(|err| {ExtendedSfsmError::Custom(err)})?;
                        })
                    }).collect();

                    proc_macro2::TokenStream::from(quote! {
                        if let Err(err) = #tokens {
                            let mut err_state: #error_state = state.into();
                            err_state.consume_error(err);
                            #state_trait::#entry(&mut err_state).map_err(|err| {ExtendedSfsmError::Custom(err)})?;
                            #(#error_state_transition_entries)*
                            return Ok(#enum_name::#error_state_entry(Some(err_state)));
                        }
                    })
                } else {
                    proc_macro2::TokenStream::from(quote! {
                        #tokens.map_err(|err| {ExtendedSfsmError::Custom(err)})?;
                    })
                }
            }
        }
    }
}

pub struct StateMachineToTokens<'a> {
    machine: &'a Machine,
}

impl<'a> StateMachineToTokens<'a> {
    pub fn new(machine: &'a Machine) -> Self {
        Self {
            machine,
        }
    }
}

impl ToTokens for StateMachineToTokens<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let sfsm_name = &self.machine.name;
        let enum_name = &self.machine.enum_name;
        let init_state = &self.machine.init;
        let init_state_entry = &self.machine.init.enum_name;
        let attribute = &self.machine.attributes;
        let vis = &self.machine.visibility;
        let state_trait = &self.machine.trait_definitions.state_trait;
        let transit_trait = &self.machine.trait_definitions.transit_trait;
        let entry = &self.machine.trait_definitions.entry;

        let states: Vec<StateToTokens> = (&self.machine.states).into_iter().map(|state| {
            StateToTokens::new(self.machine, state)
        }).collect();

        let state_entries: Vec<StateEntriesToTokens> = (&self.machine.states).into_iter().map(|state| {
            StateEntriesToTokens::new(state)
        }).collect();

        let exits: Vec<StopToTokens> = (&self.machine.states).into_iter().map(|state| {
            return StopToTokens::new(self.machine, state);
        }).collect();

        let is_states: Vec<IsStateToTokens> = (&self.machine.states).into_iter().map(|state| {
            return IsStateToTokens::new(self.machine, state);
        }).collect();

        let init_transition_entry_tokens: Vec<TokenStream> = (&init_state.transits).into_iter().map(|transition| {
            TransitToErrorToTokens::wrap_if_fallible(&self.machine, proc_macro2::TokenStream::from(quote! {
                #transit_trait::<#transition>::#entry(&mut state)
            }), &init_state)
        }).collect();

        let init_state_tokens: TokenStream = TransitToErrorToTokens::wrap_if_fallible(&self.machine, proc_macro2::TokenStream::from(quote! {
                #state_trait::#entry(&mut state)
            }), &init_state);

        let sfsm_error = &self.machine.sfsm_error;
        let custom_error = &self.machine.custom_error;

        let token_steam = proc_macro2::TokenStream::from(quote! {
            use sfsm::*;

            #(#attribute)*
            #vis enum #enum_name {
                #(#state_entries)*
            }

            #(#attribute)*
            #vis struct #sfsm_name {
                states: #enum_name,
            }

            impl #sfsm_name {
                pub fn new() -> Self {
                    Self {
                        states: #enum_name::#init_state_entry(None)
                    }
                }
            }

            impl StateMachine for #sfsm_name {
                type InitialState = #init_state;
                type Error = #sfsm_error#custom_error;
                type StatesEnum = #enum_name;

                fn start(&mut self, mut state: Self::InitialState) -> Result<(), Self::Error> {
                    fn run_state(mut state: #init_state) -> Result<#enum_name, #sfsm_error#custom_error> {
                        #init_state_tokens
                        #(#init_transition_entry_tokens)*
                        Ok(#enum_name::#init_state_entry(Some(state)))
                    }
                    self.states = run_state(state)?;
                    Ok(())
                }

                fn step(&mut self) -> Result<(), Self::Error> {
                    use #enum_name::*;
                    let ref mut e = self.states;
                    *e = match *e {
                        #( #states, )*
                    };
                    Ok(())
                }

                fn stop(mut self) -> Result<Self::StatesEnum, Self::Error> {
                    match self.states {
                        # ( #exits )*,
                    }
                }

                fn peek_state(&self) -> &Self::StatesEnum {
                   return &self.states;
                }
            }

            // Implement the is_state checks
            #(#is_states)*
        });

        tokens.extend(token_steam);
    }
}

pub struct StopToTokens<'a> {
    machine: &'a Machine,
    state: &'a State,
}

impl<'a> StopToTokens<'a> {
    pub fn new(machine: &'a Machine, state: &'a State) -> Self {
        Self {
            machine,
            state
        }
    }
}

impl ToTokens for StopToTokens<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let state_entry = &self.state.enum_name;
        let enum_name = &self.machine.enum_name;
        let exit_transitions = ExitTransitionToTokens::new(&self.state.transits, &self.machine, &self.state);

        let state_trait = &self.machine.trait_definitions.state_trait;
        let exit = &self.machine.trait_definitions.exit;
        let sfsm_error = &self.machine.sfsm_error;

        let exit_token_stream = TransitToErrorToTokens::wrap_if_fallible(&self.machine, proc_macro2::TokenStream::from(quote! {
                #state_trait::#exit(&mut state)
        }), &self.state);

        let token_steam = proc_macro2::TokenStream::from(quote! {
            #enum_name::#state_entry(ref mut state_option) => {
                let mut state = state_option.take().ok_or(#sfsm_error::Internal)?;
                #exit_token_stream
                #exit_transitions
                Ok(#enum_name::#state_entry(Some(state)))
            }
        });

        tokens.extend(token_steam);

    }
}

pub struct IsStateToTokens<'a> {
    machine: &'a Machine,
    state: &'a State,
}

impl<'a> IsStateToTokens<'a> {
    pub fn new(machine: &'a Machine, state: &'a State) -> Self {
        Self {
            machine,
            state
        }
    }
}

impl ToTokens for IsStateToTokens<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let state_entry = &self.state.enum_name;
        let state = &self.state;
        let enum_name = &self.machine.enum_name;
        let sfsm_name = &self.machine.name;
        let token_steam = proc_macro2::TokenStream::from(quote! {
            impl IsState<#state> for #sfsm_name {
                fn is_state(&self) -> bool {
                    return match self.states {
                        #enum_name::#state_entry(_) => {
                            true
                        }
                        _ => false
                    }
                }
            }

        });
        tokens.extend(token_steam);
    }
}

pub struct StateEntriesToTokens<'a> {
    state: &'a State,
}

impl<'a> StateEntriesToTokens<'a> {
    pub fn new(state: &'a State) -> Self {
        Self {
            state
        }
    }
}

impl ToTokens for StateEntriesToTokens<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let state_enum_name = &self.state.enum_name;
        let state = self.state;
        let token_steam = proc_macro2::TokenStream::from(quote! {
            #state_enum_name(Option<#state>),
        });

        tokens.extend(token_steam);
    }
}

pub struct StateToTokens<'a> {
    machine: &'a Machine,
    state: &'a State,
}

impl<'a> StateToTokens<'a> {
    pub fn new(machine: &'a Machine, state: &'a State) -> Self {
        Self {
            machine,
            state,
        }
    }
}

impl<'a> ToTokens for StateToTokens<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let enum_name = &self.machine.enum_name;
        let state_entry = &self.state.enum_name;
        let state_transits = &self.state.transits;
        let state = &self.state;
        let sfsm_error = &self.machine.sfsm_error;
        let custom_error = &self.machine.custom_error;
        let transition_checks: Vec<TransitionToTokens> = (&self.state.transits).into_iter().map(|trans| {
            TransitionToTokens::new(self.machine, self.state, trans)
        }).collect();

        let state_trait = &self.machine.trait_definitions.state_trait;
        let transit_trait = &self.machine.trait_definitions.transit_trait;
        let execute = &self.machine.trait_definitions.execute;

        let state_execute_tokens = TransitToErrorToTokens::wrap_if_fallible(&self.machine, proc_macro2::TokenStream::from(quote! {
                #state_trait::#execute(&mut state)
        }), &self.state);

        let transition_execute_tokens: Vec<TokenStream> = state_transits.into_iter().map(|transition| {
            TransitToErrorToTokens::wrap_if_fallible(&self.machine, proc_macro2::TokenStream::from(quote! {
                #transit_trait::<#transition>::#execute(&mut state)
            }), &self.state)
        }).collect();

        let token_steam = proc_macro2::TokenStream::from(quote! {
                #enum_name::#state_entry(ref mut state_option) => {
                    fn run_state(state_option: &mut Option<#state>) -> Result<#enum_name, #sfsm_error#custom_error> {
                        let mut state = state_option.take().ok_or(#sfsm_error::Internal)?;
                        #state_execute_tokens
                        #( #transition_execute_tokens )*
                        #( #transition_checks )*
                        {
                            return Ok(#enum_name::#state_entry(Some(state)));
                        }
                    }
                    run_state(state_option)?
                }
        });

        tokens.extend(token_steam);
    }
}

impl ToTokens for State {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let generics = &self.generics;
        let token_steam = proc_macro2::TokenStream::from(quote! {
            #name#generics
        });

        tokens.extend(token_steam);
    }
}

pub struct TransitionToTokens<'a> {
    machine: &'a Machine,
    state: &'a State,
    target: &'a State
}

impl<'a> TransitionToTokens<'a> {
    pub fn new(machine: &'a Machine, state: &'a State, target: &'a State) -> Self {
        Self {
            machine,
            state,
            target
        }
    }
}

impl ToTokens for TransitionToTokens<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {

        let target_state_entry = &self.target.enum_name;
        let enum_name = &self.machine.enum_name;
        let entry = &self.machine.trait_definitions.entry;
        let exit_transitions = ExitTransitionToTokens::new(&self.state.transits, self.machine, &self.state);

        let state_trait = &self.machine.trait_definitions.state_trait;
        let transit_trait = &self.machine.trait_definitions.transit_trait;
        let exit = &self.machine.trait_definitions.exit;

        let exit_token_stream = TransitToErrorToTokens::wrap_if_fallible(&self.machine, proc_macro2::TokenStream::from(quote! {
                #state_trait::#exit(&mut state)
        }), &self.state);

        let target_state = (&self.machine.states).into_iter().find(|state| {
            return state.enum_name == *target_state_entry;
        }).expect("Internal error. Expected to find a state matching the transition");

        let transition_entry_tokens: Vec<TokenStream> = (&target_state.transits).into_iter().map(|transition| {
            TransitToErrorToTokens::wrap_if_fallible(&self.machine, proc_macro2::TokenStream::from(quote! {
                #transit_trait::<#transition>::#entry(&mut state)
            }), &self.state)
        }).collect();

        let state_entry_tokens: TokenStream = TransitToErrorToTokens::wrap_if_fallible(&self.machine, proc_macro2::TokenStream::from(quote! {
                #state_trait::#entry(&mut state)
            }), &self.state);

        let token_steam = proc_macro2::TokenStream::from(quote! {
            if #transit_trait::<#target_state>::guard(&state) == TransitGuard::Transit {
                #exit_token_stream

                #exit_transitions

                let mut state: #target_state = state.into();

                #state_entry_tokens
                #(#transition_entry_tokens);*

                return Ok(#enum_name::#target_state_entry(Some(state)));
            } else
        });

        tokens.extend(token_steam);
    }
}

pub struct ExitTransitionToTokens<'a> {
    machine: &'a Machine,
    transits: &'a Vec<State>,
    state: &'a State,
}

impl<'a> ExitTransitionToTokens<'a> {
    pub fn new(transits: &'a Vec<State>, machine: &'a Machine, state: &'a State) -> Self {
        Self {
            transits,
            machine,
            state
        }
    }
}

impl ToTokens for ExitTransitionToTokens<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let transits = self.transits;
        let transit_trait = &self.machine.trait_definitions.transit_trait;
        let exit = &self.machine.trait_definitions.exit;

        let exit_token_streams: Vec<proc_macro2::TokenStream> = transits.into_iter().map(|transits| {
            TransitToErrorToTokens::wrap_if_fallible(&self.machine, proc_macro2::TokenStream::from(quote! {
                #transit_trait::<#transits>::#exit(&mut state)
            }), &self.state)
        }).collect();

        let token_steam = proc_macro2::TokenStream::from(quote! {
            #( #exit_token_streams )*
        });
        tokens.extend(token_steam);
    }
}


pub struct StateMessageToTokens<'a> {
    state_message: &'a StateMessage,
    messages: &'a Messages,
}

impl<'a> StateMessageToTokens<'a> {
    pub fn new(state_message: &'a StateMessage, messages: &'a Messages) -> Self {
        Self {
            state_message,
            messages,
        }
    }
}

impl ToTokens for StateMessageToTokens<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let message_dir = &self.state_message.message;
        let enum_entry = &self.state_message.state.enum_name;
        let state = &self.state_message.state;
        let enum_name = &self.messages.enum_name;
        let sfsm_name = &self.messages.name;

        let token_steam = match message_dir {
            MessageDir::Push(message) => {
                let message_name = &message.name;
                let message_args = &message.generics;
                proc_macro2::TokenStream::from(quote! {
                    impl PushMessage<#state, #message_name#message_args> for #sfsm_name {
                        fn push_message(&mut self, message: #message_name#message_args) -> Result<(), MessageError<#message_name#message_args>> {
                            match self.states {
                                #enum_name::#enum_entry(ref mut state_option) => {
                                    if let Some(ref mut state) = state_option {
                                        state.receive_message(message);
                                        return Ok(())
                                    }
                                }
                                _ => {
                                    // Do nothing, this will return and error at the end of the function
                                }
                            }
                            return Err(MessageError::StateIsNotActive(message));
                        }
                    }
                })
            }
            MessageDir::Poll(message) => {
                let message_name = &message.name;
                let message_args = &message.generics;
                proc_macro2::TokenStream::from(quote! {
                    impl PollMessage<#state, #message_name#message_args> for #sfsm_name {
                        fn poll_message(&mut self) -> Result<Option<#message_name#message_args>, MessageError<()>> {
                            match self.states {
                                #enum_name::#enum_entry(ref mut state_option) => {
                                    if let Some(ref mut state) = state_option {
                                        let message = state.return_message();
                                        return Ok(message)
                                    }
                                }
                                _ => {
                                    // Do nothing, this will return and error at the end of the function
                                }
                            }
                            return Err(MessageError::StateIsNotActive(()));
                        }
                    }
                })
            }
        };

        tokens.extend(token_steam);
    }
}

pub struct MessagesToTokens<'a> {
    messages: &'a Messages,
}

impl<'a> MessagesToTokens<'a> {
    pub fn new(messages: &'a Messages) -> Self {
        Self {
            messages,
        }
    }
}

impl ToTokens for MessagesToTokens<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let messages = &self.messages.messages;

        let messages_to_tokens: Vec<StateMessageToTokens> = messages.into_iter().map(|message| {
            StateMessageToTokens::new(message, self.messages)
        }).collect();

        let token_steam = proc_macro2::TokenStream::from(quote! {
            #(#messages_to_tokens)*
        });

        tokens.extend(token_steam);
    }
}