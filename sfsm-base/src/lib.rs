#![cfg_attr(not(test), no_std)]

/// Trait that must be implemented by all states
///
/// Allows to define behavior when entering, exiting and running the state. Both the entry and exit
/// function will only be executed once for each state. The execute function will be executed as
/// long as the state does not transition into another state. There can only ever be one single
/// state active.
pub trait State {

    /// Implement any behavior that hast to be executed when entering the state.
    fn entry(&mut self) {}

    /// Implement any behavior that has to be executed when the state is being executed.
    /// This function will be called as long as the state does not transit.
    fn execute(&mut self) {}

    /// Implement any behavior that hast to be executed when exiting the state.
    fn exit(&mut self) {}
}

/// Trait that must be implemented by a state that want to transition to DestinationState.
///
/// All states can have none or many transitions.
/// Both the entry and exit function will only be executed once for each state. The execute
/// function will be executed as long as the state does not transition into another state.
/// On top of the transition trait the state must implement the Into<DestinationState> trait
/// to specify what happens with the source state data while transitioning and how the destination
/// state is generated.
/// The only non optional function is the guard function that specifies when the state transitions.
/// Note: All transition behavior is always executed after the state trait behavior.
pub trait Transition<DestinationState>: Into<DestinationState> {
    /// Implement any behavior that hast to be executed when entering the state.
    fn entry(&mut self) {}

    /// Implement any behavior that has to be executed when the state is being executed.
    /// This function will be called as long as the state does not transit.
    fn execute(&mut self) {}

    /// Implement any behavior that hast to be executed when exiting the state.
    fn exit(&mut self) {}

    /// Specifies when the state has to transit. As long as the guard returns false, the state
    /// stays in the current state. When true is returned, the state machine will transit to
    /// DestinationState
    fn guard(&self) -> bool;
}

// Test the concept
#[cfg(test)]
mod tests {
    use crate::{State, Transition};
    use std::rc::Rc;
    use std::cell::RefCell;

    // Definitions of data structs and transitions required
    #[derive(Debug, PartialEq)]
    enum StateMonitor {
        PreInit,
        Init,
        Process,
    }
    struct GlobalData { pub val: u32, pub monitor: Rc<RefCell<StateMonitor>> }
    struct InitData { pub val: u32, pub global: GlobalData }
    struct ProcessData { pub global: GlobalData  }

    // Init state definitions
    impl State for InitData {
        fn entry(&mut self) {
            self.val = 1;
            self.global.val = 0;
            let mut monitor = self.global.monitor.borrow_mut();
            *monitor = StateMonitor::Init;
        }
    }

    impl Transition<ProcessData> for InitData {
        // Transit immediately
        fn guard(&self) -> bool {
            true
        }
    }

    impl Into<ProcessData> for InitData {
        fn into(self) -> ProcessData {
            ProcessData {
                global: self.global,
            }
        }
    }


    // Process state definitions
    impl State for ProcessData {
        fn entry(&mut self) {
            let mut monitor = self.global.monitor.borrow_mut();
            *monitor = StateMonitor::Process;
        }

        fn execute(&mut self) {
            self.global.val = self.global.val + 1;
        }
    }

    impl Into<InitData> for ProcessData {
        fn into(self) -> InitData {
            InitData {
                val: 0,
                global: self.global
            }
        }
    }

    impl Transition<ProcessData> for ProcessData {
        // Transit immediately
        fn guard(&self) -> bool {
            return self.global.val == 1;
        }
    }

    impl Transition<InitData> for ProcessData {
        fn guard(&self) -> bool {
            return self.global.val == 2;
        }
    }

    // One enum entry for every state will have to be generated
    enum SfsmStates {
        InitStateEntry(Option<InitData>),
        ProcessStateEntry(Option<ProcessData>),
    }

    struct StaticFiniteStateMachine {
        states: SfsmStates,
        do_entry: bool,
    }

    impl StaticFiniteStateMachine {
        // Will this have to be fully generated?
        pub fn new(data: InitData) -> Self {
            Self {
                states: SfsmStates::InitStateEntry(
                    Some(data)
                ),
                do_entry: true
            }
        }

