#![cfg_attr(not(test), no_std)]

pub extern crate sfsm_proc;
pub extern crate sfsm_base;

/// # Static finite state machine
///
/// State machines are a existential part of many software architectures and are particularly
/// common on low level systems such as embedded systems. They allow a complicated system to be
/// broken down into many small states with clearly defined transitions between each other.
/// But while help to break down complexity, they must also be well documented to be understandable.
///
/// Rust lends itself to implement state machines fairly well thanks the way its enums are designed.
/// Unfortunately this still comes with a large amount of boilerplate.
///
/// Sfsm aims to let the user implement simple, efficient and easy to review state machines that are
/// usable on embedded systems.
/// The main objectives therefore are:
/// - no_std compatibility
/// - Self documenting
/// - Easy to use
/// - Efficient
///
/// Sfsm tries to achieve these objectives, by providing a state machine generator in sfsm-proc and
/// a transition as well as state trait in sfsm-proc. With this, the user can specify the whole state
/// machine on a few lines that are easy to review. From this definition, the whole state machine
/// can be generated without relying on dynamic mechanisms and thus allows to be fully static.
/// All that is left to do, is to implement the states and transition necessary to fulfill the
/// Transition and State traits.
///
///
///
///
///















































#[cfg(test)]
mod tests {

    use crate::sfsm_proc::add_state_machine;
    use crate::sfsm_base::{Transition, State};
    use std::rc::Rc;
    use std::cell::RefCell;

    struct InitState {
        state_message: Rc<RefCell<String>>
    }

    struct WaitingState {
        counter: u32,
        state_message: Rc<RefCell<String>>
    }

    struct EndState {
        state_message: Rc<RefCell<String>>
    }

    impl State for InitState {
        fn entry(&mut self) {
            println!("Init: Enter");
        }

        fn execute(&mut self) {
            let mut msg = self.state_message.borrow_mut();
            *msg = "InitState".to_string();
            println!("Init: Execute");
        }

        fn exit(&mut self) {
            println!("Init: Exit");
        }
    }

    impl State for WaitingState {
        fn entry(&mut self) {
            println!("Waiting: Enter");
        }

        fn execute(&mut self) {
            self.counter += 1;
            let mut msg = self.state_message.borrow_mut();
            *msg = "WaitingState".to_string();
            println!("Waiting: Execute");
        }

        fn exit(&mut self) {
            println!("Waiting: Exit");
        }
    }

    impl State for EndState {
        fn entry(&mut self) {
            println!("End: Enter");
        }

        fn execute(&mut self) {
            let mut msg = self.state_message.borrow_mut();
            *msg = "EndState".to_string();
            println!("End: Execute");
        }

        fn exit(&mut self) {
            println!("End: Exit");
        }
    }

    impl Transition<WaitingState> for InitState {
        fn entry(&mut self) {
            println!("Init -> Waiting: Enter");
        }

        fn execute(&mut self) {
            println!("Init -> Waiting: Execute");
        }

        fn exit(&mut self) {
            println!("Init -> Waiting: Exit");
        }

        fn guard(&self) -> bool {
            return true;
        }
    }

    impl Transition<EndState> for WaitingState {
        fn guard(&self) -> bool {
            return self.counter == 2;
        }
    }

    impl Into<WaitingState> for InitState {
        fn into(self) -> WaitingState {
            WaitingState {
                state_message: self.state_message,
                counter: 0,
            }
        }
    }

    impl Into<EndState> for WaitingState {
        fn into(self) -> EndState {
            EndState {
                state_message: self.state_message,
            }
        }
    }


    add_state_machine!(
        StaticSfms,
        InitState,
        [InitState, EndState, WaitingState],
        [
            InitState -> WaitingState,
            WaitingState -> EndState
        ]
    );

    #[test]
    fn generation() {

        let state_message = Rc::new(RefCell::new("".to_string()));

        let init = InitState {
            state_message: state_message.clone(),
        };
        let mut sfsm = StaticSfms::new(init);

        sfsm.step();
        let msg = state_message.borrow().clone();
        assert_eq!(msg, "InitState".to_string());

        sfsm.step();
        let msg = state_message.borrow().clone();
        assert_eq!(msg, "WaitingState".to_string());

        sfsm.step();
        let msg = state_message.borrow().clone();
        assert_eq!(msg, "WaitingState".to_string());

        sfsm.step();
        let msg = state_message.borrow().clone();
        assert_eq!(msg, "EndState".to_string());
    }
}
