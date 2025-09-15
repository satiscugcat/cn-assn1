#!/bin/bash
cd server || exit
cargo run &
cd ../client || exit
cargo run -- "$@" 
