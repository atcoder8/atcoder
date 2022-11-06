#!/bin/bash

if [ $# != 1 ]; then
  echo "Usage: ./new <CONTEST NAME>"
  exit 1
fi

cargo compete new $1 --open
code $1
