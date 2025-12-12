#!/usr/bin/env bash

cargo init     # Initialize a cargo (rust) project
cargo add ggez # Add the GGEZ framework as a dependency for the project

# Get rid of the useless files
rm -f README.md init.sh
