# Static state machine generator for no_std environments

Sfsm tries to achieve these objectives, by providing a state machine generator in sfsm-proc and
a transition as well as state trait in sfsm-proc. With this, the user can specify the whole state
machine on a few lines that are easy to review. From this definition, the whole state machine
can be generated without relying on dynamic mechanisms and thus allows to be fully static.
All that is left to do, is to implement the states and transition necessary to fulfill the
Transition and State traits.

State machines are an essential part of many software architectures and are particularly common on low
level systems such as embedded systems. They allow a complicated system to be broken down into many
small states with clearly defined transitions between each other. But while they help to break down
complexity, they must also be well documented to be understandable.

Rust is well suited to implementing state machines thanks the way its enums are designed.
Unfortunately this still comes with a large amount of boilerplate.

Sfsm aims to let the user implement simple, efficient and easy to review state machines that are usable
on embedded systems. The main objectives therefore are:

The main objectives therefore are:
- no_std compatibility
- Self documenting
- Easy to use
- Low cost

Sfsm tries to achieve these objectives by providing a state machine generator in sfsm-proc and a
transition as well as state trait in sfsm-proc. With this, the user can specify the whole state machine on
a few lines that are easy to review. From this definition, the whole state machine can be generated
without relying on dynamic mechanisms and thus allows to be fully static. All that is left to do is to
implement the states and transition necessary to fulfill the Transition and State traits.

# Example usage
A state machine can be defined with the following macro call.
```
 add_state_machine!(
     Rocket,                                   // Name of the state machine. Accepts a visibility modifier.
     WaitForLaunch,                                 // The initial state the state machine will start in
     [WaitForLaunch, Ascent],                       // All possible states
     [
         WaitForLaunch => Ascent,                   // All transitions
     ]
 );
```
This will generate a state machine called ``` Rocket ``` with an initial state in ``` WaitForLaunch ```.
There are two possible states the state machine will be in - ``` WaitForLaunch ``` and ``` Ascent ```.
``` WaitForLaunch ``` is the initial state and can transit to ``` Ascent ``` due to the ``` WaitForLaunch => Ascent ``` transition
definition. A state machine can have as many states and transitions as desired but all of they must implement the ``` State ```
and the according ``` Transition ``` traits.

# Error handling state
With the ``` add_fallible_state_machine ``` macro, a state machine can be defined that adds intrinsic error handling
to the normal state machines. It allows to specify an error state and type. As soon as an error occurs, the state machine
immediately jumps into the error state where the error can be handled. 
A fallible state machine can be defined with the following macro call:
```
 add_fallible_state_machine!(
    Rocket,                                 // Name of the state machine. Accepts a visibility modifier.
    WaitForLaunch,                               // The initial state the state machine will start in
    [WaitForLaunch, Ascent, HandleMalfunction],  // All possible states
    [
        WaitForLaunch => Ascent,                 // All possible Transitions
        HandleMalfunction => WaitForLaunch
    ],
    RocketMalfunction,                      // The error type
    HandleMalfunction                       // The error state
 );
```
Similar to the normal state machine, this will generate a state machine for which the user has to implement the behavior
of the states and transitions. In the fallible state machine, the traits that have to be implemented are 
``` TryState ``` and ``` TryTransition ``` traits. Additionally, the error state must implement the
``` TryErrorState ``` trait to implement how the error is handled.

# Messaging system
Additionally, messages to be passed into, or polled from the states can be defined.
```
 add_messages!(
     Rocket,
     [
         StartLaunch -> WaitForLaunch,               // Command the WaitForLaunch state to liftoff
         Status <- Launch,                           // Poll the status of the launch state
     ]
 );
```
This creates the code to pass ``` StartLaunch ``` into the ``` WaitForLaunch ``` state and allows to poll ``` Status ``` from the ``` Launch ```
state. Each state can have multiple receive and return messages, but it must implement the according ``` ReturnMessage ``` and ``` ReceiveMessage ``` traits.
For more information, take a look at the [examples](https://gitlab.com/sfsm/sfsm/-/tree/develop/examples) or at the [doc](https://docs.rs/sfsm).