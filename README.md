# Rust 6502 Emulator
This is a 6502 emulator I'm writing to get more familiar with Rust, which I also plan to use in future emulators such as Apple ][ and Commodore 64.

## Testing
This makes use of [Tom Harte's 6502 processor tests](https://github.com/TomHarte/ProcessorTests) for automatic testing. Essentially, these are randomly generated tests for each opcode in JSON format which defines the initial state and expected final state. To acquire these tests, run `clone_tests.sh` then simply call `cargo test` from the root of this repository to actually perform automated testing.

## License
This project is licensed under the MIT license and is completely free to use and modify.
