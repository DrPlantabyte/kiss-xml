#!/bin/bash
clear
echo "cargo test -- --nocapture --test-threads=1"
cargo       test -- --nocapture --test-threads=1
