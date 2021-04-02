#![cfg_attr(not(test), no_std)]

pub extern crate sfsm_proc;
pub extern crate sfsm_base;

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
