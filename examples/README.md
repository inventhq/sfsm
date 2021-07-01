# Examples
The following folder contains a few examples to explain how to use the state machine generator. 

- Basic: An clean example without any generics or messages showing the very basic usage.
- Basic Extended: Same as the basic example, but shows how generics and attributes can be used.
- Rocket Liftoff: An bit more elaborate example of how an actual state machine could look like.
- Messages: An example that shows how messages can be passed to states or be polled from states.

# Run
Run the example with;
```bash
cargo expand --example basic
```
or run all the example tests with:
```bash
cargo test --examples
```

# Expand
If you want to see what exactly the macro generates, you let [cargo expand](https://github.com/dtolnay/cargo-expand) generate the state machines of the examples.
```bash
cargo expand --example simple
```
