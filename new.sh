#!/bin/bash

# Pathname of the directory for the AtCoder contest.
readonly CONTEST_DIR_PREFIX="$HOME/atcoder"

if [ $# -ne 1 ]; then
  echo "Usage: ./new <CONTEST ID>"
  exit 1
fi

cd $CONTEST_DIR_PREFIX

# Get contest name from command line argument.
readonly CONTEST_NAME=$1

# Add a package for the contest to the workspace.
sed -i "s/^\]$/  \"$CONTEST_NAME\",\n\]/" "$CONTEST_DIR_PREFIX/Cargo.toml"

# Create a package for the contest.
cargo compete new $CONTEST_NAME --open

# # Open the directory for the contest in Visual Studio Code.
# code $CONTEST_NAME
