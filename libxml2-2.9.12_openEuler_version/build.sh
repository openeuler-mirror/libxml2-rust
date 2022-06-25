#!/bin/bash
rm -rf CMakeCache.txt
make clean
cmake -DSTEP="build"
make
cd rust
cargo clean
cargo build --release
cd ../
cmake -DSTEP="link"
make