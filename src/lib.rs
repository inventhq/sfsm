#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]
extern crate sfsm_proc;
extern crate sfsm_base;

pub use sfsm_proc::*;
pub use sfsm_base::*;
