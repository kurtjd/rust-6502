# Rust 6502 Emulator
A 6502 emulator written in Rust which passes all of Tom Harte's extensive tests (see below)*

\*Except some of the illegal/undefined opcodes do not currently make the correct memory accesses so cycle accuracy for these opcodes are not tested. They are functionally correct, however, according to the tests.

## Testing
This makes use of [Tom Harte's 6502 processor tests](https://github.com/TomHarte/ProcessorTests) for automatic testing. Essentially, these are randomly generated tests for each opcode in JSON format which defines the initial state and expected final state. To acquire these tests, run `clone_tests.sh` then simply call `cargo test` from the root of this repository to actually perform automated testing.

You may also test instructions and opcodes individually:  
`./test_instr <instruction-name>`  
`./test_opcode <opcode-in-hex>`

## License
This project is licensed under the MIT license and is completely free to use and modify.
