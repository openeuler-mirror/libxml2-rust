#!/bin/bash
rm -f CMakeCache.txt 
cmake -DSTEP="build"
make
cd rust_ffi
cargo clean
cd ..
cd rust
cargo clean
cargo build
cd ..
cmake -DSTEP="link"
make
ctest