#!/bin/bash

cargo build --release
mkdir -p $PREFIX/bin
cp target/release/vcd-to-csv $PREFIX/bin
