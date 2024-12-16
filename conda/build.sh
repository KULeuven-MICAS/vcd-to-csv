#!/bin/bash

cargo build --release
mkdir -p ${PREFIX}/bin
cp ${SRC_DIR}/target/release/vcd-to-csv ${PREFIX}/bin