        pub fn step(&mut self) {
            let ref mut e = self.states;
            *e = match *e {
                SfsmStates::InitStateEntry(ref mut state_option) => {

                    let mut state = state_option.take().unwrap();

                    if self.do_entry {
                        State::entry(&mut state);
                        Transition::entry(&mut state);
                        self.do_entry = false;
                    }

                    State::execute(&mut state);
                    Transition::<ProcessData>::execute(&mut state);

                    if Transition::<ProcessData>::guard(&state) {

                        State::exit(&mut state);
                        Transition::<ProcessData>::exit(&mut state);

                        let next_state_data: ProcessData = state.into();

                        self.do_entry = true;
                        SfsmStates::ProcessStateEntry(Some(next_state_data))
                    } else {
                        SfsmStates::InitStateEntry(Some(state))
                    }
                }
                SfsmStates::ProcessStateEntry(ref mut state_option) => {

                    let mut state = state_option.take().unwrap();

                    if self.do_entry {
                        State::entry(&mut state);
                        Transition::<InitData>::execute(&mut state);
                        Transition::<ProcessData>::execute(&mut state);
                        self.do_entry = false;
                    }

                    State::execute(&mut state);
                    Transition::<InitData>::execute(&mut state);
                    Transition::<ProcessData>::execute(&mut state);

                    if Transition::<InitData>::guard(&state) {

                        State::exit(&mut state);
                        Transition::<InitData>::exit(&mut state);
                        Transition::<ProcessData>::exit(&mut state);

                        let next_state_data: InitData = state.into();

                        self.do_entry = true;
                        SfsmStates::InitStateEntry(Some(next_state_data))
                    } else if Transition::<ProcessData>::guard(&state) {

                        State::exit(&mut state);
                        Transition::<InitData>::exit(&mut state);
                        Transition::<ProcessData>::exit(&mut state);

                        let next_state_data: ProcessData = state.into();

                        self.do_entry = true;
                        SfsmStates::ProcessStateEntry(Some(next_state_data))
                    }
                    else {
                        SfsmStates::ProcessStateEntry(Some(state))
                    }
                }
            }
        }

        pub fn peek_state(&self) -> &SfsmStates {
            return &self.states;
        }

        pub fn stop(mut self) -> SfsmStates {
            match self.states {
                SfsmStates::InitStateEntry(ref mut state_option) => {
                    let mut state = state_option.take().unwrap();
                    State::exit(&mut state);
                    Transition::<ProcessData>::exit(&mut state);

                    SfsmStates::InitStateEntry(Some(state))
                }
                SfsmStates::ProcessStateEntry(ref mut state_option) => {
                    let mut state = state_option.take().unwrap();
                    State::exit(&mut state);
                    Transition::<InitData>::exit(&mut state);
                    Transition::<ProcessData>::exit(&mut state);
                    SfsmStates::ProcessStateEntry(Some(state))
                }
            }
        }
    }


    #[test]
    fn concept() {

        let monitor = Rc::new(RefCell::new(StateMonitor::PreInit));

        let global = GlobalData {
            val: 0,
            monitor: monitor.clone(),
        };

        let init = InitData {
            val: 0,
            global
        };

        let mut sfms = StaticFiniteStateMachine::new(init);
        sfms.step();

        assert_eq!(*monitor.borrow(), StateMonitor::Init);

        sfms.step();
        assert_eq!(*monitor.borrow(), StateMonitor::Process);

        let state = sfms.peek_state();

        match state {
            SfsmStates::ProcessStateEntry(_in_state) => {
                assert!(true);
            }
            _ => {
                assert!(false);
            }
        }

        sfms.step();
        assert_eq!(*monitor.borrow(), StateMonitor::Process);

        sfms.step();
        assert_eq!(*monitor.borrow(), StateMonitor::Init);

        let exit = sfms.stop();

        match exit {
            SfsmStates::ProcessStateEntry(_) => {
                assert!(true);
            }
            _ => {
                assert!(false);
            }
        }
    }
}
