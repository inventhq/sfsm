#![cfg_attr(not(test), no_std)]

//! # Static finite state machine
//!
//! Sfsm tries to achieve these objectives, by providing a state machine generator in sfsm-proc and
//! a transition as well as state trait in sfsm-proc. With this, the user can specify the whole state
//! machine on a few lines that are easy to review. From this definition, the whole state machine
//! can be generated without relying on dynamic mechanisms and thus allows to be fully static.
//! All that is left to do, is to implement the states and transition necessary to fulfill the
//! Transition and State traits.
//!
//! State machines are an essential part of many software architectures and are particularly common on low
//! level systems such as embedded systems. They allow a complicated system to be broken down into many
//! small states with clearly defined transitions between each other. But while they help to break down
//! complexity, they must also be well documented to be understandable.
//!
//! Rust is well suited to implementing state machines thanks the way its enums are designed.
//! Unfortunately this still comes with a large amount of boilerplate.
//!
//! Sfsm aims to let the user implement simple, efficient and easy to review state machines that are usable
//! on embedded systems. The main objectives therefore are:
//!
//! The main objectives therefore are:
//! - no_std compatibility
//! - Self documenting
//! - Easy to use
//! - Low cost
//!
//! Sfsm tries to achieve these objectives by providing a state machine generator in sfsm-proc and a
//! transition as well as state trait in sfsm-proc. With this, the user can specify the whole state machine on
//! a few lines that are easy to review. From this definition, the whole state machine can be generated
//! without relying on dynamic mechanisms and thus allows to be fully static. All that is left to do is to
//! implement the states and transition necessary to fulfill the Transition and State traits.
//!
//!
//! # How to use
//! To see the whole example, expand the source
//!```rust
//! extern crate sfsm_proc;
//! extern crate sfsm_base;
//! use sfsm_proc::{add_state_machine, match_state_entry};
//! use sfsm_base::{State, Transition, TransitGuard};
//! use std::marker::PhantomData;
//!
//! // To start out, first define the state machine.
//! add_state_machine!(
//!    Hiker,  // Name of the state machine. Used to run it later
//!    Hike<Up>,   // The initial state the state machine will start with
//!    [
//!         // Define all states. These states must correspond to a struct
//!         Hike<Up>,
//!         Hike<Down>,
//!         Picknic
//!    ],
//!    [
//!         // Define all transitions with: Src -> Dst
//!         Hike<Up> -> Picknic,
//!         Picknic -> Hike<Down>
//!    ]
//! );
//!
//! // Add the structs that correspond to the defined states.
//! struct Up {};
//! struct Down {};
//!
//! struct Hike<Dir> {
//!     marker: PhantomData<Dir>,
//!     is_down: bool,
//! }
//!
//! struct Picknic {
//!    apples: u32,
//! }
//!
//! // Implement the states traits
//! # impl State for Hike<Up> {
//! #     fn entry(&mut self) {
//! #         println!("****************************************");
//! #         println!("Hike<Up>: Start hiking up");
//! #         self.is_down = false;
//! #     }
//! #     fn execute(&mut self) {
//! #         println!("Hike<Up>: Keep walking");
//! #     }
//! #     fn exit(&mut self) {
//! #         println!("Hike<Up>: Take a break");
//! #     }
//! # }
//! impl State for Picknic {
//!     fn entry(&mut self) {
//!         println!("****************************************");
//!         println!("Picknic: Start eating a picknick");
//!     }
//!     fn execute(&mut self) {
//!         self.apples -= 1;
//!         println!("Picknic: Eat an apple");
//!     }
//!     fn exit(&mut self) {
//!         println!("Picknic: Get up");
//!     }
//! }
//!
//! # impl State for Hike<Down> {
//! #     fn entry(&mut self) {
//! #         println!("****************************************");
//! #         println!("Hike<Down>: Start walking back down");
//! #     }
//! #     fn execute(&mut self) {
//! #         println!("Hike<Down>: Keep walking");
//! #     }
//! #     fn exit(&mut self) {
//! #         println!("Hike<Down>: Go back home");
//! #         println!("****************************************");
//! #         self.is_down = true;
//! #     }
//! # }
//!
//! // Note: The state implementation for Hike<Up> and Hike<Down> is hidden
//!
//! // ...
//!
//! // Then implement the transitions
//! # impl Transition<Picknic> for Hike<Up> {
//! #     fn guard(&self) -> TransitGuard {
//! #          return TransitGuard::Transit;
//! #     }
//! # }
//! impl Transition<Hike<Down>> for Picknic {
//!    fn entry(&mut self) {
//!     // You might want to do something here. Like starting to count apples
//!    }
//!    fn execute(&mut self) {
//!     // You could keep counting apples
//!    }
//!    fn exit(&mut self) {
//!     // You might want to sum up what you counted before
//!    }
//!     fn guard(&self) -> TransitGuard {
//!         if self.apples == 0 {
//!             TransitGuard::Transit
//!         } else {
//!             TransitGuard::Remain
//!         }
//!     }
//! }
//!
//! # impl Into<Picknic> for Hike<Up> {
//! #     fn into(self) -> Picknic {
//! #         Picknic {
//! #             apples: 3,
//! #         }
//! #     }
//! # }
//! impl Into<Hike<Down>> for Picknic {
//!     fn into(self) -> Hike<Down> {
//!         Hike {
//!             marker: PhantomData,
//!             is_down: false,
//!         }
//!     }
//! }
//!
//! // Note: The transition Hike<Up> -> Picknic is hidden
//!
//! // ...
//!
//! // And then run the state machine.
//! let init: Hike<Up> = Hike {
//!    marker: PhantomData,
//!    is_down: true,
//! };
//!
//! // Create the state machine with the name defined and pass the initial state into it.
//! let mut sfsm = Hiker::new(init);
//!
//! // If you want to check which state the machine currently is in, you can peek it.
//! // Note that the generated enum will be named: [CHOOSEN_NAME_OF_SFSM]States and the entries
//! // will be called [NAME_OF_STRUCT_WITH_TYPES]State
//! let in_state = sfsm.peek_state();
//!
//! // The IsState trait implementations helps you to quickly test if its the state you expect.
//! assert!(IsState::<Hike<Up>>::is_state(&sfsm));
//!
//! // Start stepping!
//! sfsm.step().unwrap();
//! assert!(IsState::<Picknic>::is_state(&sfsm));
//!
//! sfsm.step().unwrap();
//! assert!(IsState::<Picknic>::is_state(&sfsm));
//!
//! sfsm.step().unwrap();
//! assert!(IsState::<Picknic>::is_state(&sfsm));
//!
//! sfsm.step().unwrap();
//! assert!(IsState::<Hike<Down>>::is_state(&sfsm));
//!
//! sfsm.step().unwrap();
//!
//! // Once you are done using the state machine, you can stop it and return the current state.
//! let exit = sfsm.stop().unwrap();
//!
//! match exit {
//!     // If you don't want to type out the state enum use the match_state_entry! macro here
//!     // It generates the following: [SFSM_NAME]States::[STATE_NAME_AND_TYPES]State(state)
//!     // Otherwise you have to type it out manually with the given schema.
//!     match_state_entry!(Hiker, Hike<Down>, exit_state) => {
//!         // Access "exit_state" here
//!         assert!(exit_state.unwrap().is_down);
//!     }
//!     _ => {
//!         assert!(false);
//!     }
//! }
//!
//!```
//! This will then produce the following output:
//!```text
//! ****************************************
//! Hike<Up>: Start hiking up
//! Hike<Up>: Keep walking
//! Hike<Up>: Take a break
//! ****************************************
//! Picknic: Start eating a picknick
//! Picknic: Eat an apple
//! Picknic: Eat an apple
//! Picknic: Eat an apple
//! Picknic: Get up
//! ****************************************
//! Hike<Down>: Start walking back down
//! Hike<Down>: Keep walking
//! Hike<Down>: Go back home
//! ****************************************
//!```
//! For more detailed descriptions about the traits, look at the sfsm-base doc.

extern crate sfsm_proc;
extern crate sfsm_base;

pub use sfsm_proc::*;
pub use sfsm_base::*;
