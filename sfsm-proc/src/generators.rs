use quote::{quote, ToTokens};
use crate::types::{Machine, State};

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

        let token_steam = proc_macro2::TokenStream::from(quote! {

            use sfsm::IsState;

            enum #enum_name {
                #(#state_entries)*
            }

            struct #sfsm_name {
                states: #enum_name,
                do_entry: bool,
            }

            impl #sfsm_name {
                pub fn new(data: #init_state) -> Self {
                    Self {
                        states: #enum_name::#init_state_entry(
                            Some(data)
                        ),
                        do_entry: true
                    }
                }

                pub fn step(&mut self) {
                    use #enum_name::*;
                    let ref mut e = self.states;
                    *e = match *e {
                        #( #states, )*
                    }
                }

                pub fn peek_state(&self) -> &#enum_name {
                   return &self.states;
                }

                pub fn stop(mut self) -> #enum_name {
                    match self.states {
                        # ( #exits )*,
                    }
                }
            }

            // Implement the is_state checks
            #(#is_states)*
        });

        tokens.extend(token_steam);
    }
}

struct StopToTokens<'a> {
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
        let exit_transitions = ExitTransitionToTokens::new(&self.state.transits);
        let token_steam = proc_macro2::TokenStream::from(quote! {
            #enum_name::#state_entry(ref mut state_option) => {
                let mut state = state_option.take().unwrap();
                State::exit(&mut state);
                #exit_transitions
                #enum_name::#state_entry(Some(state))
            }
        });

        tokens.extend(token_steam);

    }
}

struct IsStateToTokens<'a> {
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

struct StateEntriesToTokens<'a> {
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

struct StateToTokens<'a> {
    machine: &'a Machine,
    state: &'a State,
}

impl<'a> StateToTokens<'a> {
    pub fn new(machine: &'a Machine, state: &'a State) -> Self {
        Self {
            machine,
            state
        }
    }
}

impl<'a> ToTokens for StateToTokens<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let enum_name = &self.machine.enum_name;
        let state_entry = &self.state.enum_name;
        let state_transits = &self.state.transits;
        let transition_checks: Vec<TransitionToTokens> = (&self.state.transits).into_iter().map(|trans| {
            TransitionToTokens::new(self.machine, self.state, trans)
        }).collect();

        let token_steam = proc_macro2::TokenStream::from(quote! {
                #enum_name::#state_entry(ref mut state_option) => {
                    let mut state = state_option.take().unwrap();

                    if self.do_entry {
                        State::entry(&mut state);
                        #( Transition::<#state_transits>::entry(&mut state); )*
                        self.do_entry = false;
                    }

                    State::execute(&mut state);
                    #( Transition::<#state_transits>::execute(&mut state); )*
                    #( #transition_checks )*
                    {
                        #enum_name::#state_entry(Some(state))
                    }
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

struct TransitionToTokens<'a> {
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

        let target_state = self.target;
        let target_state_entry = &self.target.enum_name;
        let enum_name = &self.machine.enum_name;
        let exit_transitions = ExitTransitionToTokens::new(&self.state.transits);
        let token_steam = proc_macro2::TokenStream::from(quote! {
            if Transition::<#target_state>::guard(&state) == TransitGuard::Transit {

                State::exit(&mut state);

                #exit_transitions

                let mut next_state: #target_state = state.into();

                self.do_entry = true;
                #enum_name::#target_state_entry(Some(next_state))
            } else
        });

        tokens.extend(token_steam);
    }
}

struct ExitTransitionToTokens<'a> {
    transits: &'a Vec<State>,
}

impl<'a> ExitTransitionToTokens<'a> {
    pub fn new(transits: &'a Vec<State>) -> Self {
        Self {
            transits,
        }
    }
}

impl ToTokens for ExitTransitionToTokens<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let transits = self.transits;
        let token_steam = proc_macro2::TokenStream::from(quote! {
            #( Transition::<#transits>::exit(&mut state); )*
        });
        tokens.extend(token_steam);
    }
}
