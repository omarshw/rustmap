#!/bin/bash

# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add Rust to the PATH
source $HOME/.cargo/env

# Installation Verification
rustc--version
cargo --version
