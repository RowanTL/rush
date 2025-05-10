# rush

An implementation of PushGP in Rust.

This implementation works for basic functionality and testing.
Anything that could be demanding fails. I believe this is
due to how I coded it, barely using any references and
cloning everything. Use if you please.

This is truly following the test-driven development model
so each of the files should have its own tests module.

# Building

`cargo build` and it'll build.

### Adding new instructions

When adding new instructions, a python script generates the function lists and places them
in `src/instructions/list.rs`. When creating new instructions, it is important to run
`scripts/instruction_list.py` to add newly created instructions to their corresponding function
lists. This script uses python 3.13. Must be ran from the root of the directory as
`python scripts/instruction_list.py`.

You must be able to run `cargo expand` for this script to function. Nix users should be okay.
If you're not on nix, run `cargo install cargo-expand`.

## Link for later

https://miroslavtushev.medium.com/does-my-sample-have-to-be-normally-distributed-for-a-t-test-7ee91aaaca2a
