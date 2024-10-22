#!/bin/bash

# Compile the Rust project in release mode
cargo build --release

# Get the name of the binary
BINARY_NAME=$(basename $(find target/release -maxdepth 1 -type f -executable))

# Generate the command to run the compiled binary
RUN_COMMAND="./target/release/$BINARY_NAME"

# Print the command to the console for easy copy-paste
echo "To run the compiled binary, use the following command:"
echo $RUN_COMMAND
